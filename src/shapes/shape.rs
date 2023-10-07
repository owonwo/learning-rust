use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::shapes::circle::Circle;
use crate::shapes::collision::{Contains, PointIter, Points};
use crate::shapes::rect::Rect;

pub enum Shape {
    Circle(Circle),
    Rect(Rect),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tag, value) = s.split_once(" ").unwrap_or(("", ""));

        return match tag {
            "rect" => Ok(Shape::Rect(value.parse::<Rect>()?)),
            "circle" => Ok(Shape::Circle(value.parse::<Circle>()?)),
            _ => Err(anyhow::anyhow!("Invalid shape provided"))
        };
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Shape::Circle(c) => write!(f, "{}", c),
            Shape::Rect(r) => write!(f, "{}", r),
        }
    }
}

impl Contains for &Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        return match self {
            Shape::Rect(rect) => rect.contains_point(point),
            Shape::Circle(circ) => circ.contains_point(point),
        }
    }
}

impl Points for &Shape {
    fn points(&self) -> PointIter {
        return match self {
            Shape::Rect(rect) => rect.points(),
            Shape::Circle(circ) => circ.points(),
        }
    }
}
