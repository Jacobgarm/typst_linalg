#import "./lib.typ" as mat
#{
  let m1 = math.mat((20,20),(0,20))
  let m2 = math.mat((0.2,1),(5,7))
  let m3 = math.mat((0, 4, 2), (0, 0, 5), (1, 0, 0))
  mat.RREF(m3)
}


