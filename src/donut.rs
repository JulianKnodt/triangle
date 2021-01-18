                            fn main()
                      {let mut t=0f32;let ts
                  =|[x,y,z,r,w]:[f32;5]|{let q=
                (x*x+z*z).sqrt()-r;(q*q+y*y).sqrt
              ()-w};let m=|l|move|s|(s,2.*(s as f32
            )/l-1.);let mut o=vec![vec![b' ';64];36]
           ;loop{let(p,q)=(t         *3.).sin_cos();let
          (n,b)=(t*2.).                 sin_cos();for(i,
         u)in ( 0..64).                    map(m(64.)){for
         (j,v)in(0..36                       ).map(m(36. )
        ){for(_,z)in                         (0..32).map(
        m(32.)){ let                          l=u*n-v*b;if
       ts([u*b+ v*n                           ,l*q+z*p,z*q-l
       *p,0.7, 0.25                            ])<0.{o[j][i]=
        b".,-~:;=!*#$@"                       [((u*u+v*(v-
        1.)+v*(v-1.)+z*                      (z+1.))/((u*
         u+v*v+z*z)*(u*                   u+(v-1.)*(v-1.
          )+(z+1.)*(z+1.)                 )).sqrt()*11.).
           max(0.).min(11.).round()as usize];break;}}}}
            print!("\x1b[H");for r in&mut o[..]{print
              !("{}\n",std::str::from_utf8(r).unwrap
                ());r.fill(b' ');}t+=1e-3;}}/*bas
                  ed off https://www.a1k0n.net
                      /2011/07/20/donut-mat
                            h.html**/
