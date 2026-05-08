# TODO — pending fixes from audit (2026-05-06)

Items deferred from a third-party code audit. Listed roughly in order of impact.

## 1. IdleInhibitHandler is a no-op (real bug)

`src/handlers/mod.rs:415–418` — `inhibit()` and `uninhibit()` do nothing.
`IdleNotifierState` is wired up, so swayidle/screensaver tick on schedule even
while mpv/Firefox is requesting inhibit. Result: screen locks while watching
video.

Fix (niri pattern, ~30 lines):

1. Add `idle_inhibiting_surfaces: HashSet<WlSurface>` to `DriftWm`.
2. `inhibit`/`uninhibit` insert/remove from the set.
3. Each refresh, compute
   `is_inhibited = surfaces.iter().any(|s| surface_primary_scanout_output(s, states).is_some())`
   — the `primary_scanout_output` check filters hidden inhibitors (e.g. a
   background browser tab playing video while another window is fullscreen).
4. Call `idle_notifier_state.set_is_inhibited(is_inhibited)`.

Reference: `niri.rs:3993–4005` (`refresh_idle_inhibit`).

## 2. Runtime `unsafe { std::env::set_var(...) }` in config hot-reload

`src/state/reload.rs` lines 99, 109, 118, 120, 143, 149 mutate process env
*after* threads exist (Mesa shader threads, libinput, glibc NSS). POSIX
`setenv` is not thread-safe with concurrent `getenv`; this is exactly what
Rust 2024 made `unsafe` for.

Fix: route `[env]` through `Command::env()` instead of process env. Same
hot-reload semantics, no UB. Five touchpoints:

1. `state/mod.rs:134` — `spawn_command(cmd, env: &HashMap<String,String>)`,
   call `child.envs(env)`. Update callers in `input/actions.rs:30, 39` and
   `main.rs:203`.
2. `xwayland.rs:61` — `.envs(&state.config.env)` on satellite Command. Drop
   `unsafe { set_var("DISPLAY", …) }` at line 100; pass `DISPLAY=:N` via
   `.env()` to the `dbus-update-activation-environment` shell-out at line 174.
3. `state/reload.rs` — delete the env diff loop (lines 139–151) and the four
   XCURSOR env sets; replace with `self.config.env = new_config.env;` at the
   bottom. Cursor settings already live in `self.config`; nothing in driftwm
   actually reads `XCURSOR_*` from process env.
4. `config/mod.rs:218–235` — drop the startup env sets (sound today since
   pre-thread, but keeps the policy "never touch process env" simple).
5. `main.rs:118–140` — `systemctl import-environment` shell-out: pass values
   via `Command::env()` instead of relying on process env.

After: only `set_var` left is the small early-startup block in `main.rs`
(`RUST_LOG`, `DRIFTWM_CONFIG`, `WAYLAND_DISPLAY`, four `XDG_*`) — all
pre-thread, all sound. `state/reload.rs` has zero `unsafe`.

## 3. Pin smithay git revision

`Cargo.toml:9, 21` use `git = "..."` without `rev =`. Cargo.lock saves us on
fresh clones, but `cargo update` silently bumps to whatever's on smithay
master, which breaks regularly. niri pins (`Cargo.toml:31`).

Fix: copy current rev from `Cargo.lock` into both smithay deps in
`Cargo.toml` as `rev = "..."`. Five-minute change.

## 4. `active_output().unwrap()` panic surface

`active_output()` returns `None` when zero outputs exist (laptop lid + all
externals unplugged, brief windows during udev hotplug). Several call
sites unwrap it. The `seat.get_pointer().unwrap()` calls are fine
(infallible-by-construction after init); only `active_output().unwrap()`
matters.

Fix: `grep -rn 'active_output().unwrap()'`, audit each, replace with
`if let Some(out) = self.active_output()` and a sensible no-op early
return where the caller can tolerate it.

Skip the broader 186-call unwrap sweep — most are infallible by
construction (seat capabilities, post-init invariants).

## 5. Replace hand-rolled JSON in persistence with serde_json

