fn main(){
  let mut t=0f32;
  let torus_sdf=|[x,y,z,r,w]:[f32;5]|{
    let q=(x*x+z*z).sqrt()-r;(q*q+y*y).sqrt()-w
  };
  let m=|l|move|s|(s,2.*(s as f32)/l-1.);
  let mut output=vec![vec![b' ';64];36];
  loop{
    let(xs,xc)=(t*3.).sin_cos();
    let(zs,zc)=(t*2.).sin_cos();

    for(i,u)in(0..64).map(m(64.)){
      for(j,v)in(0..36).map(m(36.)){
        for(_,z)in(0..32).map(m(32.)){
          let vt=u*zs-v*zc;
          if torus_sdf([u*zc+v*zs,vt*xc+z*xs,z*xc-vt*xs,0.7,0.25])<0.{
            let al=(u*u+v*(v-1.)+z*(z+1.))/
              ((u*u+v*v+z*z)*(u*u+(v-1.)*(v-1.)+(z+1.)*(z+1.))).sqrt();
            output[j][i]=b".,-~:;=!*#$@"[(al*11.).max(0.).round()as usize];
            break;
          }
        }
      }
    }

    print!("\x1b[H");
    for r in output.iter_mut(){
      print!("{}\n", std::str::from_utf8(r).unwrap());
      r.fill(b' ');
    }
    t+=0.0005;
  }
}
