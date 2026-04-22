# Atlas Workspace Editor Integration

Nova-Forge includes an integration layer for the **Atlas Workspace** external editor (from the [shifty81/Atlas-Workspace-Rust](https://github.com/shifty81/Atlas-Workspace-Rust) project). The editor opens the game project as a first-class workspace — browsing assets, previewing worlds, and managing project configuration — without needing to build or run the game itself.

---

## Table of Contents

1. [What is Atlas Workspace?](#what-is-atlas-workspace)
2. [Project File — NovaForge.atlas](#project-file--novaforgeatlas)
3. [Prerequisites](#prerequisites)
4. [Opening the Project in Atlas](#opening-the-project-in-atlas)
5. [Project Directory Layout](#project-directory-layout)
6. [Content Browser](#content-browser)
7. [World / Scene View](#world--scene-view)
8. [Registries](#registries)
9. [Configuration Files](#configuration-files)
10. [Capabilities](#capabilities)
11. [Runtime Settings](#runtime-settings)
12. [Extending the Project](#extending-the-project)

---

## What is Atlas Workspace?

Atlas Workspace is a standalone graphical editor designed to work with Nova-Forge game projects. It provides:

- A **Content Browser** for navigating assets
- A **World / Scene view** that loads the default world for a visual preview
- Registry browsers for assets, documents, PCG rules, build targets, and launch targets
- Project-level configuration management (settings, PIE — Play In Editor — settings, PCG preview defaults)

The editor is **external** to the Nova-Forge game binary. It reads project metadata from the `NovaForge.atlas` file at the repository root and accesses data files on disk directly.

---

## Project File — NovaForge.atlas

The file `NovaForge.atlas` in the repository root is the entry point for the Atlas editor. It is a JSON document that describes the project to the editor.

```json
{
  "schema": "atlas.project.v1",
  "projectId": "novaforge",
  "projectName": "NovaForge",
  ...
}
```

**Key fields:**

| Field | Value | Purpose |
|-------|-------|---------|
| `schema` | `atlas.project.v1` | Tells the editor which schema version to use |
| `projectId` | `novaforge` | Unique machine-readable identifier |
| `adapter` | `novaforge` | Selects the Nova-Forge–specific adapter plugin inside Atlas |
| `capabilities` | (list) | Declares which editor modules are available (see [Capabilities](#capabilities)) |
| `roots` | (object) | Maps logical names to physical directory paths |
| `registries` | (object) | Paths to JSON registry files |
| `startup.world` | `Data/Worlds/DefaultWorld.json` | World loaded when the editor opens the scene view |
| `runtime.tickRate` | `30` | Server ticks per second reported to the editor |
| `runtime.maxPlayers` | `64` | Maximum player count reported to the editor |

---

## Prerequisites

1. **Clone the Nova-Forge repository** — the atlas file references paths relative to the repository root.
2. **Install Atlas Workspace** — download and build [Atlas-Workspace-Rust](https://github.com/shifty81/Atlas-Workspace-Rust) following the instructions in that repository's README.
3. Ensure the following directories exist at the repository root (they are created automatically by the game on first run, but can also be created manually):
   - `Content/`
   - `Data/`
   - `Config/`
   - `Schemas/`
   - `Generated/`

---

## Opening the Project in Atlas

1. Launch the **Atlas Workspace** application.
2. Choose **Open Project** from the start screen or the **File** menu.
3. Navigate to the root of your Nova-Forge clone and select **`NovaForge.atlas`**.
4. The editor loads the project. The **Content Browser** populates from `Data/Registries/AssetRegistry.json` and the **Scene View** opens `Data/Worlds/DefaultWorld.json`.

---

## Project Directory Layout

Atlas reads from and writes to the following directories (all paths are relative to the repository root):

```
Nova-Forge/
├── NovaForge.atlas          ← Editor project file
├── Content/                 ← Raw game content (models, textures, audio, etc.)
├── Data/
│   ├── Registries/
│   │   ├── AssetRegistry.json      ← Content Browser source of truth
│   │   ├── DocumentRegistry.json   ← Document assets registry
│   │   ├── SchemaRegistry.json     ← Data schema registry
│   │   ├── PCGRegistry.json        ← Procedural generation rules
│   │   ├── BuildTargetRegistry.json
│   │   └── LaunchTargetRegistry.json
│   └── Worlds/
│       └── DefaultWorld.json       ← Default world (loaded in scene view)
├── Config/
│   ├── WorkspaceProjectSettings.json
│   ├── PIESettings.json            ← Play-In-Editor settings
│   └── PCGPreviewDefaults.json
├── Schemas/                 ← Data validation schemas
└── Generated/               ← Auto-generated files (do not edit manually)
```

> The `.novaforge/` hidden directory is used by the game engine for internal metadata and is not read by the Atlas editor.

---

## Content Browser

The Content Browser in Atlas reads **`Data/Registries/AssetRegistry.json`** to populate its asset tree. Each entry in that file describes an asset with a unique ID, type, name, and file path.

**To add a new asset so it appears in the Content Browser:**

1. Place the asset file under `Content/` (any subdirectory).
2. Register it in `Data/Registries/AssetRegistry.json` following the existing entry format.
3. The Atlas editor will pick up the change the next time you refresh the Content Browser (or reopen the project).

---

## World / Scene View

When Atlas opens a project it loads the **startup world** defined in `NovaForge.atlas`:

```json
"startup": {
  "world": "Data/Worlds/DefaultWorld.json"
}
```

This world replaces the editor's default Earth-globe preview. The JSON world file describes the terrain generator parameters, entity placements, sky settings, and other world-level configuration.

**To change which world the editor opens by default**, update the `startup.world` field in `NovaForge.atlas` to point to a different world file under `Data/Worlds/`.

---

## Registries

Atlas exposes six registry files. Each is a JSON array of typed records:

| Registry file | Purpose |
|--------------|---------|
| `AssetRegistry.json` | All browsable content assets |
| `DocumentRegistry.json` | Text / data document assets |
| `SchemaRegistry.json` | JSON Schema definitions for data validation |
| `PCGRegistry.json` | Procedural-content-generation rule sets |
| `BuildTargetRegistry.json` | Compilation targets (debug, release, etc.) |
| `LaunchTargetRegistry.json` | Run configurations (client, server, headless) |

---

## Configuration Files

| File | Purpose |
|------|---------|
| `Config/WorkspaceProjectSettings.json` | Global editor preferences for this project |
| `Config/PIESettings.json` | Play-In-Editor launch parameters (start map, game mode, etc.) |
| `Config/PCGPreviewDefaults.json` | Default settings for PCG preview windows |

These files are created with defaults the first time Atlas opens the project. You can edit them by hand or through the Atlas **Project Settings** panel.

---

## Capabilities

The `capabilities` array in `NovaForge.atlas` tells the Atlas editor which feature modules to enable:

| Capability | Description |
|-----------|-------------|
| `Rendering3D` | 3D scene viewport |
| `Physics3D` | Physics simulation preview |
| `UI` | UI widget preview |
| `Networking` | Network topology view |
| `AI` | Behaviour-tree and rtsim AI inspector |
| `Animation` | Character and object animation preview |
| `VoxelEditor` | Voxel-level block editing tools |
| `TerrainEditor` | Terrain sculpting and biome painting |
| `Blueprints` | Visual scripting graph editor |

Removing a capability from the list disables the corresponding editor module and hides its panel from the Atlas UI.

---

## Runtime Settings

The `runtime` block in `NovaForge.atlas` provides metadata about the game for Atlas diagnostics and launch configurations:

```json
"runtime": {
  "entryWorld": "Data/Worlds/DefaultWorld.json",
  "tickRate": 30,
  "maxPlayers": 64
}
```

| Field | Description |
|-------|-------------|
| `entryWorld` | The world the game server loads on startup (used by PIE "Launch Server" action) |
| `tickRate` | Server tick rate (ticks per second); used by Atlas performance profiling views |
| `maxPlayers` | Maximum concurrent players; displayed in Atlas server status panels |

---

## Extending the Project

### Adding a new world
1. Create `Data/Worlds/<your_world>.json` following the schema of `DefaultWorld.json`.
2. Register it in `Data/Registries/DocumentRegistry.json` (optional but recommended for the Content Browser).
3. To make it the editor startup world, update `startup.world` in `NovaForge.atlas`.

### Changing the project display name
Edit `projectName` in `NovaForge.atlas`.

### Disabling a capability
Remove the relevant string from the `capabilities` array. For example, to disable the Blueprint editor:

```json
"capabilities": [
  "Rendering3D", "Physics3D", "UI", "Networking",
  "AI", "Animation", "VoxelEditor", "TerrainEditor"
]
```

### Adding a launch target
Add an entry to `Data/Registries/LaunchTargetRegistry.json` describing the executable, arguments, and working directory. Atlas will expose it in the **Launch** dropdown.
