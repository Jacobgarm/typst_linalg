#import "./lib.typ": *
#{
  let m1 = math.mat((1,2),(3,4))
  let m2 = math.mat((2,1),(5,7))
  
  bytes_mat(p.neg(mat_bytes(m1)))
}

