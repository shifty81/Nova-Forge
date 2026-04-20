# Nova-Forge Roadmap

This document tracks the planned development of Nova-Forge across milestone phases.
Checked items are complete (or substantially complete); unchecked items are planned or in progress.

---

## Phase 0 — Foundation (Completed / Ongoing)

- [x] Fork Veloren, rename binaries to `nova-forge-voxygen` / `nova-forge-server-cli`
- [x] Strip mandatory authentication (stub auth layer, allow any username)
- [x] LAN discovery and auto-connect
- [x] `nova-forge.sh` build/run/release helper script
- [x] Nix flake for reproducible builds
- [x] Persistent singleplayer world save management UI *(COMPLETE)*

---

## Phase 1 — Core Nova-Forge Modifications

Items that modify or extend existing base engine systems.

### GUI Scale slider for large/HiDPI monitors *(partially complete)*

The current absolute-scale slider was capped at 2.0×, making icons and text very small on 4K or ultrawide monitors.

- [x] Extend slider range from 2.0× to 4.0× for large monitor users
- [x] Update the dropdown preset list to include 2.5, 3.0, 3.5, 4.0
- [x] Add a "DPI-aware auto" default that detects monitor DPI and sets a sensible scale *(COMPLETE)*

### Singleplayer world management *(COMPLETE)*

- [x] List, create, rename, and delete singleplayer worlds from the main menu
- [x] Per-world settings: seed, day length, PvP/PvE combat mode
- [x] Per-world difficulty scaling (enemy HP / damage multipliers) *(Easy/Normal/Hard)*

### LAN server UX improvements *(COMPLETE)*

- [x] Show LAN server version and player count in the browser list
- [x] Connection status feedback during discovery

### Settings persistence *(COMPLETE)*

- [x] All Nova-Forge-specific settings (GUI scale, networking preferences) saved to `settings.ron`
- [x] Per-world gameplay settings (seed, day length, PvP mode, max players) persisted in each world's `meta.ron`

---

## Phase 2 — Player Housing System

**Status: Substantially Complete — remaining items below**

Player housing is a major new system. The following design decisions have been resolved during implementation:

1. **Plot ownership model** — Proximity claim via `/plot_claim` (or build-mode toggle key B). Max plot side and max plots per player are configurable in server `settings.ron`.
2. **Plot boundaries** — Fixed-size AABB rectangular region. Boundaries visualised in-world via a gold WireBox overlay (DebugShape::WireBox).
3. **Persistence backend** — `PlayerPlot` ECS component on the server + `PlayerBuildArea` area container. Plot data persisted via `state_ext` on character save/login.
4. **Build permissions system** — Owner + trusted-player list. Trust managed via `/plot_trust` and `/plot_untrust`.
5. **Furniture / decoration system** — Pending (see tasks below).
6. **Economy integration** — Housing is free; no currency system.
7. **Server configuration** — `GameplaySettings` in `settings.ron` has `max_plot_side` and `max_plots_per_player`.
8. **World zones** — Players can build anywhere they claim.
9. **Migration / import** — Pending.
10. **Multiplayer sync** — Block-place events validated server-side via `CanBuild` + `PlayerBuildArea`.

### Implementation tasks

- [x] `PlotClaim` server-side component and storage
- [x] `/plot_claim`, `/plot_release`, `/plot_info`, `/plot_clear` (admin) chat commands
- [x] `/plot_trust <player>` and `/plot_untrust <player>` — grant/revoke per-player build access
- [x] Plot boundary visualisation (WireBox overlay via DebugShape)
- [x] Build-mode UI (toggle, block palette scroll-wheel selection)
- [x] Housing tab in the map window showing owned plot (name, center, size, trusted players)
- [ ] Build-mode undo/redo stack
- [ ] Furniture entity type + placement UI
- [ ] Server admin panel UI entries for housing config (in-game settings window)

---

## Phase 3 — Gameplay Extensions

- Custom skill trees / talent system (Nova-Forge exclusive skills)
- Seasonal events calendar
- Extended crafting: player-made blueprints

---

## Phase 4 — Polish & Release

- Installer / launcher (Windows `.msi`, Linux AppImage, macOS `.dmg`)
- Auto-update mechanism for the launcher
- Public server listing (opt-in)
- Full localisation pass for all Nova-Forge-specific UI strings
