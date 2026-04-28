#[derive(Debug, PartialEq, Eq)]
pub enum UTF8Error {
  UnexpectedEOF,
  ExpectedContinuation(u8),
  InvalidByte(u8),
  InvalidChar(u32),
  OverlongEncoding(u32),
}

/// Validates and decodes utf-8 encoded bytes.
pub fn decode_utf8(bytes: &[u8]) -> Result<Vec<char>, UTF8Error> {
  #[inline]
  fn get_continuation<'a, A: Iterator<Item = &'a u8>>(iter: &mut A) -> Result<u8, UTF8Error> {
    match *iter.next().ok_or(UTF8Error::UnexpectedEOF)? {
      // 0b10xxxxxx
      b @ 0x80..0xC0 => Ok(b & 0b00111111),
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
        let b = get_continuation(&mut iter)? as u32;
        (a << 6 | b, 0x80)
      }
      a @ 0xE0..0xF0 => {
        // 0b1110xxxx
        let a = (a & 0b00001111) as u32;
        let b = get_continuation(&mut iter)? as u32;
        let c = get_continuation(&mut iter)? as u32;
        (a << 12 | b << 6 | c, 0x800)
      }
      a @ 0xF0..0xF8 => {
        // 0b11110xxx
        let a = (a & 0b00000111) as u32;
        let b = get_continuation(&mut iter)? as u32;
        let c = get_continuation(&mut iter)? as u32;
        let d = get_continuation(&mut iter)? as u32;
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn latin() {
    let text = "salve mundus";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn chinese() {
    let text = "你好世界";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn emoji() {
    let text = "👋🌍";
    let utf8 = decode_utf8(text.as_bytes()).unwrap();
    let text2 = String::from_iter(utf8);
    assert_eq!(text, text2);
  }

  #[test]
  fn invalid() {
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
}
