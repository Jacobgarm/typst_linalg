#let p = plugin("./linalg.wasm")

#let mat_bytes(m) = bytes(m.rows.map(row => row.map(item => item.text).join(",")).join(";"))
#let bytes_mat(b) = math.mat(..str(b).split(";").map(row_s => row_s.split(",").map(entry_s => float(entry_s))))

#let mat_add(m1, m2) = bytes_mat(p.add(mat_bytes(m1), mat_bytes(m2)))
#let mat_sub(m1, m2) = bytes_mat(p.sub(mat_bytes(m1), mat_bytes(m2)))
#let mat_mul(m1, m2) = bytes_mat(p.mul(mat_bytes(m1), mat_bytes(m2)))

#let rowswap(m, r1, r2) = bytes_mat(p.rowswap(mat_bytes(m), bytes(str(r1)), bytes(str(r2))))
