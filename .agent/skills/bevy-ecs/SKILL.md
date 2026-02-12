---
name: Bevy 0.17 ECS Patterns
description: Hướng dẫn sử dụng các ECS patterns cụ thể trong dự án Ambidex Survival (Bevy 0.17)
---

# Bevy 0.17 ECS Patterns — Ambidex Survival

Skill này cung cấp các template code và hướng dẫn cho các ECS patterns được sử dụng trong dự án.

---

## 1. Tạo Component với RequiredComponents

### Marker Component (entity type)

```rust
// src/components/my_entity.rs

use bevy::prelude::*;
use super::physics::{Collider, Velocity};

/// Marker component — entity tự động có đầy đủ dependencies
#[derive(Component, Default)]
#[require(Transform, Visibility, Velocity, Collider)]
pub struct MyEntity;
```

### Data Component

```rust
#[derive(Component)]
pub struct MyStats {
    pub health: f32,
    pub speed: f32,
}

impl Default for MyStats {
    fn default() -> Self {
        Self {
            health: crate::configs::my_entity::DEFAULT_HEALTH,
            speed: crate::configs::my_entity::DEFAULT_SPEED,
        }
    }
}
```

### Enum Component (state machine)

```rust
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyState {
    Idle,
    Active,
    Cooldown,
}
```

### Đăng ký trong mod.rs

```rust
// src/components/mod.rs
pub mod my_entity;  // ← Thêm dòng này
```

---

## 2. Tạo System

### System cơ bản (per-frame)

```rust
// src/systems/my_system.rs

use bevy::prelude::*;
use crate::components::my_entity::MyEntity;

#[allow(clippy::needless_pass_by_value)]
pub fn my_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MyEntity)>,
) {
    let dt = time.delta_secs();
    for (mut transform, _entity) in &mut query {
        transform.translation.x += 100.0 * dt;
    }
}
```

### System với Single<T> (singleton query)

```rust
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::player::Player;

/// Single<T> chỉ dùng khi CHẮC CHẮN có đúng 1 entity
#[allow(clippy::needless_pass_by_value)]
pub fn camera_follow_player(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera.translation = player.translation;
}
```

### Combat System với CombatContext

```rust
use crate::systems::combat::{CombatContext, CombatInputParams};
use crate::components::player::{Hand, Player, PlayerStats, CombatStats, Progression};
use crate::components::weapon::Weapon;
use bevy::prelude::*;

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
pub fn my_weapon_system(
    mut params: CombatInputParams,
    mut player_query: Query<
        (&mut Transform, &PlayerStats, &CombatStats, &Progression, Entity),
        With<Player>,
    >,
    hand_query: Query<(&Hand, &Weapon)>,
) {
    let (mut transform, stats, combat_stats, progression, entity) =
        player_query.single_mut();

    // Tính cursor position
    let (camera, camera_transform) = *params.camera;
    let cursor_pos = params.window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world_2d(camera_transform, pos).ok())
        .unwrap_or_default();

    let spawn_pos = transform.translation.truncate();

    // Tạo CombatContext
    let mut ctx = CombatContext {
        owner_entity: entity,
        transform: &mut transform,
        cursor_pos,
        spawn_pos,
        damage_multiplier: stats.damage_multiplier,
        combat_stats,
        progression,
    };

    // Xử lý từng hand
    for (hand, weapon) in &hand_query {
        fire_weapon(&mut ctx, &mut params, hand, weapon);
    }
}

fn fire_weapon(
    ctx: &mut CombatContext,
    params: &mut CombatInputParams,
    hand: &Hand,
    weapon: &Weapon,
) {
    // Weapon logic ở đây
    // Dùng ctx.spawn_pos, ctx.cursor_pos, ctx.damage_multiplier, etc.
}
```

---

## 3. Observers & Events

### Định nghĩa Event

```rust
// src/systems/combat/events.rs

use bevy::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct MyCustomEvent {
    pub entity: Entity,
    pub value: f32,
}
```

### Trigger Event

```rust
// Trong bất kỳ system nào có Commands
commands.trigger(MyCustomEvent {
    entity: target,
    value: 42.0,
});
```

### Observer (reactive handler)

```rust
use bevy::prelude::*;

pub fn handle_my_event(
    trigger: Trigger<MyCustomEvent>,
    mut query: Query<&mut Transform>,
) {
    let event = trigger.event();
    if let Ok(mut transform) = query.get_mut(event.entity) {
        transform.translation.y += event.value;
    }
}
```

### Đăng ký Observer trong Plugin

```rust
// src/plugins/my_plugin.rs
impl Plugin for MyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_my_event);  // ← Observer
    }
}
```

---

## 4. GameState Scheduling

### Run system chỉ khi Playing

```rust
app.add_systems(
    Update,
    my_system.run_if(in_state(GameState::Playing)),
);
```

### Run system ở nhiều states

```rust
app.add_systems(
    Update,
    ui_update_system.run_if(
        in_state(GameState::Playing)
            .or(in_state(GameState::WeaponMenu))
    ),
);
```

### Chuyển state

```rust
fn go_to_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::WeaponMenu);
}
```

---

## 5. Spatial Partitioning (UniformGrid)

### Grid cho collision detection

```rust
use crate::components::physics::UniformGrid;

pub fn my_query_nearby(
    grid: Res<UniformGrid>,
    query: Query<&Transform, With<Enemy>>,
) {
    let search_pos = Vec2::new(100.0, 200.0);
    let nearby_entities = grid.query_nearby(search_pos);

    for entity in nearby_entities {
        if let Ok(transform) = query.get(entity) {
            // Process nearby enemy
        }
    }
}
```

---

## 6. Unit Test Pattern (Bevy)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_my_system() {
        let mut app = App::new();

        // Setup
        app.init_resource::<Time>();
        app.add_systems(Update, my_system);

        // Spawn test entity
        let entity = app.world_mut().spawn((
            Transform::default(),
            MyComponent { value: 10.0 },
        )).id();

        // Advance time
        {
            let mut time = app.world_mut().resource_mut::<Time>();
            time.advance_by(Duration::from_millis(100));
        }

        // Run one frame
        app.update();

        // Assert
        let component = app.world().get::<MyComponent>(entity).unwrap();
        assert!(component.value > 10.0);
    }
}
```

---

## 7. Resource Pattern

```rust
// src/resources/my_resource.rs

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MyManager {
    pub counter: u32,
    pub active: bool,
}

// Đăng ký trong main.rs:
// .init_resource::<MyManager>()
```

---

## Tham chiếu nhanh: Đường dẫn file

| Pattern | Location |
|---------|----------|
| Components | `src/components/*.rs` |
| Configs | `src/configs/*.rs`, `src/configs/weapons/*.rs`, `src/configs/spells/*.rs` |
| Systems | `src/systems/**/*.rs` |
| Resources | `src/resources/*.rs` |
| Plugins | `src/plugins/*.rs` |
| Visuals | `src/visuals/**/*.rs` |
| Events | `src/systems/combat/events.rs` |
| CombatContext | `src/systems/combat/mod.rs` |
