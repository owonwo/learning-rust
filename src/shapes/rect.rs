use crate::shapes::area::Area;
use std::fmt::{Display, Formatter};
use crate::shapes::circle::Circle;
use crate::shapes::collision::Collides;

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height;
    }
}


impl Collides<Rect> for Rect {
    fn is_colliding(&self, other: &Rect) -> bool {
        self.contains_point((other.x, other.y))
    }
}


impl Collides<Circle> for Rect {
    fn is_colliding(&self, other: &Circle) -> bool {
        self.contains_point((other.x, other.y))
    }
}


pub struct RectIter {
    points: Vec<(f64, f64)>,
    index: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.points.len() {
            return None;
        }

        let value = self.points.get(self.index);
        self.index += 1;

        return value.map(|x| *x);
    }
}


impl IntoIterator for Rect {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&(self)).into();
    }
}

impl IntoIterator for &Rect {
    type Item = (f64, f64);
    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (self).into();
    }
}

impl Area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Rectangle({},{}) {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

impl From<&Rect> for RectIter {
    fn from(value: &Rect) -> Self {
        return RectIter {
            points: vec![
                (value.x, value.y),
                (value.x + value.width, value.y),
                (value.x + value.width, value.y + value.height),
                (value.x, value.y + value.height),
            ],
            index: 0,
        };
    }
}
