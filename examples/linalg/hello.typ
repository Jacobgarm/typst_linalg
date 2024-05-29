#import "./lib.typ": *
#{
  let m1 = math.mat((1,2),(3,4))
  let m2 = math.mat((2,1),(5,7))
  
  bytes_mat(p.mul(mat_bytes(m1), mat_bytes(m2)))
}

