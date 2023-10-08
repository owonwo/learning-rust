use crate::shapes::area::Area;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::shapes::collision::{Contains, PointIter, Points};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Contains for Rect {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return x >= self.x && x <= self.x + self.width &&
            y >= self.y && y <= self.y + self.height;
    }
}

impl Points for Rect {
    fn points(&self) -> PointIter {
        return PointIter {
            idx: 0,
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x + self.width, self.y + self.height),
                (self.x, self.y + self.height),
            ],
        };
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64);
    type IntoIter = PointIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&(self)).points();
    }
}

impl IntoIterator for &Rect {
    type Item = (f64, f64);
    type IntoIter = PointIter;

    fn into_iter(self) -> Self::IntoIter {
        return (self).points();
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


impl FromStr for Rect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s
            .split(" ")
            .collect();

        if v.len() != 4 {
            return Err(anyhow::anyhow!("Invalid Rect type provided!"));
        }

        return Ok(Rect {
            x: v[0].parse()?,
            y: v[1].parse()?,
            width: v[2].parse()?,
            height: v[3].parse()?,
        });
    }
}