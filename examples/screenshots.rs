use screenshots::Screen;
use std::{fs, time::Instant};

fn main() {
  let start = Instant::now();
  let screens = Screen::all().unwrap();

  for screen in screens {
    println!("capturer {:?}", screen);
    let mut image = screen.capture().unwrap();
    let bytes = image.bytes();

    println!("amount of bytes: {}", image.width() * image.height() * 4);
    println!("length of real bytes: {}", bytes.len());
  }

  println!("运行耗时: {:?}", start.elapsed());
}
