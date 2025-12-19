use bevy::prelude::*;
use std::collections::HashMap;

/// Uniform Grid for spatial partitioning - used to optimize collision detection
/// Only enemies are inserted into the grid for player-enemy collision checks
#[derive(Resource)]
pub struct UniformGrid {
    pub cell_size: f32,
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl Default for UniformGrid {
    fn default() -> Self {
        Self::new(100.0) // Default cell size covers reasonable area
    }
}

impl UniformGrid {
    #[must_use]
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            cells: HashMap::default(),
        }
    }

    /// Clear all entities from the grid
    pub fn clear(&mut self) {
        self.cells.clear();
    }

    /// Get cell coordinates for a world position
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn cell_coords(&self, pos: Vec2) -> (i32, i32) {
        let x = (pos.x / self.cell_size).floor() as i32;
        let y = (pos.y / self.cell_size).floor() as i32;
        (x, y)
    }

    /// Insert an entity at a world position
    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let coords = self.cell_coords(pos);
        self.cells.entry(coords).or_default().push(entity);
    }

    /// Query all entities in the cell containing pos and its 8 neighbors (3x3 grid)
    #[must_use]
    pub fn query_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let (cx, cy) = self.cell_coords(pos);
        let mut result = Vec::new();

        for dx in -1..=1 {
            for dy in -1..=1 {
                if let Some(entities) = self.cells.get(&(cx + dx, cy + dy)) {
                    result.extend(entities.iter().copied());
                }
            }
        }

        result
    }
    /// Query all entities in grid cells intersecting the given AABB (min, max)
    #[must_use]
    pub fn query_aabb(&self, min: Vec2, max: Vec2) -> Vec<Entity> {
        let (min_x, min_y) = self.cell_coords(min);
        let (max_x, max_y) = self.cell_coords(max);
        let mut result = Vec::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(entities) = self.cells.get(&(x, y)) {
                    result.extend(entities.iter().copied());
                }
            }
        }

        result
    }
}

/// Component to mark entities that should ignore the grid for collision checks
/// Used for global or very large/long projectiles (like Laser/Global spells)
#[derive(Component)]
pub struct IgnoreGrid;

/// Custom velocity component for physics simulation
#[derive(Component, Default, Clone, Copy)]
pub struct Velocity {
    pub linvel: Vec2,
    pub angvel: f32,
}

impl Velocity {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            linvel: Vec2::ZERO,
            angvel: 0.0,
        }
    }
}

/// Custom collider for simple collision detection
#[derive(Component, Clone, Copy)]
pub enum Collider {
    Circle {
        radius: f32,
    },
    Rectangle {
        half_width: f32,
        half_height: f32,
    },
    /// Line segment from origin, along direction, with given length and width
    Line {
        direction: Vec2,
        length: f32,
        width: f32,
    },
}

impl Default for Collider {
    fn default() -> Self {
        Self::Circle { radius: 15.0 }
    }
}

impl Collider {
    #[must_use]
    pub const fn ball(radius: f32) -> Self {
        Self::Circle { radius }
    }

    #[must_use]
    pub const fn cuboid(half_width: f32, half_height: f32) -> Self {
        Self::Rectangle {
            half_width,
            half_height,
        }
    }

    #[must_use]
    pub const fn line(direction: Vec2, length: f32, width: f32) -> Self {
        Self::Line {
            direction,
            length,
            width,
        }
    }
}

/// Check collision between two circle colliders
#[must_use]
pub fn circle_circle_collision(pos_a: Vec2, radius_a: f32, pos_b: Vec2, radius_b: f32) -> bool {
    let dist_sq = pos_a.distance_squared(pos_b);
    let radius_sum = radius_a + radius_b;
    dist_sq <= radius_sum * radius_sum
}

/// Check collision between a circle and a rectangle (AABB - no rotation)
#[must_use]
pub fn circle_rect_collision(
    circle_pos: Vec2,
    circle_radius: f32,
    rect_pos: Vec2,
    half_width: f32,
    half_height: f32,
) -> bool {
    let closest_x = circle_pos
        .x
        .clamp(rect_pos.x - half_width, rect_pos.x + half_width);
    let closest_y = circle_pos
        .y
        .clamp(rect_pos.y - half_height, rect_pos.y + half_height);

    let dist_sq = (circle_pos.y - closest_y)
        .mul_add(circle_pos.y - closest_y, (circle_pos.x - closest_x).powi(2));
    dist_sq <= circle_radius * circle_radius
}

/// Check collision between a circle and a line segment
/// Line starts at `line_start` and extends in direction for given length
#[must_use]
pub fn circle_line_collision(
    circle_pos: Vec2,
    circle_radius: f32,
    line_start: Vec2,
    direction: Vec2,
    length: f32,
    width: f32,
) -> bool {
    // Line end point
    let line_end = line_start + direction * length;

    // Vector from line start to circle
    let to_circle = circle_pos - line_start;

    // Project circle onto line direction
    let line_vec = line_end - line_start;
    let line_len_sq = line_vec.length_squared();

    if line_len_sq == 0.0 {
        // Degenerate line (point)
        return circle_pos.distance_squared(line_start) <= (circle_radius + width).powi(2);
    }

    // Clamp projection to line segment [0, 1]
    let t = (to_circle.dot(line_vec) / line_len_sq).clamp(0.0, 1.0);

    // Closest point on line segment to circle
    let closest_point = line_start + line_vec * t;

    // Check distance from circle center to closest point
    let dist_sq = circle_pos.distance_squared(closest_point);
    let threshold = circle_radius + width;

    dist_sq <= threshold * threshold
}

/// Check collision between two colliders at given positions
#[must_use]
pub fn check_collision(
    pos_a: Vec2,
    collider_a: &Collider,
    pos_b: Vec2,
    collider_b: &Collider,
) -> bool {
    match (collider_a, collider_b) {
        (Collider::Circle { radius: r_a }, Collider::Circle { radius: r_b }) => {
            circle_circle_collision(pos_a, *r_a, pos_b, *r_b)
        }
        (
            Collider::Circle { radius },
            Collider::Rectangle {
                half_width,
                half_height,
            },
        ) => circle_rect_collision(pos_a, *radius, pos_b, *half_width, *half_height),
        (
            Collider::Rectangle {
                half_width,
                half_height,
            },
            Collider::Circle { radius },
        ) => circle_rect_collision(pos_b, *radius, pos_a, *half_width, *half_height),
        (
            Collider::Circle { radius },
            Collider::Line {
                direction,
                length,
                width,
            },
        ) => circle_line_collision(pos_a, *radius, pos_b, *direction, *length, *width),
        (
            Collider::Line {
                direction,
                length,
                width,
            },
            Collider::Circle { radius },
        ) => circle_line_collision(pos_b, *radius, pos_a, *direction, *length, *width),
        (
            Collider::Rectangle {
                half_width: half_width_a,
                half_height: half_height_a,
            },
            Collider::Rectangle {
                half_width: half_width_b,
                half_height: half_height_b,
            },
        ) => {
            // AABB vs AABB
            (pos_a.x - half_width_a <= pos_b.x + half_width_b)
                && (pos_a.x + half_width_a >= pos_b.x - half_width_b)
                && (pos_a.y - half_height_a <= pos_b.y + half_height_b)
                && (pos_a.y + half_height_a >= pos_b.y - half_height_b)
        }
        // Line vs Rectangle/Line - fallback to false (not commonly needed)
        _ => false,
    }
}
