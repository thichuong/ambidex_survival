---
description: Game development workflow cho Ambidex Survival (Bevy 0.17 ECS)
---

# Game Development Workflow

## Tổng quan

Workflow phát triển game tuần tự 7 bước, áp dụng cho mọi thay đổi (feature mới, balance, bugfix, UI).

---

## Bước 1: Plan (Phân tích & Thiết kế)

1. **Xác định scope**: Feature mới? Bugfix? Balance change? UI update?
2. **Phân tích ECS impact**:
   - Components nào cần thêm/sửa? → `src/components/`
   - Configs nào cần thêm/sửa? → `src/configs/`
   - Systems nào cần thêm/sửa? → `src/systems/`
   - Resources nào cần thêm/sửa? → `src/resources/`
   - Plugin nào cần update? → `src/plugins/`
   - Visuals nào cần thêm/sửa? → `src/visuals/`
   - UI nào cần thêm/sửa? → `src/systems/ui/`
3. **Kiểm tra dependencies**: Đọc code hiện tại để hiểu data flow.
4. **Tuân thủ patterns**: `CombatContext`, `RequiredComponents`, `Observer/Event`, `Faction`.

---

## Bước 2: Implement (Viết Code)

Thứ tự implement theo dependency graph:

```
1. Components (data structs)
      ↓
2. Configs (constants & balance)
      ↓
3. Systems (logic)
      ↓
4. Events (communication)
      ↓
5. Plugin registration
      ↓
6. Visuals (rendering)
      ↓
7. UI (menus & HUD)
```

### Quy tắc implement
- Mỗi weapon/spell system dùng `CombatContext` pattern.
- Entity markers dùng `#[require(...)]` để đảm bảo components.
- Reactive logic (collision, damage, death) dùng `Observer`.
- Per-frame logic đăng ký `add_systems(Update, ...)`.
- Combat systems phải có `.run_if(in_state(GameState::Playing))`.
- Balance values trong `configs/`, KHÔNG hardcode.

---

## Bước 3: Check (Static Analysis)

// turbo
```bash
cargo check 2>&1
```

- Sửa tất cả compile errors trước khi tiếp tục.
- Kiểm tra warnings — đặc biệt unused imports và dead code.

---

## Bước 4: Lint (Clippy)

// turbo
```bash
cargo clippy -- -W clippy::pedantic -W clippy::nursery 2>&1
```

- Project dùng `pedantic` + `nursery` lint level.
- Sửa tất cả warnings trừ khi có `#[allow]` với lý do chính đáng.
- Các `#[allow]` phổ biến hợp lệ:
  - `#[allow(clippy::needless_pass_by_value)]` — Bevy system params phải owned.
  - `#[allow(clippy::type_complexity)]` — Complex Bevy queries.
  - `#[allow(clippy::unnecessary_wraps)]` — Bevy system return type convention.

---

## Bước 5: Test (Automated)

// turbo
```bash
cargo test 2>&1
```

- Unit tests trong cùng file source (`#[cfg(test)] mod tests`).
- Test pattern Bevy:
  ```rust
  #[test]
  fn test_my_system() {
      let mut app = App::new();
      app.init_resource::<Time>();
      app.add_systems(Update, my_system);
      // Spawn test entities
      let entity = app.world_mut().spawn(MyComponent::default()).id();
      // Advance time if needed
      app.world_mut().resource_mut::<Time>().advance_by(Duration::from_millis(100));
      app.update();
      // Assert results
      assert!(app.world().get_entity(entity).is_ok());
  }
  ```

---

## Bước 6: Manual Test (Run Game)

### Native
```bash
cargo run
```

### Web (WASM)
```bash
trunk serve
```

- Test gameplay flow: WeaponMenu → Battle → GameOver → Restart.
- Kiểm tra weapon interactions, collision, damage numbers.
- Kiểm tra UI scaling ở nhiều kích thước cửa sổ.
- Kiểm tra enemy AI behavior (Elite, Mirror Mage).

---

## Bước 7: Format & Finalize

// turbo
```bash
cargo fmt
```

- Format toàn bộ code trước khi commit.
- Review lại diff — đảm bảo không có thay đổi ngoài ý muốn.

---

## Bước 8: Build Verification (Optional)

### WASM Production Build
```bash
trunk build --release
```

- Kiểm tra output trong `dist/`.
- Profile release: `opt-level = "z"`, `lto = true`, `strip = true`.

---

## Quick Reference Commands

| Mục đích | Command |
|----------|---------|
| Check compile | `cargo check` |
| Lint | `cargo clippy` |
| Test | `cargo test` |
| Test with logs | `cargo test -- --nocapture` |
| Run native | `cargo run` |
| Run web | `trunk serve` |
| Format | `cargo fmt` |
| Build WASM | `trunk build --release` |

---

## Troubleshooting

### WASM build fails "core not found"
```bash
rustup target add wasm32-unknown-unknown
```

### Clippy pedantic too strict
Thêm `#[allow(clippy::lint_name)]` với comment giải thích lý do.

### Bevy system ordering issues
Dùng `.before()` / `.after()` hoặc tách vào system sets riêng.

### Entity query panics
- Kiểm tra `#[require(...)]` trên marker component.
- Dùng `Single<T>` chỉ khi chắc chắn có đúng 1 entity.
- Dùng `Option<Single<T>>` nếu entity có thể không tồn tại.