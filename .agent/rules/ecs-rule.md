---
trigger: always_on
---

# ECS Architecture Rules — Ambidex Survival (Bevy 0.17)

## 1. Kiến Trúc Tổng Quan

Dự án tuân thủ **Plugin-based, Data-Driven ECS** trên Bevy 0.17.

```text
src/
├── components/   ← Pure data (NO logic)
├── configs/      ← Constants & balance tuning
├── plugins/      ← System registration ONLY
├── resources/    ← Global state (ECS Resources)
├── systems/      ← ALL game logic (queries + transforms)
├── utils/        ← Generic helpers
└── visuals/      ← Rendering & VFX
```

### Nguyên tắc cốt lõi
- **Components = Data only**. Không chứa method logic.
- **Systems = Logic only**. Đọc/ghi Components qua Queries.
- **Resources = Global state**. Dùng `Res<T>` / `ResMut<T>`.
- **Plugins = Registration**. Chỉ gọi `add_systems`, `add_observer`, `init_resource`.
- **Configs = Constants**. Tách riêng khỏi logic, dễ balance.
- **Events = Decoupled communication**. Không gọi trực tiếp giữa systems.

---

## 2. Quy Tắc Components (`src/components/`)

### Đặt tên & tổ chức
- Mỗi domain 1 file: `player.rs`, `weapon.rs`, `enemy.rs`, `physics.rs`, `attack_effects.rs`.
- Component struct phải `#[derive(Component)]`.
- Dùng **marker components** (empty struct) cho entity tagging: `Player`, `Enemy`, `GameCamera`.

### RequiredComponents (Bevy 0.17)
Dùng `#[require(...)]` để đảm bảo entity spawn đầy đủ dependencies:

```rust
#[derive(Component, Default)]
#[require(Transform, Visibility, Velocity, Collider, Currency, Health, PlayerStats, CombatStats, Progression)]
pub struct Player;
```

> **QUAN TRỌNG**: Mọi component marker entity phải khai báo `#[require(...)]` đầy đủ.
> Không spawn entity thiếu component — dùng `#[require]` thay vì bundle thủ công.

### Enums
- Dùng `enum` cho state machines: `WeaponType`, `GunMode`, `SwordMode`, `SpellType`, `Faction`.
- Derive `Debug, Clone, Copy, PartialEq, Eq` cho enums.
- Dùng `#[default]` attribute thay vì impl `Default` thủ công khi có thể.

### Default implementations
- Mọi stateful component **phải** impl `Default`.
- Giá trị mặc định tham chiếu từ `configs/` constants.

---

## 3. Quy Tắc Systems (`src/systems/`)

### Tổ chức theo domain
```text
systems/
├── combat/          ← Weapon firing, skills, collision
│   ├── collision/   ← Detection → Damage → Effects → Death
│   ├── magic/       ← Spell implementations
│   ├── mod.rs       ← CombatContext, shared types
│   ├── gun.rs, sword.rs, shuriken.rs
│   ├── elite_ai.rs, yellow_ai.rs
│   └── events.rs
├── ui/              ← HUD, menu, settings
│   └── menu/        ← Weapon menu sub-modules
├── player.rs
├── enemy.rs
├── physics.rs
└── damage_text.rs
```

### CombatContext Pattern (BẮT BUỘC cho combat systems)
Tất cả weapon/spell systems **phải** sử dụng `CombatContext` struct:

```rust
pub struct CombatContext<'a> {
    pub owner_entity: Entity,
    pub transform: &'a mut Transform,
    pub cursor_pos: Vec2,
    pub spawn_pos: Vec2,
    pub damage_multiplier: f32,
    pub combat_stats: &'a CombatStats,
    pub progression: &'a Progression,
}
```

> Không truyền từng parameter riêng lẻ vào helper functions.
> Luôn gói vào `CombatContext`.

### CombatInputParams (SystemParam)
Dùng `#[derive(SystemParam)]` để gom các system parameters thường dùng:

```rust
#[derive(SystemParam)]
pub struct CombatInputParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub time: Res<'w, Time>,
    pub mouse_input: Res<'w, ButtonInput<MouseButton>>,
    pub key_input: Res<'w, ButtonInput<KeyCode>>,
    pub input_settings: Res<'w, InputSettings>,
    pub window: Single<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<GameCamera>>,
    pub cached_assets: Res<'w, CachedAssets>,
    // ...
}
```

### Bevy 0.17 Patterns BẮT BUỘC

| Pattern | Cách dùng | Ví dụ |
|---------|-----------|-------|
| `Single<T>` | Singleton queries (1 entity duy nhất) | `Single<&Window, With<PrimaryWindow>>` |
| `Mut<T>` | Change detection | `&mut Transform` trong query |
| `Observer` | Reactive logic (không chạy mỗi frame) | `app.add_observer(damage_processing_system)` |
| `Event` | Decoupled communication | `CollisionEvent`, `DamageEvent`, `EnemyDeathEvent` |
| `States` | Game state management | `GameState::Playing`, `GameState::WeaponMenu` |
| `run_if` | Conditional scheduling | `.run_if(in_state(GameState::Playing))` |

