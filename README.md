# Ambidex Survival

**Ambidex Survival** is a high-octane dual-stick survival shooter built with **Rust** and the **Bevy** engine. Take control of two independent hands, customize your loadout with a variety of weapons and spells, and survive as long as you can against ever-increasing waves of enemies.

## ⚔️ The "Ambidex" Combat System

The core of the game is the **Ambidex System**, which gives you independent control over your character's two hands. Each hand can be equipped with any weapon type, allowing for thousands of potential combinations.

- **Independent Hands**: Each hand (Left and Right) has its own weapon, cooldown, and skill state.
- **Weapon Selection**: Access the **Weapon Menu** between rounds to swap weapons for either hand.

## 🔫 Weapons & Skills

Every weapon features a primary attack and a unique "Skill" (`Q` for Left Hand, `E` for Right Hand).

### 🗡️ Sword (Melee)
A powerful close-quarters weapon with two distinct styles.
- **Normal Mode**: Standard strikes with moderate range and high damage.
    - *Damage*: 60 | *Range*: 200 | *Cooldown*: 0.5s
- **Shattered Mode (Skill Toggle)**: The blade shatters into fragments, covering a massive area but dealing lower damage per hit.
    - *Damage*: 20 | *Range*: 600 | *Cooldown*: 0.5s

### 🔫 Gun (Firearm)
A versatile ranged weapon with three fire modes.
- **Single Shot**: High precision and high damage.
    - *Damage*: 60 | *Speed*: 1000 | *Cooldown*: 0.5s
- **Shotgun**: Fires a wide spread of 7 pellets.
    - *Damage*: 15 (per pellet) | *Speed*: 800 | *Cooldown*: 0.5s
- **Rapid Fire**: Lower damage but incredibly high fire rate. Hold button to spray.
    - *Damage*: 20 | *Speed*: 900 | *Cooldown*: 0.1s
- **Skill Cycle**: Toggles between Single -> Shotgun -> Rapid.

### ❄️ Shuriken (Utility)
Rapid-fire throwing stars with a unique mobility skill.
- **Projectiles**: Throw fast-moving shurikens that persist for 2 seconds.
    - *Damage*: 30 | *Speed*: 1000 | *Count*: Max 12 active
- **Teleport (Skill)**: Instantly teleport to the nearest active shuriken. Great for dodging or repositioning.

### 🔮 Magic (Spellcasting)
The most customizable weapon. Each Magic Hand has two spell slots: **Primary** and **Secondary**.
- **Spell Slots**: Each hand carries two spells that you can toggle between using the skill key.
- **Spells Available**:
    - **Energy Bolt**: Projectile that creates a large 80-unit explosion on impact (30 Dmg).
    - **Laser**: Instant-hit high-velocity beam (30 Dmg).
    - **Nova**: A radial burst centered on the player for high area damage (80 Dmg).
    - **Blink**: Short-range teleport to the cursor position.
    - **Global**: A massive strike that hits everything on screen (15 Dmg).

## 💰 Economy & Progression

- **Gold**: Earn **10G** for every enemy killed.
- **Shop (Menu)**: Spend your hard-earned gold on upgrades:
    - **Heal**: Restore 30 HP for **50G**.
    - **Damage Up**: Permanently increase your damage by **10%** for **100G** (stacks).
- **Rounds**: The game proceeds in rounds. Clear all enemies to open the menu and prepare for the next wave. Wave size and intensity increase every round.

## 🕹️ Controls

| Action | Input | Description |
| :--- | :--- | :--- |

> **Note**: Skills vary by weapon (e.g., Teleport for Shuriken, Mode Toggle for Sword/Gun/Magic).

## 🛠️ Built With

*   [Bevy Engine](https://bevyengine.org/) - A data-driven game engine built in Rust.
*   [Bevy Rapier](https://github.com/dimforge/bevy_rapier) - 2D Physics engine.
*   [Iyadesu](https://github.com/iyadesu) - (Author)

## 📝 License

This project is a prototype created for educational and demonstration purposes.
