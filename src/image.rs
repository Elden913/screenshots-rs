use png::{BitDepth, ColorType, Encoder, EncodingError};

pub struct Image {
  width: u32,
  height: u32,
  bytes: Vec<u8>,
}

impl Image {
  pub fn new(width: u32, height: u32, bytes: Vec<u8>) -> Self {
    Image {
      width,
      height,
      bytes,
    }
  }

  pub fn from_bgra(width: u32, height: u32, bgra: Vec<u8>) -> Result<Self, EncodingError> {
    let mut bytes = bgra;

    // BGRA 转换为 RGBA
    for i in (0..bytes.len()).step_by(4) {
      let b = bytes[i];
      let r = bytes[i + 2];

      bytes[i] = r;
      bytes[i + 2] = b;
      bytes[i + 3] = 255;
    }

    Ok(Image::new(width, height, bytes))
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn bytes(&self) -> &Vec<u8> {
    &self.bytes
  }
}
