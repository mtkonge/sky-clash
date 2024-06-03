use std::{rc::Rc, sync::Mutex};

pub struct SharedPtr<T: ?Sized>(Rc<Mutex<T>>);

impl<T: ?Sized> Clone for SharedPtr<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> SharedPtr<T> {
    pub fn new(v: T) -> Self {
        Self(Rc::new(Mutex::new(v)))
    }

    pub fn lock(&self) -> std::sync::MutexGuard<T> {
        self.0.lock().unwrap()
    }
}

impl<T> From<T> for SharedPtr<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
