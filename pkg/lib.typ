#let p = plugin("./linalg.wasm")

#let mat_bytes(m) = bytes(m.rows.map(row => row.map(item => item.text.replace("−","-")).join(",")).join(";"))
#let bytes_mat(b) = math.mat(..str(b).split(";").map(row_s => row_s.split(",").map(entry_s => float(entry_s))))

#let vec(..a) = math.vec(..a.pos().map(item => str(item)))

#let vec_bytes(v) = bytes(v.children.map(item => item.text.replace("−","-")).join(","))
#let bytes_vec(b) = vec(..str(b).split(",").map(entry_s => float(entry_s)))

#let num_bytes(n) = bytes(str(n).replace("−","-"))
#let bytes_num(b) = float(str(b))
  
#let add(m1, m2) = bytes_mat(p.add(mat_bytes(m1), mat_bytes(m2)))
#let sub(m1, m2) = bytes_mat(p.sub(mat_bytes(m1), mat_bytes(m2)))
#let mul(m1, m2) = bytes_mat(p.mul(mat_bytes(m1), mat_bytes(m2)))
#let mul_vec(m, v) = bytes_vec(p.mul_vec(mat_bytes(m), vec_bytes(v)))

#let rowswap(m, r1, r2) = bytes_mat(p.rowswap(mat_bytes(m), num_bytes(r1), num_bytes(r2)))

#let REF(m) = bytes_mat(p.REF(mat_bytes(m)))
#let RREF(m) = bytes_mat(p.RREF(mat_bytes(m)))
#let inverse(m) = bytes_mat(p.inverse(mat_bytes(m)))
#let exp(m) = bytes_mat(p.exp(mat_bytes(m)))
#let pow(m, i) = bytes_mat(p.pow(mat_bytes(m), num_bytes(i)))

#let det(m) = bytes_num(p.det(mat_bytes(m)))
#let trace(m) = bytes_num(p.trace(mat_bytes(m)))
