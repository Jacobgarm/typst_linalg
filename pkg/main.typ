#import "./lib.typ" as mat
#{
  let m1 = math.mat((20,20),(0,20))
  let m2 = math.mat((0.2,1),(5,7))
  let m3 = math.mat((0.5, 1, 0), (0, 1, 0), (0, 0, 2))
  // let v = mat.vec(1,2)
  // mat.mul_vec(m1,v)
  mat.mul(mat.inverse(m3), m3)
}


