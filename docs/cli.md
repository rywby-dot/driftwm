# CLI reference

<!-- Generated from the clap command tree — do not edit by hand.
     Regenerate with: UPDATE_CLI_DOCS=1 cargo test docs_cli_md_is_up_to_date -->

driftwm's command-line interface: the root command that starts the compositor,
and every `driftwm msg` subcommand for controlling a running one. For the raw
JSON wire protocol behind `msg`, see [ipc.md](ipc.md).

## `driftwm`

```
driftwm [OPTIONS] [COMMAND]
```

A trackpad-first infinite canvas Wayland compositor.

With no subcommand, starts the compositor, auto-detecting the backend (udev on a TTY, winit when nested). The `msg` subcommand instead talks to an already-running instance over its IPC socket; see docs/ipc.md for the raw wire protocol.

- `--backend <udev|winit>` — Backend to use [default: udev on a TTY, winit if nested]
- `--config <PATH>` — Use an alternate config file
- `--check-config` — Validate the config and exit

### `driftwm msg`

```
driftwm msg [OPTIONS] <COMMAND>
```

Send a command to the running compositor over its IPC socket.

Auto-targets the instance named by `WAYLAND_DISPLAY` (override with `DRIFTWM_SOCKET`). A subcommand with no arguments reads state; with arguments it writes. Add `--json` for the raw JSON reply.

- `--json` — Print the raw JSON reply

#### `driftwm msg camera`

```
driftwm msg camera [X] [Y]
```

Get the camera position, or set it (animated) with `<x> <y>` (viewport center, Y-up).

With no arguments, prints the current camera position. With `<x> <y>`, pans the viewport (animated) to center that canvas point; positive `y` is up.

Reply: `{"Ok":{"Camera":{"x":500.0,"y":300.0}}}`.

#### `driftwm msg zoom`

```
driftwm msg zoom [LEVEL]
```

Get the zoom level, or set it with `<level>` (clamped to the supported range).

Setting is animated and clamped to the supported range (out to fit-all, in to native resolution — no magnification).

Reply: `{"Ok":{"Zoom":0.5}}`.

#### `driftwm msg layout`

```
driftwm msg layout [OPTIONS]
```

Print the active keyboard layout (full XKB name, e.g. `English (US)`).

With `--short`, prints the configured layout code for the active group (e.g. `us`, `ru`) instead — what most status bars want.

Reply: `{"Ok":{"Layout":"English (US)"}}` (or `"us"` with `--short`).

- `--short` — Print the configured layout code instead (e.g. `us`, `ru`)

#### `driftwm msg state`

```
driftwm msg state
```

Dump camera, zoom, and the window inventory.

Prints camera, zoom, keyboard layout, and every window — each with a stable `id` usable as a selector for `focus`/`move`/`close`/`screenshot window` — plus fullscreen, pinned, layer-shell, and per-output details.

Reply: `{"Ok":{"State":{"camera":[..],"zoom":1.0,"windows":[..],"outputs":[..]}}}`.

#### `driftwm msg debug-counters`

```
driftwm msg debug-counters
```

Print internal collection sizes for leak diagnosis (unstable keys).

An introspection endpoint, not a stable interface: the keys are internal field names that can change between releases. Meant for leak diagnosis — a count should return to its idle baseline once the windows and clients that raised it are gone.

Reply: `{"Ok":{"DebugCounters":{"decorations":2,"stage_entries":2}}}`.

#### `driftwm msg subscribe`

```
driftwm msg subscribe
```

Stream state snapshots as they change (one JSON line per event with --json).

Turns the connection into a live feed: the server acks, then pushes one event with the current state immediately and again on every change (per rendered frame while something animates, not throttled). Runs until interrupted.

Each event is `{"State":{..}}` — the whole snapshot, same shape as the `state` reply, and not wrapped in `Ok`/`Err`.

#### `driftwm msg focus`

```
driftwm msg focus [OPTIONS] [APP_ID]
```

Print the focused window, or focus a window by app_id substring or `--id` (the stable id shown in `state`).

With no argument, prints the focused window's `id` and `app_id`. Given an `app_id` substring (case-insensitive) or `--id <n>`, focuses that window, navigating to it only if it is off-screen.

Reply: `{"Ok":{"Focused":{"id":5,"app_id":"alacritty"}}}` (or `{"Ok":{"Focused":null}}`).

- `--id <ID>` — Focus the window with this stable id (from `state`)

#### `driftwm msg move`

