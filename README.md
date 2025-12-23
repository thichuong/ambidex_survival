# Ambidex Survival

<p align="center">
  <img src="assets/icon-512.png" alt="Ambidex Survival Icon" width="200" />
</p>

**Ambidex Survival** is a high-octane dual-stick survival shooter built with **Rust** and the **Bevy** engine. Take control of two independent hands, customize your loadout with a variety of weapons and spells, and survive as long as you can against ever-increasing waves of enemies, including dangerous **Elite** variants.

## ⚔️ The "Ambidex" Combat System

The core of the game is the **Ambidex System**, which gives you independent control over your character's two hands. Each hand can be equipped with any weapon type, allowing for thousands of potential combinations.

- **Independent Hands**: Each hand (Left and Right) has its own weapon, cooldown, and skill state.
- **Weapon Selection**: Access the **Weapon Menu** between rounds to swap weapons for either hand.

## 🏗️ Technical Architecture

The game utilizes a **Decoupled ECS Design** powered by Bevy 0.17, organized into a **Plugin-based Architecture** for modularity and maintainability.

### Core Principles
- **Plugin Organization**: Structured into independent plugins (`CombatPlugin`, `UIPlugin`, `PhysicsPlugin`, `VisualsPlugin`, `PlayerPlugin`) for clean separation of concerns.
- **Modular Weapon Systems**: Every weapon (Sword, Gun, Shuriken, Magic) is an independent system using optimized ECS filters and trait-like component patterns.
- **Bevy 0.17 Ergonomics**: Leverages `Single<T>` for singleton access, `Mut<T>` for efficient change detection, and `()` system return types for standard compliance.
- **Unified Combat Context**: All weapon and spell logic shares a `CombatContext` struct, streamlining parameter passing and simplifying function signatures.
- **Event-Driven Communication**: Relies on Bevy's native `Event` system for clean, decoupled communication between plugins.
- **Reactive Observers**: High-performance reactive logic triggered by `On<E>` observers for combat (collisions, damage), UI interactions (purchases, card selection), and visual effects, significantly reducing per-frame overhead.
- **RequiredComponents**: Leverages `#[require(...)]` to ensure entities are spawned with their full set of dependencies, reducing boilerplate in spawning systems.
- **Collision Pipeline**: Dedicated `collision/` sub-module handles detection, damage processing, and visual effects in separate, focused systems.
- **GameState Management**: Proper system scheduling and UI transitions via Bevy `States` (Playing, Paused, GameOver, WeaponMenu, Tutorial).
- **Faction System**: Integrated targeting logic ensures attacks hit intended factions while preventing friendly fire among enemies.

## 📁 Project Structure
For a detailed breakdown of the codebase organization, see [PROJECT_STRUCTURE.md](./PROJECT_STRUCTURE.md).

## 🚀 Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Trunk](https://trunkrs.dev/#install) (for Web/Wasm builds)

### Run Locally (Native)
```bash
cargo run
```

### Run in Browser (Web)
```bash
trunk serve
```

## 🧪 Testing

The project includes unit tests for core mechanics like physics, spatial partitioning, and combat systems.

### Run Tests
```bash
cargo test
```

### Run Tests with Logging
```bash
cargo test -- --nocapture
```

## 🔫 Weapons & Skills

Every weapon features a primary attack and a unique "Skill" (`Q` for Left Hand, `E` for Right Hand).

### 🗡️ Sword (Melee)
A powerful close-quarters weapon with advanced mechanical depth.
- **Swing Mechanics**: Features a semi-circular damage arc (180°) that adjusts its origin point based on the equipped hand (Left/Right) for a more natural combat feel.
- **Modes**:
    - **Normal Mode**: Standard strikes with moderate range and high damage.
    - **Shattered Mode (Skill Toggle)**: The blade shatters into fragments, covering a massive area but dealing lower damage per hit.
- **AOE Property**: The sword is classified as an AOE weapon, which applies a 50% penalty to lifesteal healing.

### 🔫 Gun (Firearm)
A versatile ranged weapon with three fire modes.
- **Single Shot**: High precision and high damage.
- **Shotgun**: Fires a wide spread of 7 pellets.
- **Rapid Fire**: Lower damage but incredibly high fire rate. Hold button to spray.
- **Skill Cycle**: Toggles between Single -> Shotgun -> Rapid.

