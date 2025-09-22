use crate::prelude::*;
use crate::shapes::{Circle, Rect};

pub struct CollisionHit {
    pub position: Vec2,
    pub normal: Vec2,
}

pub trait Collider2 {
    fn check_rect(&self, self_pos: Vec2, rect: Rect, rect_pos: Vec2) -> Option<CollisionHit>;
    fn check_circle(
        &self,
        self_pos: Vec2,
        circle: Circle,
        circle_pos: Vec2,
    ) -> Option<CollisionHit>;
}

impl Collider2 for Circle {
    fn check_rect(&self, self_pos: Vec2, rect: Rect, rect_pos: Vec2) -> Option<CollisionHit> {
        let min = rect_pos;
        let max = rect_pos + rect.size;

        let p = Vec2::clamp(self_pos, min, max);
        let d = self_pos - p;

        let dist_sq = d.length_squared();
        let radius_sq = self.radius * self.radius;
        if dist_sq > radius_sq {
            return None;
        }

        let normal = if dist_sq < 0.0001 {
            let min_x = self_pos.x - min.x;
            let min_y = self_pos.y - min.y;
            let max_x = max.x - self_pos.x;
            let max_y = max.y - self_pos.x;
            let min = f32::min(min_x, f32::min(min_y, f32::min(max_x, max_y)));
            if f32::abs(min - min_x) < f32::EPSILON {
                Vec2::new(-1.0, 0.0)
            } else if f32::abs(min - max_x) < f32::EPSILON {
                Vec2::new(1.0, 0.0)
            } else if f32::abs(min - min_y) < f32::EPSILON {
                Vec2::new(0.0, -1.0)
            } else {
                Vec2::new(0.0, 1.0)
            }
        } else {
            d.normalize()
        };

        Some(CollisionHit {
            position: p,
            normal,
        })
    }

    fn check_circle(
        &self,
        self_pos: Vec2,
        circle: Circle,
        circle_pos: Vec2,
    ) -> Option<CollisionHit> {
        let diff = circle_pos - self_pos;
        let dist = diff.length();
        if dist < 0.01 || dist > (self.radius + circle.radius) {
            return None;
        }

        let n = diff / dist;

        Some(CollisionHit {
            position: Vec2::new(
                self_pos.x + self.radius * n.x,
                self_pos.y + self.radius * n.y,
            ),
            normal: n,
        })
    }
}

impl Collider2 for Rect {
    fn check_rect(&self, self_pos: Vec2, rect: Rect, rect_pos: Vec2) -> Option<CollisionHit> {
        let diff = rect_pos - self_pos;
        let extent = self.size + rect.size;
        let pen = extent - Vec2::abs(diff);
        if pen.x <= f32::EPSILON || pen.y <= f32::EPSILON {
            return None;
        }

        let normal = if pen.x < pen.y {
            Vec2::new(if diff.x > 0.0 { 1.0 } else { -1.0 }, 0.0)
        } else {
            Vec2::new(0.0, if diff.y > 0.0 { 1.0 } else { -1.0 })
        };

        let point = self_pos + rect.size * -normal;
        Some(CollisionHit {
            position: point,
            normal,
        })
    }

    fn check_circle(
        &self,
        self_pos: Vec2,
        circle: Circle,
        circle_pos: Vec2,
    ) -> Option<CollisionHit> {
        Circle::check_rect(&circle, circle_pos, *self, self_pos)
    }
}
