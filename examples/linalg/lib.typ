#let p = plugin("./linalg.wasm")

#let mat_bytes(m) = bytes(m.rows.map(row => row.map(item => item.text).join(",")).join(";"))
#let bytes_mat(b) = math.mat(..str(b).split(";").map(row_s => row_s.split(",")))

