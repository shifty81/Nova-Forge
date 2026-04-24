# Nova-Forge — Full Implementation Plan
> Generated: 2026-04-24  
> Source: Development session covering bug fixes, server GUI, worldgen preview, and Nova-Forge Studio asset editor.

---

## 🔴 Priority 0 — Immediate Bug Fixes

These are crashes and broken builds affecting every session right now.

### Fix 1 — `quality: Rare` crash (Game-breaking panic on singleplayer load)

**Root cause:** 13+ recipe RON files across `assets/common/items/recipes/` use `quality: Rare`,
but the `Quality` enum in `common/src/comp/inventory/item/mod.rs` never had a `Rare` variant.
Any NPC spawning with leather gear triggers the panic immediately on world load.

| File | Change |
|---|---|
| `common/src/comp/inventory/item/mod.rs` | Add `Rare` between `High` and `Epic` in `Quality` enum |
| `voxygen/src/hud/mod.rs` | Add `QUALITY_RARE` color constant + handle in `get_quality_col()` |
| `common/src/trade.rs` | Add `Quality::Rare` sell discount arm (`0.55`) |
| `common/src/bin/csv_import.rs` | Add `"Rare" => Quality::Rare` to armor + weapon import match blocks |
| `voxygen/src/hud/bag.rs` | Add `Quality::Rare` arm to slot image match |
| `voxygen/src/hud/crafting.rs` | Add `Quality::Rare` arm to slot image match |
| `voxygen/src/hud/loot_scroller.rs` | Add `Quality::Rare` arm to slot image match |
| `voxygen/src/hud/trade.rs` | Add `Quality::Rare` arm to slot image match |
| `voxygen/src/ui/widgets/item_tooltip.rs` | Add `Quality::Rare` arm to slot image match |

> Asset RON files are **not touched** — they were already correct.

---

### Fix 2 — `V5::from_world` dead-code compiler warning

**Root cause:** When `Current` was bumped to `V6`, `V5::from_world` became unreachable but was
not removed.

| File | Change |
|---|---|
| `voxygen/src/singleplayer/singleplayer_world.rs` | Remove the `impl V5 { pub fn from_world(...) }` block only — keep `impl ToWorld for V5` |

---

### Fix 3 — Server GUI not included in release build

**Root cause:** `nova-forge.sh` `cmd_release` and `cmd_build` only compile `nova-forge-server-cli`.
The `server-gui` crate is fully implemented (egui/eframe 1024×700 window with config form, live log
view, player list, graceful shutdown, broadcast, and admin dialogs) but is never built or launched.

| File | Change |
|---|---|
| `nova-forge.sh` `cmd_build` | Add `--bin nova-forge-server-gui` |
| `nova-forge.sh` `cmd_release` | Add `--bin nova-forge-server-gui` + info line |
| `nova-forge.sh` | Add `cmd_server_gui()` function |
| `nova-forge.sh` | Add `server-gui` to case dispatch and interactive menu |
| `nova-forge.sh` `cmd_release` | `mkdir -p target/release/assets/plugins` after asset copy |

---

### Fix 4 — Plugins directory missing at runtime

**Symptom:** `Failed to read plugins from assets: NotFound` in release build logs.  
**Fix:** Ensure `target/release/assets/plugins/` is created during the release asset copy step.

---

## 🟡 Priority 1 — CI / Release Pipeline

### Add release artifact CI job

Currently `.github/workflows/ci.yml` only runs `cargo check`, `cargo test`, and `cargo clippy`.
There is no job that produces downloadable binaries.

**New job (runs on push to `master` only):**
```yaml
release-build:
  runs-on: windows-latest
  steps:
    - cargo build --release --bin nova-forge-server-cli
    - cargo build --release --bin nova-forge-server-gui
    - actions/upload-artifact: nova-forge-server-cli.exe, nova-forge-server-gui.exe
```

---

## 🟢 Priority 2 — Nova-Forge Studio (Visual Asset Editor)

A standalone `egui/eframe` desktop application at `nova-forge-studio/` in the repo root.

**Core principles:**
- **Never touches game source code** — reads and writes `assets/` directory only
- **Excluded from workspace** `Cargo.toml` — does not slow game builds
- **Direct asset editing** — save → file updated → next game launch picks it up
- **Track A / Track B toggle** in top bar — experimental modules locked on Track A
- Launched via `./nova-forge.sh studio`

