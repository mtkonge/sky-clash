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

pub struct Receiver<T>(Aptr<VecDeque<T>>);

impl<T> Receiver<T> {
    pub fn try_receive(&mut self) -> Option<T> {
        self.0.lock().pop_front()
    }
}

pub struct Sender<T>(Aptr<VecDeque<T>>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, value: T) {
        self.0.lock().push_back(value);
    }
    pub fn send_important(&mut self, value: T) {
        self.0.lock().push_front(value);
    }
}

pub struct Actor(());
impl Actor {
    pub fn new<T>() -> (Sender<T>, Receiver<T>) {
        let queue = Aptr::new(VecDeque::new());
        (Sender(queue.clone()), Receiver(queue.clone()))
    }
}
