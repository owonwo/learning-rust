use crate::shapes::area::Area;
use crate::shapes::collision::{Collides};
use crate::shapes::rect::Rect;

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}


impl Collides<Rect> for Circle {
    fn is_colliding(&self, other: &Rect) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true
            }
        }

        return false;
    }
}


impl Collides<Circle> for Circle {
    fn is_colliding(&self, other: &Circle) -> bool {
        self.contains_point((other.x, other.y))
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