```
driftwm msg move [OPTIONS] [X] [Y]
```

Get a window's position, or move it (center, Y-up) with `<x> <y>`. Targets the focused window, or `--id` (the stable id shown in `state`).

Positions are a center point with `y` pointing up. Pinned and fullscreen windows live in screen space, not on the canvas, so `move` refuses to reposition them.

Reply: `{"Ok":{"Position":{"x":100,"y":200}}}`.

- `--id <ID>` — Target the window with this stable id (from `state`)

#### `driftwm msg opacity`

```
driftwm msg opacity [OPTIONS] [VALUE]
```

Get a window's opacity, or set it with `<value>` (0.0–1.0). Targets the focused window, or `--id` (the stable id shown in `state`).

Applies to any rendered window, pinned and fullscreen included; the change takes effect next frame. The value is runtime-only — seeded from an `opacity` window rule at map time, never persisted. Values outside `0.0`–`1.0` are rejected, not clamped; a window no rule touched reads `1`.

Reply: `{"Ok":{"Opacity":0.85}}`.

- `--id <ID>` — Target the window with this stable id (from `state`)

#### `driftwm msg close`

```
driftwm msg close [OPTIONS] [APP_ID]
```

Close the focused window, or a window by app_id substring or `--id`.

Targets the focused window by default, or a window by `app_id` substring (case-insensitive) or `--id <n>` (from `state`). Errors when nothing matches.

Reply: `{"Ok":"Ok"}`.

- `--id <ID>` — Close the window with this stable id (from `state`)

#### `driftwm msg action`

```
driftwm msg action <SPEC>...
```

Run a config action, e.g. `action close-window`, `action quit`, `action switch-layout next`.

Runs any compositor action by the same string you would write in a config keybinding, parsed with the exact config parser, so every keybindable action is reachable. Replies `Ok` whenever the spec parses — even if it had no effect (e.g. `close-window` with nothing focused); only an unparseable spec errors.

The socket is a full control surface: `action` can `exec`/`spawn`, `quit`, and `reload-config`. It is safe only because the socket is `0600`.

Reply: `{"Ok":"Ok"}`.

- `<SPEC>...` — Action and arguments, exactly as written in config (e.g. `nudge-window up`)

#### `driftwm msg screenshot`

```
driftwm msg screenshot [OPTIONS] [COMMAND]
```

Capture a canvas PNG (custom DPI). With no subcommand, captures the active output's current view of the canvas.

A canvas capture, not a screen grab: it re-renders a virtual viewport onto the canvas, reaching off-screen content at any resolution. Windows get full chrome; panels/layer-shells and blur are not drawn (use `grim` for a literal grab). `-o -` streams the PNG to stdout. Captures tile internally but cap at 16384 px/side.

Reply: `{"Ok":{"Screenshot":{"path":"/abs/shot.png","width":1920,"height":1080}}}`.

- `--scale <SCALE>` — Pixels per canvas unit — higher captures more detail than the screen shows (default: `1`)
- `-o, --output <OUTPUT>` — Output PNG path, or `-` for stdout [default: ./driftwm-screenshot-<time>.png]

##### `driftwm msg screenshot window`

```
driftwm msg screenshot window [OPTIONS] [APP_ID]
```

The focused window, or a window by app_id substring or `--id`.

Composed alone on transparency, so overlapping windows never appear; pinned and fullscreen windows capture like any other (a fullscreen window has no chrome). Reply shape is the shared `Screenshot` reply above.

- `--id <ID>` — Capture the window with this stable id (from `state`)

##### `driftwm msg screenshot all`

```
driftwm msg screenshot all
```

The bounding box of all non-widget windows.

A scene with the canvas background plus every window's chrome, framed with a `[zoom] fit_padding` margin. Reply shape is the shared `Screenshot` reply above.

##### `driftwm msg screenshot region`

```
driftwm msg screenshot region [OPTIONS] <COORDS>...
```

A rectangle — `X Y W H` (canvas coords, center/Y-up) or slurp's native `X,Y WxH`. Commas and the `x` separator are tolerated, so `$(slurp)` drops in directly. Treated as output-screen pixels with `--from-screen`.

Captures a scene (canvas background plus window chrome) over the rectangle. Reply shape is the shared `Screenshot` reply above.

- `<COORDS>...` — Four ints `X Y W H`, or slurp's `X,Y WxH` (quoted or not)
- `--from-screen` — Treat the rectangle as output-screen pixels mapped via the active viewport