### Directory structure

```
nova-forge-studio/
├── Cargo.toml           ← standalone crate, NOT in workspace [members]
├── src/
│   ├── main.rs          ← eframe entry point (1024×768 min, resizable)
│   ├── app.rs           ← top bar, tab router, Track A/B toggle, dirty-state tracking
│   ├── asset_io.rs      ← RON read/write helpers, recursive file browser, hot-watch
│   └── modules/
│       ├── worldgen.rs      ← 🌍 World gen editor + preview
│       ├── planet.rs        ← 🪐 [Experimental] Planet / orbital config
│       ├── dungeon.rs       ← 🏰 Prefab room RON editor + tile grid
│       ├── ships.rs         ← 🚀 [Experimental] Ship/station definitions
│       ├── animations.rs    ← 🎬 Animation RON browser + timeline viewer
│       ├── sounds.rs        ← 🔊 Audio asset browser + waveform display
│       ├── items.rs         ← ⚔️  Item RON field editor
│       ├── icons.rs         ← 🖼️  Sprite/icon grid browser
│       └── biomes.rs        ← 🌿 Biome weight & civ spawn tuning
└── preview/
    └── index.html       ← Interactive HTML mockup (zero dependencies)
```

---

### Module: 🌍 Worldgen

The world generation pipeline is a full multi-stage simulation:
```
Seed → Noise heightmap → Mountain uplift → Erosion sim (rivers)
     → Temperature/humidity → Ocean → Biome assignment
     → Civilisation placement → Sites (towns, dungeons) → WorldMapMsg
```

Output is a `WorldMapMsg` containing:
- `rgba` grid — full shaded+topo biome colour map (same as in-game world map)
- `alt` grid — per-chunk f32 heightmap
- `horizons` — hillshading data

#### Phase 1 — 2D Map Preview (achievable immediately)

- Parameter sliders: seed, map size (`map_size_lg` 7=tiny/fast to 10=full), sea level,
  mountain scale, erosion quality, Track A/B toggle
- "▶ Preview" button: spawns background thread calling `World::generate()` at `map_size_lg=7`
  (~30 second gen on modern hardware)
- Progress bar streamed from the world sim's `WorldGenerateStage` events
- Renders the `rgba` grid as an egui texture on completion
- View tabs: **2D Map** | **Heightmap** (greyscale) | **Biomes** | **Sites** (civ overlay)
- "◼ Full Gen" button: uses `map_size_lg=10` for a full bake
- "💾 Export PNG" calls the existing `heightmap_visualization` pipeline

#### Phase 2 — 3D Orbit Camera (wgpu custom render pass)

eframe exposes a `custom_3d_callback` hook into the wgpu render pipeline.

**Implementation:**
- Upload `alt` grid as a f32 heightmap texture to GPU
- Render subdivided plane mesh (512×512 quads)
- Vertex shader displaces Y axis by heightmap sample × scale factor
- Fragment shader samples `rgba` biome texture for colour
- Arcball orbit camera: left-drag = orbit, scroll = zoom, right-drag = pan
- Architecturally identical to `voxygen/src/render/pipelines/lod_terrain.rs` — simplified

**Planet view (Experimental Track B):**
- Sphere mesh with cubemap projection of the biome texture
- Orbital ring placeholder for the space layer

#### Worldgen parameters to expose

| Parameter | Effect |
|---|---|
| `seed` (u32) | Entire world layout |
| `map_size_lg` (7–10) | World size — affects gen time exponentially |
| `mountain_scale` | Overall terrain height multiplier |
| `sea_level` | Ocean coverage fraction |
| `erosion_quality` | Number of erosion passes |
| `tropical_temp` | Biome latitude distribution |
| `experimental_worldgen` | Track A vs Track B pipeline |
| Uplift noise octaves / frequency | Mountain range shapes |

---

### Module: 🏰 Dungeon / Prefab

- Browse `assets/world/manifests/` and `assets/world/site2/` RON files
- Tile grid editor for room layouts
- Experimental Track B: prefab dungeon rooms and boss arenas
- Preview thumbnail rendered from room bounds AABB data

---

### Module: 🚀 Ships & Stations (Experimental Track B only)

This covers the mid-to-late game space layer planned for the experimental lane:

