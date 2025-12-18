# Ambidex Survival

**Ambidex Survival** is a dual-stick survival shooter prototype built with **Rust** and the **Bevy** game engine. It features a unique "Ambidex" system where the player controls two independent hands, each capable of wielding different weapons with distinct mechanics and skills.

## � Key Features

### ⚔️ The "Ambidex" Combat System
Unlike traditional shooters, **Ambidex Survival** gives you independent control over your character's two hands.
*   **Left Hand (Left Click + Q)**: Equip a weapon for your left hand.
*   **Right Hand (Right Click + E)**: Equip a different weapon for your right hand.
*   **Synergy**: Combine weapons for unique playstyles (e.g., Gun for long-range + Sword for close encounters, or Double Magic for maximum chaos).

### 🔫 Weapon Mastery & Skills
Each weapon has a primary attack and a generic "Skill" key (`Q` or `E`) that changes its functionality:

*   **🗡️ Sword**: A reliable melee weapon.
    *   **Normal Mode**: A balanced 2-phase slash with standard range.
    *   **Shattered Mode (Skill)**: Toggles the sword into an unstable form. Deal **Reduced Damage** but covers a massive area with **Extended Range** and random strikes. Perfect for crowd control.

*   **🔫 Gun**: A versatile firearm with three tactical modes:
    *   **Single Shot**: High damage, high velocity precision rounds.
    *   **Shotgun (Skill Cycle)**: Fires a spread of projectiles for massive area coverage.
    *   **Rapid Fire (Skill Cycle)**: Hold Left-Click to unleash a machine-gun stream of low-damage bullets.

*   **💠 Magic**: The ultimate customizable weapon.
    *   **Loadout System**: Choose two spells from your grimoire to equip to a single hand (Primary & Secondary slots).
    *   **Spells**:
        *   *Energy Bolt*: Standard projectile that explodes on impact.
        *   *Laser*: Instant-hit beam weapon.
        *   *Nova*: Point-blank radial burst.
        *   *Blink*: Short-range teleport.
        *   *Global*: Damages all enemies on screen.
    *   **Tactical Swap (Skill)**: Instantly toggle which spell slot is active (Primary ↔️ Secondary) for dynamic combos.

*   **⭐ Shuriken**: A utility throwing weapon.
    *   **Teleport (Skill)**: Instantly teleport to the location of your nearest active shuriken projectile. Use this for dodging or aggressive positioning.

### 💀 Survival Mechanics
*   **Infinite Waves**: Enemies spawn continuously and increase in intensity.
*   **Physics-Based Interaction**: Projectiles and enemies interact using the `rapier2d` physics engine.
*   **Visual Polish**: Dynamic particle effects, weapon trails, and screen shake feedback.

## 🚀 Getting Started

### Prerequisites

*   [Rust](https://www.rust-lang.org/tools/install) (latest stable)
*   [Trunk](https://trunkrs.dev/) (for Web builds): `cargo install trunk`
*   Standard build tools (gcc, alsa-lib, udev dependencies for Bevy on Linux)

### Running Natively

```bash
cargo run --release
```

### Running on Web (WASM)

```bash
trunk serve
```

```bash
trunk serve --release
```

Then open `http://localhost:8080` in your browser.

## 🕹️ Controls

| Action | Input | Description |
| :--- | :--- | :--- |
| **Move** | `W`, `A`, `S`, `D` | Move the player character. |
| **Aim** | `Mouse Cursor` | Rotate hands towards cursor. |
| **Left Hand Action** | `Left Click` | Use the weapon equipped in the **Left Hand**. |
| **Right Hand Action** | `Right Click` | Use the weapon equipped in the **Right Hand**. |
| **Left SKill** | `Q` | Activate special skill or toggle mode for **Left Hand**. |
| **Right Skill** | `E` | Activate special skill or toggle mode for **Right Hand**. |

> **Note**: Skills vary by weapon (e.g., Teleport for Shuriken, Mode Toggle for Sword/Gun/Magic).

## 🛠️ Built With

*   [Bevy Engine](https://bevyengine.org/) - A data-driven game engine built in Rust.
*   [Bevy Rapier](https://github.com/dimforge/bevy_rapier) - 2D Physics engine.
*   [Iyadesu](https://github.com/iyadesu) - (Author)

## 📝 License

This project is a prototype created for educational and demonstration purposes.
