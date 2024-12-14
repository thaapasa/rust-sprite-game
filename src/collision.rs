use ggez::glam::Vec2;
use ggez::graphics::Rect;

use crate::primitives::RectExt;

const COLLISION_TOLERANCE: f32 = 0.0001;

/// Finds minimum translation vector that moves target backwards along
/// velocity, so it doesn't collide with `o` anymore.
pub fn find_mtv(target: &Rect, o: &Rect, velocity: Vec2, along_x: bool) -> Option<Vec2> {
    let coll_box = target.clip(o)?;

    // Collision amount, per axis
    let mut cx = coll_box.w;
    let mut cy = coll_box.h;
    if cx.abs() <= COLLISION_TOLERANCE && cy.abs() <= COLLISION_TOLERANCE {
        return None;
    }

    // pushback
    let pb = velocity.normalize() * -1.0;

    // Adjust direction
    if pb[0] < 0.0 {
        cx = -cx
    }
    if pb[1] < 0.0 {
        cy = -cy
    }

    if along_x {
        Some(Vec2::new(cx, 0.0))
    } else {
        Some(Vec2::new(0.0, cy))
    }
}
