# Screenshot smoke reference frames

These committed PNGs are **reference captures for human comparison only** — the
`scripts/smoke` check asserts *structural* properties of the rendered pixels
(not all-black, not all-white, geometry present, letterbox margins dark), not
pixel-perfect equality, because WebGPU output varies by adapter and driver (see
`WI-SMOKE-0001` non-goals).

Frames here (all the deterministic Blasterites tester at `t=2000ms`, pre-impact):

- `blasterites-wide.png` — a wide (non-4:3) viewport. The scene stays a centered
  4:3 playfield with black **pillarbox** bars left and right; geometry is not
  stretched. This is the non-4:3 resize behavior EV-0009 did not cover.
- `blasterites-tall.png` — a tall (non-4:3) viewport, with black **letterbox**
  bars top and bottom.

The canonical exact-4:3 pre-impact (`t=2000ms`) and post-impact (`t=4000ms`)
frames are recorded in `project/evidence/EV-0009/`.

To regenerate, serve the demos (`scripts/demos`) and open, at the desired
window size:

    /?demo=blasterites&frame&t=2000

then capture the canvas.
