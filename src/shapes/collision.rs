pub trait Collides<T> {
    fn is_colliding(&self, other: &T) -> bool;

    fn is_all_colliding(&self, others: &[T]) -> bool {
        for i in others {
            if self.is_colliding(i) {
                return true;
            }
        };

        return false;
    }
}


impl<T, S> Collides<S> for T where T: Contains, S: Points {
    fn is_colliding(&self, other: &S) -> bool {
        for i in other.points() {
            if self.contains_point(i) {
                return true;
            }
        }

        return false;
    }

    fn is_all_colliding(&self, others: &[S]) -> bool {
        for i in others {
            if self.is_colliding(i) {
                return true;
            }
        };

        return false;
    }
}


pub struct PointIter {
    pub idx: usize,
    pub points: Vec<(f64, f64)>,
}

impl Iterator for PointIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.points.len() < 1 {
            return None;
        }

        let single_point = self.points.get(self.idx);
        self.idx += 1;

        return single_point.copied();
    }
}


pub trait Points {
    fn points(&self) -> PointIter;
}

pub trait Contains {
    fn contains_point(&self, points: (f64, f64)) -> bool;
}

impl From<Vec<(f64, f64)>> for PointIter {
    fn from(points: Vec<(f64, f64)>) -> Self {
        PointIter {
            idx: 0,
            points,
        }
    }
}