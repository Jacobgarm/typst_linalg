#import "@preview/cetz:0.1.2"
#import "./lib.typ" as mat

// Settings.
#let pxw = 1000pt
#let pxh = 700pt
#let fov = 90deg
#let near = 0.01
#let far = 100
#let move-speed = 1
#let rot-speed = 10deg
#let jump-power = 1.3

// Geometry of a cube.
#let cube = (
  vertices: (
    (-0.5,  0,  0.5),
    ( 0.5,  0,  0.5),
    ( 0.5, -1,  0.5),
    (-0.5, -1,  0.5),
    (-0.5,  0, -0.5),
    ( 0.5,  0, -0.5),
    ( 0.5, -1, -0.5),
    (-0.5, -1, -0.5),
  ),
  faces: (
    (0, 1, 2, 3, 0),
    (4, 5, 6, 7, 4),
    (1, 5),
    (0, 4),
    (2, 6),
    (3, 7),
  ),
)

// Other geometry.
#let other = (
  vertices: (
    (-5, -3, 0),
    (0, 0, 0),
    (1, 3, 0),
    (4, -3, 0),
    (-1, -2, 0),
    (-3, 2, 0),
    (-3.5, 0.5, 0),
    (1.5, 4, 0),
  ),
  faces: (
    (0, 1, 2, 3, 4, 0),
    (3, 1, 5, 6),
    (2, 7),
  ),
)

// All platforms.
#let world = (
  (pos: (0, 0, -4), size: (8, 0.5, 12), color: green),
  (pos: (-2.5, 0, -20), size: (3, 10, 10), color: blue),
  (pos: (2.5, 0, -20), size: (3, 10, 10), color: blue),
  (pos: (10, 0, -30), size: (5, 5, 5), color: eastern),
  (pos: (4, 0, -40), size: (6, 100, 6), color: yellow),
  (pos: (4, -5, -50), size: (3, 3, 3), color: aqua),
  (pos: (4, -5, -55), size: (3, 3, 3), color: aqua),
  (pos: (4, -5, -60), size: (3, 3, 3), color: aqua),
  (pos: (4, -3, -70), size: (6, 100, 6), color: yellow),
  (pos: (-10, -3, -70), size: (15, 100, 3), color: purple),
  (pos: (19, -3, -70), size: (15, 100, 3), color: purple),
  (pos: (-16, -3, -85), size: (3, 100, 15), color: green),
  (pos: (-10, -3, -90), size: (3, 3, 3), color: aqua),
  (pos: (-10, -1, -95), size: (3, 3, 3), color: aqua),
  (pos: (-5, 1, -95), size: (3, 3, 3), color: aqua),
  (pos: (-5, 3, -90), size: (3, 3, 3), color: aqua),
  (pos: (-5, 5, -85), size: (3, 3, 3), color: aqua),
  (pos: (-5, 7, -80), size: (3, 3, 3), color: aqua),
  (pos: (23, 0, -75), size: (3, 3, 3), color: aqua),
  (pos: (15, 2.75, -80), size: (3, 3, 3), color: aqua),
  (pos: (10, 5.75, -87), size: (2.25, 2.25, 2.25), color: aqua),
  (pos: (3, 8.75, -88), size: (1.5, 1.5, 1.5), color: aqua),
  (pos: (3.5, 9, -80), size: (5, 100, 5), color: red),
)

// Creates a matrix that describes a perspective transformation.
#let perspective(aspect, fov, far, near) = {
  let t = calc.tan(fov / 2)
  let x = 1 / (t * aspect)
  let y = -1 / t
  let c = -far / (far - near)
  let d = (-far * near) / (far - near)
  math.mat(
    (x, 0,  0, 0),
    (0, y,  0, 0),
    (0, 0,  c, d),
    (0, 0, -1, 0),
  )
}

// Creates a matrix that describes a translation.
#let translation(x, y, z) = {
  math.mat(
    (1, 0, 0, x),
    (0, 1, 0, y),
    (0, 0, 1, z),
    (0, 0, 0, 1),
  )
}

