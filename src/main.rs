fn main() {
  let width = 64;
  let height = 36;
  let steps = 32;
  let lums = b".,-~:;=!*#$@";
  let mut t: f32 = 0.01;

  //let sphere_sdf = |x, y, z| x * x + y * y + z * z - 0.5;
  let torus_sdf = |x: f32, y: f32, z: f32, r:f32, w:f32| {
    let q = (x * x + z * z).sqrt() - r;
    (q * q + y * y).sqrt() - w
  };

  let mut output = vec![vec![b' '; width]; height];

  print!("\x1B[2J\x1B[1;1H");
  loop {
    let (xs, xc) = (t * 3.5).sin_cos();
    let (zs, zc) = (t * 1.7).sin_cos();

    for (i, u) in (0..width).map(|u| (u, (2. * (u as f32) / (width as f32)) - 1.)) {
      for (j, v) in (0..height).map(|v| (v, 2. * (v as f32) / (height as f32) - 1.)) {
        for z in (0..steps).map(|z| 2. * (z as f32) / (steps as f32) - 1.) {
          let vt = u * zs - v * zc;
          if torus_sdf(u * zc + v * zs, vt * xc + z * xs, -vt * xs + z * xc, 0.6, 0.2) < 0.
          {
            let len = (u * u + v * v + z * z).sqrt();
            let n = [u / len, v / len, z / len];

            let len = (u * u + (v - 1.) * (v - 1.) + (z + 1.) * (z + 1.)).sqrt();
            let l = [u / len, (v - 1.) / len, (z + 1.) / len];

            let al = l[0] * n[0] + l[1] * n[1] + l[2] * n[2];

            output[j][i] = lums[(al * (lums.len() - 1) as f32).max(0.).round() as usize];
            break;
          }
        }
      }
    }

    print!("\x1b[H");
    for r in output.iter_mut() {
      print!("{}\n",std::str::from_utf8(r).unwrap());
      r.fill(b' ');
    }
    t += 0.001;
  }
}
