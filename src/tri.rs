fn
main
(){type
V=[f32; 3
];  let mut
t=0f32;   let
  dot=|[x,y,z]
:V,[a,b,c]:V|x*a
+y*b+z*c;let dot2=
|a|dot(a,a);let cross
 =|[x,y,z]:V,[a,b,c]:V
|[y*c-z*b,z*a-x*c,x*b-y*
a];let smul=|[x,y,z]:V,a|[
x*a,y*a,z*a];let m=|[x,y,z]:
V,[a,b,c]:V|[x-a,y-b,z-c];let 
n2=|[x,y,z]:V|(x*x+y*y+z*z);let 
triangle_sdf=|p,a,b,c|{let[pa, pb,
pc,ba,cb,ac]=[m(p,a),m(p,b),m(p,c),m(
b,a),m(c,b),m(a,c)];let nor=cross(ba,
ac);let o=|p,q|dot(cross(p,nor),q).signum
();if o(ba,pa)+o(cb,pb)+o(ac,pc)<2.{let s=
|p,q|dot2(m(smul(p,(dot(p,q)/dot2(p)).min(1.
).max(0.)),q));s(ba,pa).min(s(cb,pb)).min(s(ac
,pc))}else{dot(nor,pa)*dot(nor,pa)/dot2(nor)}.
sqrt()};let mut output=vec![vec![b' '; 64]; 36];let 
q=|l|move|s|(s,2.*(s as f32)/l-1.);loop{for(i,u)in(0
..64).map(q(64.)){for(j,v)in(0..36).map(q(36.)){for(_,
z)in(0..32).map(q(32.)){let[p,a,b,c]=[[u,v,z],[t.sin(),(
t*2.).cos(),(t+3.).sin()],[(3.*t).cos(),(4.*t).sin(),(2.*t
+0.5).cos()],[(5.*t).sin(),(6.*t).cos(),(2.5*t+1.4).sin()]];
if triangle_sdf(p,a,b,c)<0.05{let n=cross(m(a,b),m(a,c));let l
 =m(p,[0.;3]);let al=dot(n,l).max(dot(smul(n,-1.),l))/(n2(n)*n2(
l)).sqrt();output[j][i]=b".,-~:;=!*#$@"[(al*11.).max(0.).round()as
 usize];break;}}}}print!("\x1b[H");for r in output.iter_mut(){print!
("{}\n",std::str::from_utf8(r).unwrap());r.fill(b' ');}t += 0.001;}}//
