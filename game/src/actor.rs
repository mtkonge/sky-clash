use std::collections::VecDeque;

use aptr::*;

mod aptr {
    use std::sync::Arc;
    use std::sync::Mutex;
    use std::sync::MutexGuard;
    pub(super) struct Aptr<T>(Arc<Mutex<T>>);

    impl<T> Clone for Aptr<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T> Aptr<T> {
        pub fn new(value: T) -> Self {
            Self(Arc::new(Mutex::new(value)))
        }
        pub fn lock(&mut self) -> MutexGuard<T> {
            self.0.lock().unwrap()
        }
    }
}

pub struct Actor<T>(Aptr<VecDeque<T>>);

impl<T> Actor<T> {
    pub fn new() -> Self {
        Self(Aptr::new(VecDeque::new()))
    }
    pub fn handle(&self) -> Handle<T> {
        Handle::from_actor(self)
    }
    pub fn try_receive(&mut self) -> Option<T> {
        self.0.lock().pop_front()
    }
}

pub struct Handle<T>(Aptr<VecDeque<T>>);

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Handle<T> {
    pub fn send(&mut self, value: T) {
        self.0.lock().push_back(value);
    }
    pub fn send_important(&mut self, value: T) {
        self.0.lock().push_front(value);
    }
    pub fn from_actor(actor: &Actor<T>) -> Self {
        Self(actor.0.clone())
    }
}
