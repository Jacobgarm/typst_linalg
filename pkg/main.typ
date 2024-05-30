#import "./lib.typ" as mat
#{
  let m1 = math.mat((-4,2),(3,3))
  let m2 = math.mat((0.2,1),(5,7))
  let m3 = math.mat((1, 4, 2), (0, 0, 5), (0, 2, 0))
  mat.add(mat.REF(m1),m1)
}


