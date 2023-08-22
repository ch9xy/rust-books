struct Selector<T> {
    elements: Vec<T>,

    current: usize,
}


use std::ops::{Deref, DerefMut};

impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.elements[self.current]
    }

}


impl<T> DerefMut for Selector<T> {
    fn deref_mut(&self) -> &mut T {
        &mut self.elements[self.current]
    }
}