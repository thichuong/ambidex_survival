# Ambidex Survival

**Ambidex Survival** is a dual-stick survival shooter prototype built with **Rust** and the **Bevy** game engine. It features a unique "Ambidex" system where the player controls two independent hands, each capable of wielding different weapons with distinct mechanics and skills.

## 🎮 Features

*   **Dual-Wielding Combat**: Independently control Left and Right hands.
*   **Weapon Variety**:
    *   **Shuriken**: Throw projectiles. **Skill**: Teleport to the last thrown shuriken.
    *   **Sword**: Melee slash with a 2-phase swing. **Skill**: Toggle **Shattered Mode** for high-damage, erratic, wide-range attacks.
    *   **Bow**: Ranged attacks. **Skill**: Cycle between **Single** (High Dmg), **Multishot** (Spread), and **Rapid Speed** (Machine Gun).
    *   **Magic**: Cast spells. **Skill**: Toggle between Primary and Secondary spell slots instantly.
*   **Magic Loadout System**: customize your primary and secondary spells (Energy Bolt, Laser, Nova, Blink, Global Damage).
*   **Survival Waves**: Fight against waves of chasing enemies.
*   **Polish**: Particle effects, screen shake, trail renderers, and responsive UI.
*   **Cross-Platform**: Runs natively on Linux/Windows/Mac and in the Web Browser (WASM).

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

> **Note**: Skills vary by weapon (e.g., Teleport for Shuriken, Mode Toggle for Sword/Bow/Magic).

## 🛠️ Built With

*   [Bevy Engine](https://bevyengine.org/) - A data-driven game engine built in Rust.
*   [Bevy Rapier](https://github.com/dimforge/bevy_rapier) - 2D Physics engine.
*   [Iyadesu](https://github.com/iyadesu) - (Author)

## 📝 License

This project is a prototype created for educational and demonstration purposes.
