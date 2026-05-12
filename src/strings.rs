#[derive(Debug, PartialEq, Eq)]
pub enum UTF8Error {
  UnexpectedEOF,
  ExpectedContinuation(u8),
  InvalidByte(u8),
  InvalidChar(u32),
  OverlongEncoding(u32),
  CharacterOutOfRange(u32),
}

/// Validates and decodes utf-8 encoded bytes.
pub fn decode_utf8(bytes: &[u8]) -> Result<Vec<char>, UTF8Error> {
  #[inline]
  fn get_continuation<'a, A: Iterator<Item = &'a u8>>(iter: &mut A) -> Result<u32, UTF8Error> {
    match *iter.next().ok_or(UTF8Error::UnexpectedEOF)? {
      // 0b10xxxxxx
      b @ 0x80..0xC0 => Ok((b & 0b00111111) as u32),
      err => Err(UTF8Error::ExpectedContinuation(err)),
    }
  }

  let mut res: Vec<char> = Vec::default();
  let mut iter = bytes.iter();
  while let Some(byte) = iter.next() {
    let (ch, minimum_ch) = match *byte {
      a @ 0..0x80 => (a as u32, 0),
      // 128..192 are continuation bytes
      a @ 0xC0..0xE0 => {
        // 0b110xxxxx
        let a = (a & 0b00011111) as u32;
        let b = get_continuation(&mut iter)?;
        (a << 6 | b, 0x80)
      }
      a @ 0xE0..0xF0 => {
        // 0b1110xxxx
        let a = (a & 0b00001111) as u32;
        let b = get_continuation(&mut iter)?;
        let c = get_continuation(&mut iter)?;
        (a << 12 | b << 6 | c, 0x800)
      }
      a @ 0xF0..0xF8 => {
        // 0b11110xxx
        let a = (a & 0b00000111) as u32;
        let b = get_continuation(&mut iter)?;
        let c = get_continuation(&mut iter)?;
        let d = get_continuation(&mut iter)?;
        (a << 18 | b << 12 | c << 6 | d, 0x10000)
      }
      err => return Err(UTF8Error::InvalidByte(err)),
    };
    if ch < minimum_ch {
      // the decoded char is less than the minimum for that length of encoding
      return Err(UTF8Error::OverlongEncoding(ch));
    }
    res.push(char::from_u32(ch).ok_or(UTF8Error::InvalidChar(ch))?)
  }

  Ok(res)
}

pub fn encode_utf8(text: impl IntoIterator<Item = char>) -> Vec<u8> {
  let mut res = vec![];

  text
    .into_iter()
    .try_for_each(|ch| encode_char(ch, &mut res).map(|_| ()))
    .unwrap();

  res
}

fn encode_char(ch: char, dst: &mut impl Extend<u8>) -> Result<usize, UTF8Error> {
  let ch = ch as u32;
  let bytes = match ch {
    ..0x80 => {
      dst.extend([ch as u8]);
      1
    }
    0x80..0x800 => {
      let a = 0b11000000 | ((ch >> 6) as u8 & 0b00011111);
      let b = 0b10000000 | (ch as u8 & 0b00111111);
      dst.extend([a, b]);
      2
    }
    0x800..0x10000 => {
      let a = 0b11100000 | ((ch >> 12) as u8 & 0b00001111);
      let b = 0b10000000 | ((ch >> 6) as u8 & 0b00111111);
      let c = 0b10000000 | (ch as u8 & 0b00111111);
      dst.extend([a, b, c]);
      3
    }
    0x10000..=0x1ffff => {
      let a = 0b11110000 | ((ch >> 18) as u8 & 0b00000111);
      let b = 0b10000000 | ((ch >> 12) as u8 & 0b00111111);
      let c = 0b10000000 | ((ch >> 6) as u8 & 0b00111111);
      let d = 0b10000000 | (ch as u8 & 0b00111111);
      dst.extend([a, b, c, d]);
      4
    }
    _ => return Err(UTF8Error::CharacterOutOfRange(ch)),
  };
  Ok(bytes)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decode_latin() {
    let text = "salve mundus";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn decode_swedish() {
    let text = "hallå världen";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn decode_chinese() {
    let text = "你好世界";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn decode_emoji() {
    let text = "👋🌍";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn decode_invalid() {
    let bytes = &[167, 32, 97]; // starts with a continuation byte
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::InvalidByte(167)));
    let bytes = &[97, 32, 229, 167]; // not enough continuation bytes
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::UnexpectedEOF));
    let bytes = &[97, 32, 229, 167, 97]; // bad continuation byte
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::ExpectedContinuation(97)));
    let bytes = &[97, 0xf4, 0x93, 0x93, 0x93]; // char out of range
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::InvalidChar(1127635)));
    let bytes = &[0b11000001, 0b10100001]; // overlong encoding (0b01100001 spread out over two bytes)
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::OverlongEncoding(97)));
  }

  #[test]
  fn encode_latin() {
    let text = "a";
    let res = encode_utf8(text.chars());
    assert_eq!(res, &[97]);
    assert_eq!(String::from_utf8(res).unwrap(), text)
  }

  #[test]
  fn encode_swedish() {
    let text = "å";
    let res = encode_utf8(text.chars());
    assert_eq!(res, &[195, 165]);
    assert_eq!(String::from_utf8(res).unwrap(), text)
  }

  #[test]
  fn encode_chinese() {
    let text = "你";
    let res = encode_utf8(text.chars());
    assert_eq!(res, &[228, 189, 160]);
    assert_eq!(String::from_utf8(res).unwrap(), text)
  }

  #[test]
  fn encode_emoji() {
    let text = "🌍";
    let res = encode_utf8(text.chars());
    assert_eq!(res, &[240, 159, 140, 141]);
    assert_eq!(String::from_utf8(res).unwrap(), text)
  }
}
