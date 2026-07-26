# Screenshot smoke reference frames

Two kinds of committed reference live here:

- **`*.grid.json`** — coarse (16x12) luminance-grid *signatures*, one per smoke
  check, that `scripts/smoke` compares each capture against. Each grid is
  normalized to its brightest cell, so the comparison is about spatial
  distribution (did geometry move, disappear, or get recolored/replaced?), not
  absolute brightness — and it uses a generous tolerance so adapter/driver
  variation does not cause false failures (see `WI-SMOKE-0001` non-goals). This
  is *not* pixel-perfect image diffing. Re-baseline with
  `SMOKE_UPDATE_REFS=1 scripts/smoke` on a trusted environment when the demos
  intentionally change.
- **`*.png`** — reference captures for **human comparison only**; the check does
  not diff against them. `preset-*.png` show the three non-default DP-0007
  display presets (`MonochromeBeam`, `ColorQuadraScan`, `CleanNeon`) on the
  deterministic pre-impact tester frame, and `blasterites-{wide,tall}.png` show
  the non-4:3 letterbox/pillarbox behavior.

The check also asserts structural properties independent of any reference (not
all-black, not all-white, geometry present, pre- vs post-impact frames differ,
non-4:3 letterbox margins dark).

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
