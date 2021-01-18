use std::fs::File;
use std::io::{BufRead,BufReader};
fn main() {
  let f = File::open("tri_src.rs").expect("Could not find source file");
  let buf = BufReader::new(f);
  let mut read=String::from("");
  for line in buf.lines() {
    read += &line.expect("ok").trim();
  }
  let mut rows = 0;
  let mut total = 0;
  while total < read.len() {
    total += (rows+1)*2;
    rows += 1;
  }
  let mut char_iter = read.chars();


  for y in 0..rows {
    for _ in 0..2*(y+1) {
      print!("{}", char_iter.next().unwrap_or('*'))
    }
    println!();
  }
  assert_eq!(char_iter.next(), None, "Not enough space");
}
