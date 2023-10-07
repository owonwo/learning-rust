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