// Creates a matrix that describes a scaling.
#let scaling(x, y, z) = {
  math.mat(
    (x, 0, 0, 0),
    (0, y, 0, 0),
    (0, 0, z, 0),
    (0, 0, 0, 1),
  )
}

// Creates a matrix that describes a rotation around the Y axis.
#let rotation-y(r) = {
  let c = calc.cos(r)
  let s = calc.sin(r)
  math.mat(
     (c, 0, s, 0),
     (0, 1, 0, 0),
    (-s, 0, c, 0),
     (0, 0, 0, 1),
  )
}


// Turns normalized coordinates into screen coordinates.
#let screenify(x, y) = ((x + 0.5) * pxw, (y + 0.5) * pxh)

// Renders a single object in the level.
#let render-obj(obj, ts) = {
  ts = mat.mul(ts, translation(..obj.pos))
  ts = mat.mul(ts, scaling(..obj.scale))

  let lines = ()
  let (vertices, faces) = obj.geom

  for face in faces {
    let len = face.len()
    for i in range(len - 1) {
      let i2 = calc.rem(i + 1, len)
      let v1 = vertices.at(face.at(i))
      let v2 = vertices.at(face.at(i2))
      let (x1, y1, z1, w1) = mat.mul_vec(ts, (..v1, 1))
      let (x2, y2, z2, w2) = mat.mul_vec(ts, (..v2, 1))

      if (w1 >= near or w2 >= near) and w1 <= far and w2 <= far {
        // Find center point if one of the points is off-screen.
        if w1 <= 0 {
          let n = (w2 - near) / (w2 - w1)
          x1 = (n * x1) + ((1-n) * x2)
          y1 = (n * y1) + ((1-n) * y2)
          z1 = (n * z1) + ((1-n) * z2)
          w1 = near
        } else if w2 <= 0 {
          let n = (w1 - near) / (w1 - w2)
          x2 = (n * x2) + ((1-n) * x1)
          y2 = (n * y2) + ((1-n) * y1)
          z2 = (n * z2) + ((1-n) * z1)
          w2 = near
        }

        lines.push((
          start: screenify(x1 / w1, y1 / w1),
          end: screenify(x2 / w2, y2 / w2),
          stroke: {
            // Interpolate alpha and thickness between near and far plane.
            let alpha = (far - calc.min(w1, w2)) / far
            let color = rgb(..obj.color.components(alpha: false), alpha * 100%)
            let thickness = 2pt * alpha
            color + thickness
          },
          z: (z1 / w1 + z2 / w2) / 2,
        ))
      }
    }
  }

  lines
}

// Renders all objects into lines.
#let render-lines(ts) = {
  for platform in world {
    let obj = (
      geom: cube,
      pos: platform.pos,
      scale: platform.size,
      color: platform.color,
    )
    render-obj(obj, ts)
  }
  render-obj(
    (
      geom: other,
      pos: (0, 10, 50),
      scale: (-3, 3, 3),
      color: rgb(255, 255, 255),
    ),
    ts,
  )
}

// Renders all objects.
#let render-world(ts) = {
  for l in render-lines(ts).sorted(key: l => -l.z) {
    let _ = l.remove("z")
    place(line(..l))
  }
}


// Renders a minimap.
#let render-minimap(state) = cetz.canvas(length: 3pt, {
  import cetz.draw: *
  rect((-25, -5), (30, 100), stroke: gray, fill: black)
  for platform in world {
    let (x, _, z) = platform.pos
    let (sx, _, sz) = platform.size
    rect((x - sx/2, -z + sz/2), (x + sx/2, -z - sz/2), fill: platform.color)
  }
  let xz((x, _, z)) = (x, -z)
  arc(
    xz(state.pos),
    start: 90deg - state.rot - fov / 2,
    stop: 90deg - state.rot + fov / 2,
    fill: yellow,
    anchor: "origin",
    radius: 3,
    mode: "PIE",
  )
  circle(xz(state.pos), radius: 3pt, fill: red, name: "player")
})

