use std::fs::File;
use std::io::{BufRead,BufReader};
fn main() {
  let f = File::open("donut_src.rs").expect("Could not find source file");
  let buf = BufReader::new(f);
  let mut read=String::from("");
  for line in buf.lines() {
    read += &line.expect("ok");
  }
  let mut char_iter = read.chars();
  let sphere_sdf = |x:f32,y:f32, r:f32| x*x+y*y-r;
  for y in 0..32 {
    for x in 0..64 {
      let u = x as f32;
      let v = y as f32;
      let r = 32.;
      if sphere_sdf((u-r)/2.,v-r/2.,150.) < 0. &&
        sphere_sdf((u-r)/2.,v-r/2.,40.) > 0.
      {
        print!("{}", char_iter.next().unwrap_or('*'));
      } else {
        print!(" ");
      }
    }
    println!();
  }
  assert_eq!(char_iter.next(), None, "Not enough space");
}