**Progression tiers:**
1. **Starter planet orbital layer** — space stations orbiting the starting world (accessible mid-game)
2. **Mid-game ships** — player-crewable vessels, NPC freighters
3. **Late-game capital ships & stations** — large structures, multi-zone interiors

**Editor capabilities:**
- Ship hull RON definition editor (dimensions, module slots, spawn config)
- Space station orbital config (orbit radius, faction, loot tables)
- Interior zone layout (reuses dungeon prefab module for room graphs)
- Locked with warning banner on Track A

---

### Module: ⚔️ Item Editor

- Full RON field editor for any `.ron` file under `assets/common/items/`
- Quality picker using the correct `Quality` enum (Low → Common → Moderate → High → Rare → Epic → Legendary → Artifact → Debug)
- Tag multi-select, kind picker (Armor/Tool/Consumable/Utility/etc.)
- Ability spec reference picker
- Live preview of item tooltip as rendered in the game HUD

---

### Module: 🎬 Animations

- Browse animation RON/TOML files under `assets/voxygen/`
- Timeline viewer showing keyframe data
- Phase 2: 3D skeleton preview using bone/joint data

---

### Module: 🔊 Sounds

- Asset browser for `assets/voxygen/audio/`
- Play/stop audio files in-editor (using `rodio`/`cpal` — already a game dependency)
- Waveform visualisation
- Spatial and volume parameters editor

---

### Module: 🖼️ Icons / Sprites

- Grid view of all item icons from `assets/voxygen/element/items/`
- Filter by category, search by filename
- Click to reveal source `.png` path in file explorer

---

### Module: 🌿 Biomes

- Visual sliders for biome weights, temperature ranges, humidity bands
- Civilisation spawn probability per biome
- Changes written to `assets/world/manifests/`

---

## 🔵 Priority 3 — ROADMAP.md Updates

The following items need to be added to `ROADMAP.md` to reflect current state and plans:

| Section | Addition |
|---|---|
| Phase 0 | ✅ `server-gui` egui launcher complete (pending build script fix) |
| Phase 1 | ✅ Add `Quality::Rare` enum variant (bug fix, Priority 0) |
| Phase 1 | ✅ Fix `V5::from_world` dead-code warning |
| New Phase — Nova-Forge Studio | Full section covering all modules above |
| Phase 3 | Experimental Lane: planets, space ships, space stations |
| Phase 4 | Studio included in installer package |

---

## Suggested PR Order

| # | PR Title | Scope |
|---|---|---|
| **1** | `fix: Quality::Rare crash + V5 warning + server-gui build` | `common/`, `voxygen/`, `nova-forge.sh` |
| **2** | `ci: add release build job with binary artifacts` | `.github/workflows/ci.yml` |
| **3** | `docs: update ROADMAP + IMPLEMENTATION_PLAN` | `ROADMAP.md`, `IMPLEMENTATION_PLAN.md` |
| **4** | `feat: nova-forge-studio scaffold + Phase 1 worldgen 2D preview` | `nova-forge-studio/` (new crate) |
| **5** | `feat: studio Phase 2 — 3D orbit worldgen preview (wgpu)` | `nova-forge-studio/src/modules/worldgen.rs` |
| **6** | `feat: studio item editor + icon browser` | `nova-forge-studio/src/modules/items.rs`, `icons.rs` |
| **7** | `feat: studio dungeon/prefab editor + sound browser` | `nova-forge-studio/src/modules/dungeon.rs`, `sounds.rs` |
| **8** | `feat: studio experimental modules — ships, stations, planet view` | `nova-forge-studio/src/modules/ships.rs`, `planet.rs` |

---

## Architecture Constraints & Rules

1. **Nova-Forge Studio must never be added to the root `Cargo.toml` `[members]`** — it is a sibling tool, not part of the game workspace.
2. **All game asset changes must go through `assets/`** — Studio never modifies `.rs` source files.
3. **Track B (Experimental) features are gated behind a runtime toggle** — they must not affect Track A gameplay or builds unless explicitly enabled.
4. **Quality enum additions must be backward-compatible** — new variants should be inserted in logical order and all exhaustive match blocks must be updated simultaneously.
5. **Server GUI and Server CLI are separate binaries** — the CLI is headless (for Docker/dedicated servers), the GUI is for local/LAN hosting. Both must be included in release builds.