`src/state/persistence.rs:185–206` builds the `windows=[…]` JSON line via
`format!()` + a custom `json_escape()` (lines 238–254). Current escape
function handles quotes, backslashes, control chars `< 0x20` — the
audit's specific U+2028/U+2029 concern only matters for JS `eval()`-based
consumers (pre-ES2019), not for `JSON.parse` / `jq` / Python `json`.

Still worth replacing: `serde` is already in deps, adding `serde_json` is
one Cargo.toml line. `WindowFingerprint` gets `#[derive(Serialize)]`,
the format! and `json_escape` are deleted, ~30 lines smaller, zero risk
of escape bugs ever.

Low priority — current implementation is correct for realistic inputs.

## 6. Add `ext-background-effect` protocol support

Currently the only path to blur a window is `[window-rules]` config matched
on app_id (`src/render/mod.rs:736`). Clients can't request blur themselves
and per-surface blur regions are unsupported. Niri implements the protocol;
smithay's `wayland::background_effect` ships in the git checkout already
referenced from `Cargo.toml:9`, so no dep bump.

Reference: `niri/src/handlers/background_effect.rs`,
`niri/src/render_helpers/background_effect.rs`.

Ship 6a+6b+6c together. 6a+6b alone advertises `Capability::Blur` but
ignores the region for uniformly-translucent surfaces, which is wrong under
the spec — driftwm's existing alpha-mask pass at `src/render/blur.rs:420–449`
covers translucent-on-opaque (GTK4 headers, typical CSD) for free, but
leaks blur outside the requested region for fully-translucent surfaces
(notification daemons, custom shells, dropdowns).

### 6a. Protocol handler

New file `src/handlers/background_effect.rs`:

1. `BackgroundEffectState::new::<DriftWm>(&dh)` in init alongside the other
   protocol globals.
2. `impl ExtBackgroundEffectHandler for DriftWm` returning
   `Capability::Blur`. `set_blur_region` / `unset_blur_region` mark a
   per-surface `CachedBlurRegionUserData` (in surface data_map) as
   pending-dirty; lazily install one `add_post_commit_hook` per surface
   that flips pending→dirty on commit and triggers re-damage.
3. `delegate_background_effect!(DriftWm);`
4. `get_cached_blur_region(states) -> Option<Arc<Vec<Rectangle<i32, Logical>>>>`
   helper that reads `BackgroundEffectSurfaceCachedState::current().blur_region`
   on first access and converts to non-overlapping rects via
   `region_to_non_overlapping_rects` (port from `niri/src/utils/region.rs`).
   Cache the `Arc` so render can clone cheaply.

### 6b. Blur trigger integration

In `src/render/mod.rs:736`, OR the window-rule check with a protocol check:

```rust
let client_blur = with_states(&surface, |s| {
    crate::handlers::background_effect::get_cached_blur_region(s)
}).is_some();
let wants_blur = blur_enabled && (rule.blur || client_blur);
```

Mirror the same change at the layer-shell call site
(`src/render/mod.rs:383–388`).

### 6c. Subregion clipping

1. Extend `BlurRequestData` (`src/render/blur.rs:197`) with
   `region_rects: Option<Arc<Vec<Rectangle<i32, Physical>>>>` (None = whole
   window). Convert from logical surface-local to physical screen rects
   when building the request at `src/render/mod.rs:946`; clip each rect to
   the window bounds.
2. In the mask pass (`src/render/blur.rs:365–449`), when `region_rects.is_some()`:
   clear `cache.mask` to alpha=0 first, then for each rect render the
   surface alpha *only inside that rect* (scissor or per-rect quad). The
   existing alpha-multiply blend at lines 427–432 then clips blur to the
   union of the rects.
3. Fold the rect list into `hash_background_elements`
   (`src/render/blur.rs:18`) so `last_background_hash` invalidates when a
   client changes its region without moving or resizing the window.
4. Drop the per-surface `CachedBlurRegionUserData` when the surface is
   destroyed — smithay's data_map drops automatically with the surface,
   verify no leak via `blur_cache.retain` analogue at
   `src/render/mod.rs:1053`.
