# Nova-Forge In-Game Build Mode

Nova-Forge includes a full **in-game voxel editor** called **Build Mode**. It lets players (and admins) claim rectangular plots of terrain, then freely place and remove individual blocks within that plot using the same tools they use in regular gameplay.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites — Permissions](#prerequisites--permissions)
3. [Accessing Build Mode](#accessing-build-mode)
4. [Build Mode HUD](#build-mode-hud)
5. [Placing Blocks](#placing-blocks)
6. [Removing Blocks](#removing-blocks)
7. [Copying a Block](#copying-a-block)
8. [Block Palette](#block-palette)
9. [Plot System](#plot-system)
   - [Claiming a Plot](#claiming-a-plot)
   - [Releasing a Plot](#releasing-a-plot)
   - [Viewing Plot Info](#viewing-plot-info)
   - [Trusting Other Players](#trusting-other-players)
   - [Admin: Clearing a Plot](#admin-clearing-a-plot)
10. [Map Window — Your Plot Panel](#map-window--your-plot-panel)
11. [Server Settings for Plots](#server-settings-for-plots)
12. [Chat Commands Reference](#chat-commands-reference)
13. [Default Keybindings](#default-keybindings)
14. [Limitations](#limitations)
15. [Troubleshooting](#troubleshooting)

---

## Overview

Build Mode turns Nova-Forge into a voxel editor while you're playing. Once you claim a plot and activate Build Mode:

- You see a **wireframe box** around your claimed plot boundary.
- A **block palette** appears on the HUD showing your selected block type.
- Left-clicking **removes** a highlighted block; right-clicking **places** one.
- Scroll-wheel **cycles** through eight built-in block presets.
- Pressing **Roll (E)** copies the block your cursor is pointing at.

All changes persist to the world immediately and are visible to other players.

---

## Prerequisites — Permissions

Build Mode requires the `CanBuild` component to be attached to your character. This is granted by a server admin using the `/permit_build` command:

```
/permit_build <your_username>
```

Without this component, pressing the Build Mode toggle key does nothing. Admins have this permission automatically.

---

## Accessing Build Mode

| Action | Default Key |
|--------|-------------|
| **Toggle Build Mode on/off** | **F5** |

Press **F5** while controlling your character. The game will:

1. Send a **plot claim request** to the server, reserving a 32 × 32 × 32 cube of terrain centred on your current position (if you don't already have a plot claimed).
2. If the server approves, Build Mode activates: a wireframe boundary box appears around your plot and the block palette HUD is shown.

Press **F5** again to **release your plot** and exit Build Mode.

---

## Build Mode HUD

While Build Mode is active you will see two overlays:

### Build Mode Indicator
A text label at the top-centre of the screen reading **"BUILD MODE"** (or the localised equivalent). This confirms the mode is active.

### Block Palette Panel
A translucent vertical list appears showing all eight block presets. The currently selected entry is highlighted in **gold**. Use the **scroll wheel** to move the highlight up or down.

---

## Placing Blocks

**Right-click** while your crosshair is aimed at a surface within your plot. A new block is placed immediately adjacent to the face your cursor is pointing at, using the currently selected block preset.

> **Note:** Blocks can only be placed within the boundaries of your claimed plot. Attempts to place outside the plot boundary are silently ignored by the server.

---

## Removing Blocks

**Left-click** while your crosshair is aimed at a block within your plot. The targeted block is removed. Standard combat attacks (left-click) are suppressed while the cursor is aimed at a valid build target — the removal action takes priority.

---

## Copying a Block

Press the **Roll key (default: E)** while aiming at any block inside your plot. The block type under the cursor is copied into `selected_block`, automatically updating the palette selection to match (or using the closest preset).

---

## Block Palette

Eight presets are built into the client. Use the **scroll wheel** to cycle through them in either direction:

| Index | Name | Block Kind | Colour |
|-------|------|-----------|--------|
| 0 | **Stone** | Rock | 128, 128, 128 (grey) |
| 1 | **Weak Stone** | WeakRock | 110, 110, 100 (dark grey) |
| 2 | **Earth** | Earth | 102, 70, 40 (brown) |
| 3 | **Sand** | Sand | 210, 190, 130 (tan) |
| 4 | **Wood** | Wood | 120, 70, 30 (dark brown) |
| 5 | **Leaves** | Leaves | 60, 120, 40 (green) |
| 6 | **Snow** | Snow | 230, 235, 240 (light blue) |
| 7 | **White Misc** | Misc | 255, 255, 255 (white) |

Scroll **up** → previous entry (wraps to last). Scroll **down** → next entry (wraps to first).

---

## Plot System

Every player may own **one plot at a time** (configurable by the server admin — see [Server Settings](#server-settings-for-plots)). A plot is a rectangular 3-D area of the world. Blocks inside the plot can only be modified by the plot owner or players the owner has trusted.

### Claiming a Plot

**Via F5 (in-game):**
The game auto-claims a 32 × 32 × 32 block cube centred on your position.

**Via chat command:**
```
/plot_claim [optional_name]
```
The optional name labels the plot (shown in the Map window). If omitted the plot is labelled "Unnamed".

### Releasing a Plot

**Via F5 (in-game):** Press F5 while Build Mode is active.

**Via chat command:**
```
/plot_release
```
The plot is deleted, all trusted players lose access, and the terrain is no longer protected.

### Viewing Plot Info

```
/plot_info
```
Prints the current plot's name, centre coordinates, dimensions, and trusted-player list to your chat.

### Trusting Other Players

You can grant other players permission to build in your plot:

```
/plot_trust <username>
```

To revoke access:

```
/plot_untrust <username>
```

Trusted players do not need their own plot to build inside yours. Trust is restored automatically when the owner reconnects to the server.

### Admin: Clearing a Plot

Admins can forcibly remove another player's plot:

```
/plot_clear <username>
```

This deletes the target player's plot immediately. The player is notified in chat.

---

## Map Window — Your Plot Panel

Open the Map window (**default: M**). At the bottom of the left (quest-log) panel you will find a **"Your Plot"** section showing:

- **Plot name** (or "Unnamed")
- **Centre coordinates** (X, Y, Z)
- **Dimensions** (width × depth × height in blocks)
- **Trusted players** (comma-separated, or "None")

This panel is always visible while the Map is open, even when Build Mode is not currently active, so you can check your plot info without entering build mode.

---

## Server Settings for Plots

Server admins can adjust plot limits in `GameplaySettings` (inside the server's `settings.ron`):

| Setting | Default | Description |
|---------|---------|-------------|
| `max_plot_side` | 32 | Maximum side length of a claimed plot in blocks. Set to 0 for no limit. |
| `max_plots_per_player` | 1 | Maximum number of plots each player may claim simultaneously. |

`effective_max_plot_side()` uses the configured value when positive, or the engine default (`MAX_PLAYER_PLOT_SIDE = 32`) otherwise.

---

## Chat Commands Reference

| Command | Permission | Arguments | Description |
|---------|-----------|-----------|-------------|
| `/plot_claim` | Any (needs `CanBuild`) | `[name]` | Claim a plot centred on your position |
| `/plot_release` | Plot owner | — | Release your current plot |
| `/plot_info` | Plot owner | — | Print plot name, coords, size, trusted list |
| `/plot_trust` | Plot owner | `<username>` | Grant a player build access to your plot |
| `/plot_untrust` | Plot owner | `<username>` | Revoke a player's build access |
| `/plot_clear` | **Admin** | `<username>` | Forcibly delete another player's plot |
| `/permit_build` | **Admin** | `<username>` | Grant the `CanBuild` component (required to use Build Mode at all) |

---

## Default Keybindings

| Key | Action |
|-----|--------|
| **F5** | Toggle Build Mode (claim/release plot) |
| **Right-click** | Place block at cursor (Build Mode only) |
| **Left-click** | Remove block at cursor (Build Mode only) |
| **Scroll up** | Previous block preset |
| **Scroll down** | Next block preset |
| **E** (Roll) | Copy block under cursor into selected slot |
| **M** | Open Map window (view Your Plot panel) |

> Keybindings can be remapped in **Settings → Controls**.

---

## Limitations

- Only **one plot per player** is allowed by default (configurable).
- Plot maximum side length is **32 blocks** by default (configurable).
- Plots cannot overlap. If the area you are trying to claim conflicts with an existing plot, the server will reject the claim and display an error message.
- Blocks can only be placed or removed **within your plot boundary**. You cannot modify the world outside your plot even while Build Mode is active.
- Build Mode is purely block-based — it does not support entities, items, or scripting.

---

## Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|-------------|-----|
| F5 does nothing | Missing `CanBuild` component | Ask an admin to run `/permit_build <you>` |
| "Area too large" error | Plot side > `max_plot_side` | Move to a flat area or ask admin to increase `max_plot_side` |
| "Overlaps existing" error | Another plot is already in that location | Move to a different area and try again |
| Cannot place blocks | Cursor is aimed outside plot boundary | Aim within the wireframe box |
| Plot info not in Map | No plot currently claimed | Claim a plot first with F5 or `/plot_claim` |