### ❄️ Shuriken (Utility)
Rapid-fire throwing stars with a unique mobility skill.
- **Projectiles**: Throw fast-moving shurikens that persist for 2 seconds (Max 12).
- **Teleport (Skill)**: Instantly teleport to the nearest active shuriken. Great for dodging.

### 🔮 Magic (Spellcasting)
The most customizable weapon. Each Magic Hand has two spell slots: **Primary** and **Secondary**.
- **Spell Slots**: Toggle between your two spells using the skill key.
- **Magic CDR**: Only Magic weapons benefit from the **Cooldown Reduction** upgrade.
- **Spells Available**:
    - **Energy Bolt**: Projectile that creates a large explosion on impact.
    - **Laser**: Instant-hit high-velocity beam.
    - **Nova**: A radial burst centered on the player for high area damage.
    - **Blink**: Short-range teleport to the cursor position.
    - **Global**: A massive strike that hits everything on screen.

## 👺 Enemies & AI

The game features intelligent enemies that scale in difficulty as rounds progress.

### 🧟 Normal Enemies
Swarm-type enemies that chase the player and deal contact damage.
- **Scaling**: HP and speed increase with each round.
- **Reward**: **10G** per kill.

### ⚡ Elite Enemies
Advanced variants that appear in later rounds with unique combat logic.
- **Teleportation**: Randomly teleports near the player to maintain pressure.
- **Spread Shot**: Fires purple high-velocity shurikens in a wide arc.
- **Damage Scaling**: Elite shuriken damage scales with the player's own damage and critical hit stats, making them a constant threat.
- **HP Scaling**: Gains **20 Max HP** per round for increased tankiness.
- **Reward**: **100G** per kill.
### 🧙‍♂️ Mirror Mage (Yellow Enemies)
A tactical spellcaster that appears after Round 5.
- **Tactical Blink**: Periodically teleports to random locations around the player to keep them off-balance.
- **Global Strike**: Casts screen-wide global spells that damage the player if not careful.
- **Scaling**: 
    - **Damage**: +10% damage multiplier per round.
    - **Critical Strike**: Starts with 0% chance and 2.0x damage, gaining +10% chance and +0.5x damage every round.
- **Color**: Yellow.
- **Reward**: High prestige and tactical challenge.

## 💰 Economy & Progression

- **Gold**: Earn **10G** for every enemy killed.
- **Shop & Cards**: Spend your gold on upgrades in the Weapon Menu.
    - **Visual Feedback**: Cards display current purchase counts and limits (e.g., `[3 / 10]`). Basic upgrades with no limit show an infinity symbol (∞). When an upgrade is maxed, the card dims.
    - **Heal**: Restore 30 HP (**50G**, Unlimited).
    - **Damage Up**: Increase damage by **10%** (**100G**, Unlimited).
    - **Max HP Up**: Increase Max HP by **20** (**150G**, Max 10).
    - **Crit Damage**: Increase critical strike damage by **50%** (**200G**, Unlimited).
    - **Crit Chance**: Increase critical strike chance by **10%** (**250G**, Max 10).
    - **Lifesteal**: Heal for **10%** of damage dealt (**300G**, Max 5). **Note**: AOE damage (Sword, Explosions, Nova) heals for only 50% of the lifesteal value.
    - **Magic CDR**: Reduce Magic weapon cooldowns by **10%** (**350G**, Max 5).
    - **Nova Core**: Allow the Nova spell to explode at the mouse cursor position instead of the player (**1000G**, Max 1).
- **Rounds**: Waves scale in size and intensity. Clear all enemies to access the shop.

## 🕹️ Controls

| Action | Input | Description |
| :--- | :--- | :--- |
| **Move** | `W`, `A`, `S`, `D` | Character movement |
| **Left Attack** | `Mouse Left` | Fire left-hand weapon |
| **Right Attack** | `Mouse Right` | Fire right-hand weapon |
| **Left Skill** | `Q` | Use left-hand unique skill |
| **Right Skill** | `E` | Use right-hand unique skill |
| **Menu** | `ESC` / `Menu Button` | Toggle weapon & shop menu or back from tutorial |

## 🛠️ Built With

*   [Bevy Engine](https://bevyengine.org/) - A data-driven game engine built in Rust.
*   [Bevy Rapier](https://github.com/dimforge/bevy_rapier) - 2D Physics engine.

## 📝 License

This project is a prototype created for educational and demonstration purposes.
