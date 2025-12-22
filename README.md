# Ambidex Survival

<p align="center">
  <img src="assets/icon-512.png" alt="Ambidex Survival Icon" width="200" />
</p>

**Ambidex Survival** is a high-octane dual-stick survival shooter built with **Rust** and the **Bevy** engine. Take control of two independent hands, customize your loadout with a variety of weapons and spells, and survive as long as you can against ever-increasing waves of enemies.

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
- **Event-Driven Communication**: Unified event handling using Bevy's native `Event` system enhanced with `Message` derive, providing typesafe `MessageReader` and `MessageWriter` for inter-plugin communication.
- **Reactive Observers**: High-performance reactive logic triggered by `On<E>` observers (e.g., spawn damage text on hit, trigger effects on death), reducing per-frame overhead.
- **Collision Pipeline**: Dedicated `collision/` sub-module handles detection, damage processing, and visual effects in separate, focused systems.
- **GameState Management**: Proper system scheduling and UI transitions via Bevy `States` (Playing, Paused, GameOver, WeaponMenu).

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
A powerful close-quarters weapon with two distinct styles.
- **Normal Mode**: Standard strikes with moderate range and high damage.
- **Shattered Mode (Skill Toggle)**: The blade shatters into fragments, covering a massive area but dealing lower damage per hit.

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

## 💰 Economy & Progression

- **Gold**: Earn **10G** for every enemy killed.
- **Shop & Cards**: Spend your gold on upgrades that apply to **all** weapons (except CDR).
    - **Heal**: Restore 30 HP (**50G**).
    - **Damage Up**: Increase damage by **10%** (**100G**).
    - **Max HP Up**: Increase Max HP by **20** (**150G**).
    - **Crit Damage**: Increase critical strike damage by **50%** (**200G**).
    - **Crit Chance**: Increase critical strike chance by **10%** (**250G**, Max 10).
    - **Lifesteal**: Heal for **10%** of damage dealt (**300G**, Max 5).
    - **Magic CDR**: Reduce Magic weapon cooldowns by **10%** (**350G**, Max 8).
- **Rounds**: Waves scale in size and intensity. Clear all enemies to access the shop.

## 🕹️ Controls

| Action | Input | Description |
| :--- | :--- | :--- |
| **Move** | `W`, `A`, `S`, `D` | Character movement |
| **Left Attack** | `Mouse Left` | Fire left-hand weapon |
| **Right Attack** | `Mouse Right` | Fire right-hand weapon |
| **Left Skill** | `Q` | Use left-hand unique skill |
| **Right Skill** | `E` | Use right-hand unique skill |
| **Menu** | `ESC` / `Menu Button` | Toggle weapon & shop menu |

## 🛠️ Built With

*   [Bevy Engine](https://bevyengine.org/) - A data-driven game engine built in Rust.
*   [Bevy Rapier](https://github.com/dimforge/bevy_rapier) - 2D Physics engine.

## 📝 License

This project is a prototype created for educational and demonstration purposes.