// Finds the distance to solid ground.
#let sonar((x, y, z)) = {
  let alt = 100
  for obj in world {
    let (ox, oy, oz) = obj.pos
    let (sx, _, sz) = obj.size
    if (
      (ox - sx/2 <= x and x <= ox + sx/2) and
      (oz - sz/2 <= z and z <= oz + sz/2) and
      (oy <= y + 0.01)
    ) {
      alt = y - oy
    }
  }
  alt
}

// Determines the movement delta.
#let delta((x, y, z), rot, d) = {
  let beta = 90deg - rot
  let dx = d * calc.cos(beta)
  let dz = d * calc.sin(beta)
  (x + dx, y, z - dz)
}

// Whether the player is in the winzone.
#let within-winzone((x, y, z)) = (
  1.5 < x and x < 3.5 and 8.75 < y and y < 9.25 and -82.5 < z and z < -79.5
)

// Updates the game state.
#let update(state, u) = {
  if u == "e" {
    state.minimap = not state.minimap
    return state
  }

  if state.dead or state.won {
    return state
  }

  state.steps += 1

  if u == "w" {
    state.pos = delta(state.pos, state.rot, move-speed)
  } else if u == "s" {
    state.pos = delta(state.pos, state.rot, -move-speed)
  } else if u == "a" {
    state.rot -= rot-speed
  } else if u == "d" {
    state.rot += rot-speed
  }

  // What is the distance to solid ground?
  let ground-distance = sonar(state.pos)
  if u == " " and ground-distance < 0.1 {
    state.accel = jump-power
  }

  state.pos.at(1) += calc.max(state.accel, -ground-distance)
  state.accel = calc.max(state.accel - 0.3, -1)

  if state.pos.at(1) < -10 {
    state.dead = true
  }

  if within-winzone(state.pos) {
    state.won = true
  }

  state
}

// Renders a popup.
#let render-popup(body, subtitle: none) = place(center + horizon, rect(
  fill: black,
  width: 200pt,
  height: 100pt,
  stroke: 1pt + white,
  {
    text(20pt, underline(stroke: 2pt, offset: 4pt, upper(body)))
    if subtitle != none {
      v(15pt, weak: true)
      subtitle
    }
  }
))

// Renders the game UI.
#let render-ui(state) = block(inset: 10pt, width: 100%, height: 100%, {
  place(top + left, {
    [Mission status: ]
    if state.dead [Failed]
    else if state.won [Success]
    else [Active]
  })
  place(top + center, {
    [Assignment: Reach the goal]
  })
  place(top + right, {
    if state.minimap {
      render-minimap(state)
    } else {
      [Toggle GPS with E]
    }
  })

  if state.dead {
    render-popup[You died!]
  } else if state.won {
    render-popup(subtitle: [In #state.steps steps])[You win!]
  }
})

// Renders the game in a given state.
#let render(state) = {
  // Determine the view transformation.
  let (x, y, z) = state.pos
  let ts = mat.mul(
    perspective(pxw / pxh, fov, far, near),
    mat.mul(
      rotation-y(state.rot),
      translation(-x, -y - 1, -z),
    ),
  )

  // Render world and UI.
  render-world(ts)
  render-ui(state)
}

// Parses the document's body into an array of strings containing
// only the seven updates "w", "a", "s", "d", " ", and "e".
#let parse-updates(source) = {
  lower(source
    .replace(regex("(//|#).*\n?"), "")
    .split("\n")
    .join())
    .clusters()
    .filter(c => c in "wasd e")
}

// Entry point into the game.
#let game(source) = {
  set page(fill: black, width: pxw, height: pxh, margin: 0pt)
  set text(font: "Cascadia Code", fill: green, 11pt)

  // The initial state.
  let state = (
    pos: (0, 0, 0),
    steps: 0,
    accel: -1,
    rot: 0deg,
    minimap: false,
    dead: false,
    won: false,
  )

  // Handles game updates.
  let updates = parse-updates(source)
  for u in updates {
    state = update(state, u)
  }

  render(state)
}
