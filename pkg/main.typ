#import "./lib.typ" as mat
#{
  let m1 = math.mat((20,20),(0,20))
  let m2 = math.mat((0.2,1),(5,7))
  let m3 = math.mat((9, 1, 0), (0, 1, 0), (0, 0, 2))
  str(mat.trace(m3))
  mat.mul(mat.inverse(m3), m3)
}


