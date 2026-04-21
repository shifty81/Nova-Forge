# Nova-Forge — MagicVoxel Asset Guide

This document explains how every `.vox` file in the project is used, where it lives,
how the game loads it, and exactly what you need to do to add or modify assets using
[MagicVoxel](https://ephtracy.github.io/).

---

## Table of Contents

1. [How the Asset System Works](#1-how-the-asset-system-works)
2. [Asset Root Layout](#2-asset-root-layout)
3. [Voxel Category Reference](#3-voxel-category-reference)
   - [3.1 Armor — `assets/voxygen/voxel/armor/`](#31-armor)
   - [3.2 Figures (player body parts) — `assets/voxygen/voxel/figure/`](#32-figures-player-body-parts)
   - [3.3 NPCs / Creatures — `assets/voxygen/voxel/npc/`](#33-npcs--creatures)
   - [3.4 Weapons — `assets/voxygen/voxel/weapon/`](#34-weapons)
   - [3.5 Items (inventory icons) — `assets/voxygen/voxel/item/`](#35-items-inventory-icons)
   - [3.6 Sprites (world objects) — `assets/voxygen/voxel/sprite/`](#36-sprites-world-objects)
   - [3.7 Objects (placed entities) — `assets/voxygen/voxel/object/`](#37-objects-placed-entities)
   - [3.8 Gliders — `assets/voxygen/voxel/glider/`](#38-gliders)
   - [3.9 Lanterns — `assets/voxygen/voxel/lantern/`](#39-lanterns)
   - [3.10 World Structures — `assets/world/structure/`](#310-world-structures)
4. [Manifest Files — The Bridge Between Code and Art](#4-manifest-files--the-bridge-between-code-and-art)
   - [4.1 Armor manifests](#41-armor-manifests)
   - [4.2 Weapon manifest](#42-weapon-manifest)
   - [4.3 NPC / creature manifests](#43-npc--creature-manifests)
   - [4.4 Head & figure manifests](#44-head--figure-manifests)
5. [Item `.ron` Files](#5-item-ron-files)
6. [Colour & Grayscale Tinting](#6-colour--grayscale-tinting)
7. [Step-by-Step Workflows](#7-step-by-step-workflows)
   - [7.1 Modify an existing armor piece](#71-modify-an-existing-armor-piece)
   - [7.2 Add a brand-new armor set](#72-add-a-brand-new-armor-set)
   - [7.3 Add a new weapon skin](#73-add-a-new-weapon-skin)
   - [7.4 Add a new NPC / creature](#74-add-a-new-npc--creature)
   - [7.5 Add a new inventory item](#75-add-a-new-inventory-item)
   - [7.6 Add a new world sprite](#76-add-a-new-world-sprite)
   - [7.7 Add a world structure (dungeon prefab)](#77-add-a-world-structure-dungeon-prefab)
8. [Offset Tuning (the three numbers after the path)](#8-offset-tuning-the-three-numbers-after-the-path)
9. [MagicVoxel Tips Specific to Nova-Forge](#9-magicvoxel-tips-specific-to-nova-forge)
10. [Common Mistakes & Fixes](#10-common-mistakes--fixes)

---

## 1. How the Asset System Works

Nova-Forge uses the **`assets`** crate (part of the Veloren heritage) to load files at
runtime.  A dotted specifier like `"voxygen.voxel.armor.boreal.chest"` maps directly to
the file path `assets/voxygen/voxel/armor/boreal/chest.vox`.  The rules are:

| Specifier segment | Meaning |
|---|---|
| `voxygen` | top-level folder `assets/voxygen/` |
| `.voxel.` | sub-folder `voxel/` |
| rest of path | remaining folders + file name, dots → slashes |
| implicit `.vox` | extension is always `.vox` for voxel meshes |

The engine **hot-reloads** assets when running in debug mode — edit and save a `.vox`
file and the change appears in-game within seconds without restarting the server.

---

## 2. Asset Root Layout

```
assets/
├── common/           # Item definitions (.ron), recipes, abilities, loot tables
│   └── items/
│       ├── armor/    # Armor item stats and metadata (.ron)
│       └── weapons/  # Weapon item stats and metadata (.ron)
├── server/           # Server-side config (spawn tables, economy, etc.)
├── voxygen/          # Client-side visuals
│   └── voxel/        # ALL 3-D mesh files (.vox) + manifest files (.ron)
│       ├── armor/    # Wearable armor pieces
│       ├── figure/   # Humanoid character body parts (body, hair, beard, eyes, accessories)
│       ├── glider/   # Glider meshes
│       ├── item/     # Item pickup / inventory 3-D icons
│       ├── lantern/  # Held lanterns
│       ├── npc/      # All creatures and NPCs, broken into body segments
│       ├── object/   # Turret-style placed objects (crossbow, haniwa_sentry, …)
│       ├── sprite/   # Single-block-scale world decorations (plants, chests, doors, …)
│       └── weapon/   # Weapon meshes held in the player's hand
└── world/
    └── structure/    # Dungeon/building prefab volumes (.vox)
        ├── dwarven_quarry/prefab/
        ├── jungle_ruin/
        ├── plots/
        ├── spots/
        ├── natural/
        └── terracotta/decor/
```

---

## 3. Voxel Category Reference

### 3.1 Armor

**Path:** `assets/voxygen/voxel/armor/<set>/<piece>.vox`

Each armor set has a sub-folder.  Piece names correspond to the body slot:

| File name pattern | Body slot |
|---|---|
| `chest.vox` | Chest / torso |
| `belt.vox` | Belt |
| `shoulder.vox` (or `shoulder_1/2`) | Pauldrons — often one file that is mirrored |
| `hand.vox` | Gloves |
| `foot.vox` | Boots |
| `back.vox` | Cape / back attachment |
| `pants.vox` | Legs |
| `head.vox` | Helmet |
| `neck.vox` | Neck / necklace |

**Example tree for the `boreal` set:**
```
assets/voxygen/voxel/armor/boreal/
├── belt.vox
├── chest.vox
├── foot.vox
├── hand.vox
├── pants.vox
└── shoulder.vox
```

The mesh for `misc/chest/` contains generic slot placeholders (e.g. `grayscale.vox`,
`none.vox`) used when no specific armor is equipped or when a recolour-only variant is
needed (see [Section 6](#6-colour--grayscale-tinting)).

---

### 3.2 Figures (player body parts)

**Path:** `assets/voxygen/voxel/figure/`

These files define the **base humanoid skeleton** shared by all playable races.

```
figure/
├── body/        # Torso, belt, pants, hands, feet — the base mesh worn under armor
├── head/        # One file per race × gender:  human/male.vox, elf/female.vox, …
├── hair/        # hair/<race>/<gender>-<index>.vox  (0-indexed, None = bald)
├── beard/       # beard/<race>/<variant>-<index>.vox
├── eyes/        # eyes/<race-or-general>/<variant>.vox
└── accessory/   # Race-specific extras:  orc teeth/warpaint, danari horns, dwarf earrings, elf headbands
```

Supported races: `human`, `orc`, `danari`, `draugr`, `dwarf`, `elf`.

---

### 3.3 NPCs / Creatures

**Path:** `assets/voxygen/voxel/npc/<creature_name>/`

Every creature is made up of multiple body segment files.  The exact segments vary by
skeleton type:

| Skeleton type | Typical segments |
|---|---|
| Quadruped medium (wolf, bear, horse, …) | `head`, `jaw`, `neck`, `torso_front`, `torso_back`, `ears`, `tail`, `leg_fl`, `leg_fr`, `leg_bl`, `leg_br`, `foot_fl`, `foot_fr`, `foot_bl`, `foot_br` |
| Quadruped small | `head`, `chest`, `leg_fl`, `leg_fr`, `leg_bl`, `leg_br` |
| Biped large (trolls, giants) | `head`, `jaw`, `upper_torso`, `lower_torso`, `shoulder_l`, `shoulder_r`, `hand_l`, `hand_r`, `leg_l`, `leg_r`, `foot_l`, `foot_r`, `tail` |
| Bird medium | `head`, `torso`, `tail`, `wing_l`, `wing_r`, `leg_l`, `leg_r`, `foot_l`, `foot_r` |
| Fish | `head`, `torso`, `tail`, `fin_l`, `fin_r` |

Most species also have a `male/` sub-folder (and sometimes `female/`).

```
npc/wolf/male/
├── ears.vox
├── foot_br.vox
├── foot_fr.vox
├── head.vox
├── jaw.vox
├── leg_br.vox
├── leg_fr.vox
├── neck.vox
├── tail.vox
├── torso_back.vox
└── torso_front.vox
```

---

### 3.4 Weapons

**Path:** `assets/voxygen/voxel/weapon/<type>/`

Types: `axe`, `bow`, `dagger`, `hammer`, `sceptre`, `shield`, `spear`, `staff`, `sword`,
`tool`, `component`, `projectile`, `biped_small`.

Modular weapons (the crafting system) use a sub-folder per style:

```
weapon/sword/longsword/
├── bronze-1h.vox
├── bronze-2h.vox
├── steel-1h.vox
└── steel-2h.vox
```

Non-modular (boss drops, starters) sit directly in the type folder:
```
weapon/sword/starter.vox
weapon/sword/sword_gigas_fire.vox
```

---

### 3.5 Items (inventory icons)

**Path:** `assets/voxygen/voxel/item/<category>/`

These are the small 3-D models shown in the inventory grid and on the ground when dropped.

| Sub-folder | Contents |
|---|---|
| `consumable/` | Potions, food |
| `crafting/` | Crafting materials (cloth, leather, etc.) |
| `crafting_tool/` | Hammers, anvils shown as craftable items |
| `flowers/` | Decorative flowers |
| `food/` | Cooked meals |
| `key/` | Keys |
| `mineral/gem/` | Gems (amethyst, ruby, …) |
| `mineral/ore/` | Ores (iron, silver, coal, …) |
| `recipe/` | Learnable recipe scrolls |
| `utility/` | Misc utility items |
| `wood/` | Wood types |
| `charms/` | Charm items |

---

### 3.6 Sprites (world objects)

**Path:** `assets/voxygen/voxel/sprite/<category>/`

Sprites are placed voxel objects in the world — plants, chests, doors, furniture, etc.
They are referenced from the world generation code and terrain layer files.

Notable sub-folders:

| Folder | Examples |
|---|---|
| `chests/` | `chest.vox`, `chest_gold.vox`, `chest_demon.vox` |
| `crafting_station/` | Anvil, forge, cooking pot |
| `door/` | Wooden doors, iron gates |
| `flowers/` | Rose, sunflower, … |
| `furniture/` | Bed, chair, table, bookshelf |
| `mineral/` | Ore vein sprites placed in caves |
| `mushrooms/` | Various mushroom types |
| `grass/` | Ground grass variants |
| `crystal/` | Crystal formations |
| `lantern/` | Lantern posts |
| `sign/` | Signs |
| `window/` | Window panes |

Multiple variants for the same sprite type are numbered `0.vox`, `1.vox`, `2.vox`, …

---

### 3.7 Objects (placed entities)

**Path:** `assets/voxygen/voxel/object/<name>/`

These are entities placed in dungeons that behave more like actors than terrain
(crossbows, the Haniwa Sentry, the flamethrower, etc.).

---

### 3.8 Gliders

**Path:** `assets/voxygen/voxel/glider/`

One `.vox` file per glider skin.  Gliders are single-mesh items worn on the back when
deployed.

---

### 3.9 Lanterns

**Path:** `assets/voxygen/voxel/lantern/`

Hand-held lanterns.  One `.vox` file per lantern type.  Referenced by the lantern
manifest (`humanoid_lantern_manifest.ron`).

---

### 3.10 World Structures

**Path:** `assets/world/structure/<dungeon>/`

Prefab rooms and buildings used by the world generator.  Each `.vox` file is an entire
room or section:

```
assets/world/structure/dwarven_quarry/prefab/
├── dwarven_quarry-0-hallway2.vox
├── dwarven_quarry-1-after_forgemaster.vox
├── dwarven_quarry-2-hallway0.vox
├── dwarven_quarry-3-smelting_room.vox
├── dwarven_quarry-4-cleansing_room.vox
├── dwarven_quarry-5-excavation_site.vox
├── dwarven_quarry-6-forgemaster_room.vox
├── dwarven_quarry-7-entrance.vox
├── dwarven_quarry-7-mining_site.vox
├── dwarven_quarry-8-hallway1.vox
└── forgemaster_boss.vox
```

Structure `.vox` files can use the **full MagicVoxel colour palette** — the engine maps
palette indices to block types.  When adding new dungeon rooms, follow the existing
palette conventions in nearby structures so blocks render correctly.

---

## 4. Manifest Files — The Bridge Between Code and Art

Manifests are `.ron` files (Rusty Object Notation) that tell the game which `.vox` file
to load for each in-game item or creature and how to position/offset the mesh.

All manifests live in `assets/voxygen/voxel/` alongside the mesh folders.

### 4.1 Armor manifests

| Manifest file | Controls |
|---|---|
| `humanoid_armor_chest_manifest.ron` | Chest pieces |
| `humanoid_armor_shoulder_manifest.ron` | Shoulder / pauldron pieces |
| `humanoid_armor_hand_manifest.ron` | Gloves |
| `humanoid_armor_belt_manifest.ron` | Belts |
| `humanoid_armor_pants_manifest.ron` | Leg armor |
| `humanoid_armor_foot_manifest.ron` | Boots |
| `humanoid_armor_back_manifest.ron` | Capes / back pieces |
| `humanoid_armor_head_manifest.ron` | Helmets |

**Chest manifest entry format:**
```ron
"common.items.armor.boreal.chest": (
    vox_spec: ("armor.boreal.chest", (-7.0, -3.5, 2.0)),
    color: None
),
```

* The map key is the item's asset specifier (matching its `.ron` item-def file under `assets/common/`).
* `vox_spec` is a tuple of `(specifier_string, (x_offset, y_offset, z_offset))`.
* The specifier resolves to `assets/voxygen/voxel/armor/boreal/chest.vox`.
* `color` is `None` for items with their own palette, or `Some((r, g, b))` when the mesh
  is a greyscale template to be tinted (see [Section 6](#6-colour--grayscale-tinting)).

**Shoulder manifest entry format** (two-part because pauldrons are mirrored):
```ron
"common.items.armor.assassin.shoulder": (
    left: (
        vox_spec: ("armor.assassin.shoulder", (-5.0, -3.5, 0.0)),
        color: None
    ),
    right: (
        vox_spec: ("armor.assassin.shoulder", (-1.0, -3.5, 0.0)),
        color: None
    )
),
```

### 4.2 Weapon manifest

**File:** `assets/voxygen/voxel/biped_weapon_manifest.ron`

Maps a modular weapon component combination (primary component specifier + material
specifier + handedness) to a vox file:

```ron
Modular(("common.items.modular.weapon.primary.sword.longsword",
         "common.items.mineral.ingot.bronze",
         Two)): (
    vox_spec: ("weapon.sword.longsword.bronze-2h", (-1.5, -3.5, -5.0)),
    color: None
),
```

For non-modular weapons (fixed items), the key is `Item("common.items.weapons.sword.my_sword")`.

### 4.3 NPC / creature manifests

Each skeleton type has its own pair of manifests: a **central** manifest for the torso
and a **lateral** manifest for limbs.

| File | Covers |
|---|---|
| `quadruped_medium_lateral_manifest.ron` | Medium 4-legged animal limbs |
| `quadruped_small_lateral_manifest.ron` | Small 4-legged animal limbs |
| `quadruped_low_central_manifest.ron` | Low-body reptiles & crawlers |
| `bird_medium_central_manifest.ron` | Medium birds torso |
| `bird_medium_lateral_manifest.ron` | Medium birds wings/legs |
| `golem_central_manifest.ron` | Golems torso |
| `golem_lateral_manifest.ron` | Golems limbs |
| `theropod_central_manifest.ron` | Theropods torso |
| `arthropod_lateral_manifest.ron` | Arthropods limbs |
| `fish_medium_lateral_manifest.ron` | Medium fish fins/tail |
| `fish_small_lateral_manifest.ron` | Small fish fins/tail |
| `crustacean_lateral_manifest.ron` | Crustacean claws/legs |

Each entry maps `(SpeciesVariant, GenderVariant)` to a struct containing one entry per
body segment:

```ron
(Grolgar, Male): (
    leg_fl: (
        offset: (-2.5, -4.5, -4.0),
        lateral: ("npc.grolgar.male.leg_fr"),
    ),
    …
),
```

### 4.4 Head & figure manifests

**File:** `assets/voxygen/voxel/humanoid_head_manifest.ron`

Maps `(Race, BodyType)` to all the character creation options — head mesh, eye variants,
hair variants, beard variants, and accessories, each as a list of `Option<(specifier, offset)>`.
`None` in a list means "no hair" (bald).

```ron
(Human, Male): (
    offset: (-7.0, -2.5, -2.0),
    head: ("figure.head.human.male", (0, 2, 0)),
    hair: [
        None,
        Some(("figure.hair.human.male-0", (1, 1, 1))),
        Some(("figure.hair.human.male-1", (2, 1, 0))),
        …
    ],
    beard: [ … ],
    eyes: [ … ],
    accessory: [ None ],
),
```

---

## 5. Item `.ron` Files

Every item that can appear in the inventory needs two things:

1. **An item-definition `.ron` file** under `assets/common/items/`.
2. **An entry in the relevant manifest** (for armor/weapons) or **a `.vox` file at the
   expected path** (for items, sprites, etc.).

A minimal armor item definition:
```ron
// assets/common/items/armor/mymod/chest.ron
ItemDef(
    legacy_name: "My Custom Chest",
    legacy_description: "Looks really cool.",
    kind: Armor((
        kind: Chest,
        stats: FromSet("MyMod"),   // or explicit stats
    )),
    quality: Common,    // Mundane / Common / Moderate / High / Epic / Legendary / Artifact / Debug
    tags: [],
)
```

The item asset specifier is built from its file path:
`assets/common/items/armor/mymod/chest.ron` → `"common.items.armor.mymod.chest"`.

Additionally, every item requires entries in:
- `assets/common/item_i18n_manifest.ron` — links the specifier to a localisation key.
- `assets/voxygen/i18n/en/item/items/crafting.ftl` (or another `.ftl`) — provides the
  display name and description text.

The CI will fail the `ensure_item_localization` test if either entry is missing.

---

## 6. Colour & Grayscale Tinting

When you want multiple colour variants of the same mesh shape, you can:

1. Create a **grayscale `.vox`** file where all voxels use shades of grey.
2. In the manifest, point multiple entries at the same file with different `color` values:

```ron
"Blue": (
    vox_spec: ("armor.misc.chest.grayscale", (-7.0, -3.5, 2.0)),
    color: Some((44, 74, 109))
),
"Brown": (
    vox_spec: ("armor.misc.chest.grayscale", (-7.0, -3.5, 2.0)),
    color: Some((90, 49, 43))
),
```

The engine multiplies each voxel's grey value by the tint colour at runtime.  This is
the same mesh file, zero extra storage, infinite colour variants.

For a fully custom-coloured piece, set `color: None` and bake the colours into the
`.vox` palette directly.

---

## 7. Step-by-Step Workflows

### 7.1 Modify an existing armor piece

1. Open `assets/voxygen/voxel/armor/<set>/<piece>.vox` in MagicVoxel.
2. Edit voxels and/or palette.
3. **Save** (`Ctrl+S`) — overwrite the same file.
4. If running a debug build the change hot-reloads immediately; otherwise rebuild.
5. No manifest or `.ron` changes are needed — the file is already registered.

---

### 7.2 Add a brand-new armor set

**Example: add a `"mythril"` set with chest, shoulder, and boots.**

**Step 1 — Create the `.vox` meshes**

In MagicVoxel, design three models and save them:
```
assets/voxygen/voxel/armor/mythril/chest.vox
assets/voxygen/voxel/armor/mythril/shoulder.vox
assets/voxygen/voxel/armor/mythril/foot.vox
```

**Step 2 — Add item `.ron` definitions**

Create one `.ron` file per slot under `assets/common/items/armor/mythril/`:
```ron
// chest.ron
ItemDef(
    legacy_name: "Mythril Chestplate",
    legacy_description: "Forged from pure mythril ore.",
    kind: Armor((kind: Chest, stats: FromSet("Mythril"))),
    quality: High,
    tags: [],
)
```

Repeat for `shoulder.ron` (`kind: Shoulder`) and `foot.ron` (`kind: Foot`).

**Step 3 — Add localisation entries**

In `assets/common/item_i18n_manifest.ron` add lines like:
```ron
"common.items.armor.mythril.chest": "item-mythril_chest",
"common.items.armor.mythril.shoulder": "item-mythril_shoulder",
"common.items.armor.mythril.foot": "item-mythril_foot",
```

In `assets/voxygen/i18n/en/item/items/crafting.ftl` (or a new `.ftl` in the same
directory) add:
```ftl
item-mythril_chest =
    .name = Mythril Chestplate
    .desc = Forged from pure mythril ore.
item-mythril_shoulder =
    .name = Mythril Pauldrons
    .desc = Protect your shoulders in mythril glory.
item-mythril_foot =
    .name = Mythril Boots
    .desc = Heavy, but worth every step.
```

**Step 4 — Register meshes in manifests**

Open `assets/voxygen/voxel/humanoid_armor_chest_manifest.ron` and add inside the `map`:
```ron
"common.items.armor.mythril.chest": (
    vox_spec: ("armor.mythril.chest", (-7.0, -3.5, 2.0)),
    color: None
),
```

Open `humanoid_armor_shoulder_manifest.ron` and add:
```ron
"common.items.armor.mythril.shoulder": (
    left: (
        vox_spec: ("armor.mythril.shoulder", (-5.0, -3.5, 0.0)),
        color: None
    ),
    right: (
        vox_spec: ("armor.mythril.shoulder", (-1.0, -3.5, 0.0)),
        color: None
    )
),
```

Open `humanoid_armor_foot_manifest.ron` and add:
```ron
"common.items.armor.mythril.foot": (
    vox_spec: ("armor.mythril.foot", (-3.0, -4.0, -2.0)),
    color: None
),
```

**Step 5 — (Optional) Add to loot tables / recipes**

Add the items to the relevant loot tables under `assets/common/loot_tables/` and/or
crafting recipes under `assets/common/recipe_book.ron`.

**Step 6 — Test**

Run `cargo check` and then start the game.  Use `/give_item common.items.armor.mythril.chest 1`
in-game to test.

---

### 7.3 Add a new weapon skin

**Example: a special `"thunderblade"` sword.**

1. Create `assets/voxygen/voxel/weapon/sword/thunderblade.vox` in MagicVoxel.

2. Create `assets/common/items/weapons/sword/thunderblade.ron`:
```ron
ItemDef(
    legacy_name: "Thunderblade",
    legacy_description: "Crackles with static electricity.",
    kind: Tool((
        kind: Sword,
        hands: Two,
        stats: (
            equip_time_secs: 0.3,
            power: 1.4,
            effect_power: 1.2,
            speed: 1.1,
            range: 1.0,
            energy_efficiency: 1.0,
            buff_strength: 1.0,
        ),
    )),
    quality: Epic,
    tags: [],
    ability_spec: None,
)
```

3. Register localisation (same process as armor above).

4. Open `assets/voxygen/voxel/biped_weapon_manifest.ron` and add an `Item(...)` entry:
```ron
Item("common.items.weapons.sword.thunderblade"): (
    vox_spec: ("weapon.sword.thunderblade", (-1.5, -4.0, -5.0)),
    color: None
),
```

5. Check and test.

---

### 7.4 Add a new NPC / creature

This is the most involved workflow because creatures have multiple body segments and
require manifest entries for each.

**Example: add a `"shadowcat"` as a new quadruped medium.**

1. **Create segment `.vox` files** for every required body part in
   `assets/voxygen/voxel/npc/shadowcat/male/`:
   `head.vox`, `jaw.vox`, `neck.vox`, `torso_front.vox`, `torso_back.vox`,
   `tail.vox`, `ears.vox`, `leg_fr.vox`, `leg_br.vox`, `foot_fr.vox`, `foot_br.vox`.

2. **Add entries to the central and lateral manifests** for `QuadrupedMedium`:
   - `quadruped_medium_lateral_manifest.ron` — limbs (legs, feet, ears, jaw, tail).
   - The central manifest for the torso, neck, head.

3. **Register the species** in `common/src/comp/body/quadruped_medium.rs`
   (add `Shadowcat` to the enum and implement traits).

4. **Add spawn rules** to loot tables / npc spawn tables.

For a full creature, also check `assets/common/abilities/` for any special attacks and
`voxygen/src/scene/figure/load.rs` where the species enum is matched.

---

### 7.5 Add a new inventory item

**Example: a `"mystic_orb"` crafting material.**

1. Create `assets/voxygen/voxel/item/crafting/mystic_orb.vox` — a small (~6×6×6)
   voxel model for the inventory icon and ground drop.

2. Create `assets/common/items/crafting_ing/mystic_orb.ron`:
```ron
ItemDef(
    legacy_name: "Mystic Orb",
    legacy_description: "Glows with an inner light.",
    kind: Ingredient(("mystic_orb")),
    quality: High,
    tags: [],
)
```

3. Register the specifier in `assets/common/item_i18n_manifest.ron` and add an `.ftl`
   localisation entry.

4. The `.vox` file is loaded automatically based on the item specifier path — **no
   manifest entry needed** for ingredient items.  The engine looks for
   `assets/voxygen/voxel/item/crafting/mystic_orb.vox` via the item specifier
   `common.items.crafting_ing.mystic_orb`.

---

### 7.6 Add a new world sprite

1. Create `assets/voxygen/voxel/sprite/<category>/mysprite.vox` — keep it small; sprites
   should fit within roughly a 1–3 block footprint.

2. Register the sprite type in the engine-side sprite enum
   (`common/src/terrain/sprite.rs`) and add world-gen placement logic.

3. Add loot/harvest data if the sprite is harvestable (in `common/src/terrain/sprite.rs`
   and loot tables).

---

### 7.7 Add a world structure (dungeon prefab)

1. Build your room/corridor in MagicVoxel.  Use the **same palette conventions** as
   existing structures in `assets/world/structure/` — palette index determines block type.

2. Save the file as `assets/world/structure/<dungeon_name>/<room_name>.vox`.

3. Register it in the world generation code
   (`world/src/site/dungeon/` or relevant site module) by adding the file path to the
   structure list.

---

## 8. Offset Tuning (the three numbers after the path)

Every `vox_spec` tuple contains three float offsets: `(x, y, z)`.  These control where
the mesh's **origin point** is attached to the character's skeleton joint.

- **Negative x** pushes the model left; positive pushes right.
- **Negative y** pushes the model forward (toward screen); positive pushes back.
- **Negative z** pushes the model down; positive pushes up.

**Quick tuning workflow:**

1. Run the game in debug mode (hot-reload enabled).
2. Equip the item.
3. Edit the offset values in the manifest `.ron` file and save.
4. The mesh repositions immediately in-game — no restart needed.
5. Iterate until the attachment point looks correct from all camera angles.

**Typical starting offsets by slot:**

| Slot | Typical range |
|---|---|
| Chest | `(-7.0, -3.5, 2.0)` |
| Shoulder (left) | `(-5.0, -3.5, 0.0)` |
| Shoulder (right) | `(-1.0, -3.5, 0.0)` |
| Belt | `(-5.0, -3.5, 0.0)` |
| Pants | `(-7.0, -3.5, 1.0)` |
| Boots | `(-3.0, -3.5, -9.0)` |
| Gloves | `(-3.0, -2.0, -1.0)` |
| Helmet | `(-8.0, -5.5, 1.0)` |
| Two-handed weapon | `(-1.5, -4.0, -5.0)` |
| One-handed weapon | `(-0.5, -3.0, -4.0)` |

These are starting points; every mesh will need fine-tuning.

---

## 9. MagicVoxel Tips Specific to Nova-Forge

### Model size conventions

| Asset type | Recommended dimensions |
|---|---|
| Chest armor | ~14×7×10 voxels |
| Helmet | ~16×16×14 voxels |
| Boots | ~8×10×7 voxels |
| One-handed sword | ~2×2×20 voxels |
| Two-handed sword | ~2×2×30 voxels |
| Item icon | ~6×6×6 voxels |
| World sprite (plant) | ~5×5×12 voxels |
| World sprite (chest) | ~10×8×8 voxels |
| NPC head (medium quad) | ~10×12×8 voxels |

### Palette usage

- **Armor with `color: None`** — use the full palette freely; your chosen colours appear
  exactly in-game.
- **Grayscale / tinted armor** — keep all voxels in shades of grey (R = G = B); the
  manifest `color` tint multiplies against these values.
- **World structures** — palette indices map to specific terrain block types defined in
  the engine.  Match the palette of an existing structure in the same dungeon/site.

### MagicVoxel export settings

- Save as **MagicVoxel `.vox` format** (default).  Do NOT export as OBJ or other formats.
- Nova-Forge reads `.vox` version 150 (the current MagicVoxel format) — all current
  versions of MagicVoxel write compatible files.
- The game only reads the **first model** (model 0) in a `.vox` file.  If MagicVoxel
  shows multiple models, make sure model 0 is the one you want.

### Axis orientation

MagicVoxel uses `Z-up`.  Nova-Forge also uses `Z-up` for the voxel rendering pipeline,
so models appear with the correct orientation when saved and loaded.

---

## 10. Common Mistakes & Fixes

| Symptom | Likely cause | Fix |
|---|---|---|
| Armor is invisible | Manifest entry missing or wrong specifier | Add/fix the manifest entry; verify the specifier matches the `.ron` path exactly |
| Armor floats above or clips into body | Wrong offset in manifest | Tune the three offset values (see Section 8) |
| Build error: `cannot find value` in manifest | Wrong map key | Ensure the map key matches `common.items.…` asset specifier |
| Mesh looks grey / white | Grayscale mesh used with `color: None` | Either add a `color: Some(r,g,b)` tint or bake real colours into the palette |
| CI fails `ensure_item_localization` | Missing `item_i18n_manifest.ron` or `.ftl` entry | Add both entries (see Section 5) |
| Game panics on item load | `.vox` file missing at expected path | Check the specifier string maps to the right file path (dots → slashes, + `.vox`) |
| NPC has invisible limbs | Missing manifest entry for that segment | Add the segment entry in the correct lateral/central manifest |
| World structure has wrong blocks | Wrong palette index | Open an existing structure in the same dungeon as a reference and copy its palette |
| Hot-reload not working | Running in release mode | Hot-reload only works in debug builds (`cargo run`, not `cargo run --release`) |
