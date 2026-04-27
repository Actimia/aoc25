#[derive(Debug, PartialEq, Eq)]
pub enum UTF8Error {
  UnexpectedEOF,
  BadContinuation(u8),
  BadByte(u8),
  BadChar(u32),
  OverlongEncoding(u32),
}

/// Validates and decodes utf-8 encoded bytes.
pub fn decode_utf8(bytes: &[u8]) -> Result<Vec<char>, UTF8Error> {
  #[inline]
  fn get_continuation<'a, A: Iterator<Item = &'a u8>>(iter: &mut A) -> Result<u8, UTF8Error> {
    match *iter.next().ok_or(UTF8Error::UnexpectedEOF)? {
      b @ 128..192 => Ok(b & 0b00111111),
      err => Err(UTF8Error::BadContinuation(err)),
    }
  }

  let mut res: Vec<char> = Vec::default();
  let mut iter = bytes.iter();
  while let Some(byte) = iter.next() {
    let (ch, first) = match *byte {
      a @ 0..128 => (a as u32, 0),
      a @ 192..224 => {
        let a = (a & 0b00011111) as u32;
        let b = get_continuation(&mut iter)? as u32;
        (a << 6 | b, 0x80)
      }
      a @ 224..240 => {
        let a = (a & 0b00001111) as u32;
        let b = get_continuation(&mut iter)? as u32;
        let c = get_continuation(&mut iter)? as u32;
        (a << 12 | b << 6 | c, 0x800)
      }
      a @ 240.. => {
        let a = (a & 0b00000111) as u32;
        let b = get_continuation(&mut iter)? as u32;
        let c = get_continuation(&mut iter)? as u32;
        let d = get_continuation(&mut iter)? as u32;
        (a << 18 | b << 12 | c << 6 | d, 0x10000)
      }
      err => return Err(UTF8Error::BadByte(err)),
    };
    if ch < first {
      return Err(UTF8Error::OverlongEncoding(ch));
    }
    res.push(char::from_u32(ch).ok_or(UTF8Error::BadChar(ch))?)
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
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::BadByte(167)));
    let bytes = &[97, 32, 229, 167]; // not enough continuation bytes
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::UnexpectedEOF));
    let bytes = &[97, 32, 229, 167, 97]; // bad continuation byte
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::BadContinuation(97)));
    let bytes = &[97, 0xf4, 0x93, 0x93, 0x93]; // char out of range
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::BadChar(1127635)));
    let bytes = &[0b11000001, 0b10100001]; // overlong encoding
    assert_eq!(decode_utf8(bytes), Err(UTF8Error::OverlongEncoding(97)));
  }
}
