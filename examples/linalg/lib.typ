#let p = plugin("./linalg.wasm")

#let mat_bytes(m) = bytes(m.rows.map(row => row.map(item => item.text).join(",")).join(";"))
#let bytes_mat(b) = math.mat(..str(b).split(";").map(row_s => row_s.split(",")))

#let mat_add(m1, m2) = bytes_mat(p.add(mat_bytes(m1), mat_bytes(m2)))

#let rowswap(m, r1, r2) = bytes_mat(p.rowswap(mat_bytes(m), bytes(str(r1)), bytes(str(r2))))