### Collision Pipeline (thứ tự xử lý)
```
detection.rs → damage.rs → effects.rs → enemy_death.rs
     ↓              ↓            ↓              ↓
CollisionEvent → DamageEvent → Explosion → EnemyDeathEvent
```

### Faction System
- Mọi projectile/attack **phải** có `Faction` component.
- `Faction::Player` → hit `Enemy` entities.
- `Faction::Enemy` → hit `Player` entity.
- Không bao giờ friendly-fire giữa cùng faction.

---

## 4. Quy Tắc Resources (`src/resources/`)

- `GameState` (States): `Playing`, `WeaponMenu`, `Tutorial`, `GameOver`, `Settings`.
- `PreviousMenuState`: Track menu navigation để Back button đúng.
- `RoundManager`: Wave progression.
- `InputSettings`: Customizable keybindings.
- `CachedAssets`: Cache mesh/material handles.
- `ScreenShake`: Polish effects.
- `UniformGrid`: Spatial partitioning cho collision.

> **init_resource**: Đăng ký trong `main.rs`, KHÔNG trong plugins.

---

## 5. Quy Tắc Configs (`src/configs/`)

- **Mỗi domain 1 file**: `player.rs`, `enemy.rs`, `shop.rs`, `visuals.rs`.
- **Weapons/Spells có sub-folder**: `configs/weapons/`, `configs/spells/`.
- Dùng `pub const` hoặc `pub const fn` cho tất cả values.
- **KHÔNG hardcode** giá trị balance trong systems — luôn reference từ configs.

```rust
// ✅ Đúng
pub const PLAYER_SPEED: f32 = 300.0;

// ❌ Sai — hardcode trong system
let speed = 300.0;
```

---

## 6. Quy Tắc Plugins (`src/plugins/`)

- Mỗi plugin 1 file: `combat.rs`, `physics.rs`, `player.rs`, `ui.rs`, `visuals.rs`.
- Plugin **chỉ** register systems và observers, KHÔNG chứa logic.
- Systems chạy khi `Playing` phải có `.run_if(in_state(GameState::Playing))`.
- Dùng `add_observer` cho reactive logic (collision, damage, death).
- Dùng `add_systems(Update, (...))` cho per-frame logic.

---

## 7. Quy Tắc Visuals (`src/visuals/`)

- `ui_icons.rs`: Procedural drawing — KHÔNG dùng external image assets cho icons.
- `world/`: Spawn visual entities cho weapons, projectiles, spells.
- Visual entities **tách riêng** khỏi logic entities khi cần.
- Dùng `CachedAssets` để tránh tạo mesh/material mỗi frame.

---

## 8. Code Quality

### Clippy (pedantic + nursery)
```toml
[lints.clippy]
pedantic = "warn"
nursery = "warn"
```

- Dùng `#[allow(clippy::...)]` chỉ khi có lý do chính đáng.
- Phổ biến: `#[allow(clippy::needless_pass_by_value)]` cho Bevy system params.
- Phổ biến: `#[allow(clippy::type_complexity)]` cho complex queries.

### Formatting
- **Luôn** chạy `cargo fmt` trước commit.

### Testing
- Unit tests trong cùng file (`#[cfg(test)] mod tests`).
- Test pattern Bevy: tạo `App::new()`, thêm systems, spawn entities, gọi `app.update()`.
- Test kết quả qua world queries sau update.

---

## 9. Anti-Patterns (TRÁNH)

| ❌ Anti-Pattern | ✅ Thay thế |
|----------------|-------------|
| Logic trong component methods | Logic trong systems |
| Bundle struct thủ công | `#[require(...)]` |
| System > 100 dòng | Tách thành helper functions + `CombatContext` |
| Hardcode values trong systems | Reference từ `configs/` |
| Gọi trực tiếp giữa systems | Dùng `Event` / `Observer` |
| Query quá nhiều params | Dùng `#[derive(SystemParam)]` |
| Spawn entity thiếu components | Dùng `#[require(...)]` trên marker component |
| Xử lý collision manual | Dùng collision pipeline: detection → damage → effects |
| Tight coupling giữa plugins | Event-driven communication |

---

## 10. Checklist Khi Viết Code Mới

- [ ] Component có `#[derive(Component)]` và `Default` impl?
- [ ] Component marker có `#[require(...)]` đầy đủ?
- [ ] System dùng `CombatContext` cho combat logic?
- [ ] Values balance từ `configs/`, không hardcode?
- [ ] System registered trong đúng plugin?
- [ ] Có `.run_if(in_state(GameState::Playing))` nếu cần?
- [ ] Faction system được tôn trọng (Player vs Enemy)?
- [ ] `cargo check` pass?
- [ ] `cargo clippy` pass (pedantic + nursery)?
- [ ] `cargo test` pass?
- [ ] `cargo fmt` đã chạy?