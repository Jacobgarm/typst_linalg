#import "./lib.typ" as mat
#{
  let m1 = math.mat((-4,53),(3,1))
  let m2 = math.mat((0.2,1),(5,7))
  let m3 = math.mat((1, 4, 2), (0, 0, 5), (0, 2, 0))
  mat.add(mat.REF(m1),m1)
  mat.rowswap(m1,1,4)
  $m1 m2 =$ 
  mat.mul(mat.REF(m1), m2)
}


