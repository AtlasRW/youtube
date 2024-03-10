use crate::types::*;
use futures::Future;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug, Clone)]
pub struct Async<T> {
    inner: Arc<Mutex<T>>,
}

impl<T: Debug + Copy + Send + 'static> GetterSetter<T> for Async<T> {
    fn new(default: T) -> Self {
        Self {
            inner: Arc::new(Mutex::new(default)),
        }
    }

    fn get(&mut self) -> T {
        *self.inner.lock().unwrap()
    }

    fn set(&mut self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}

pub fn exec<F: FnOnce() -> () + Send + 'static>(f: F) {
    thread::spawn(f);
}

pub fn execute<F: Future<Output = ()> + Send + 'static>(f: F) {
    tokio::spawn(f);
}

pub fn execute_sync<F: FnOnce() -> () + Send + 'static>(f: F) {
    thread::spawn(move || f());
}
