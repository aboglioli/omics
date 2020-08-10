use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
pub struct CallsTracker<T> {
    calls: Mutex<HashMap<String, Vec<T>>>,
}

impl<T: Clone> CallsTracker<T> {
    pub fn new() -> Self {
        CallsTracker {
            calls: Mutex::new(HashMap::new()),
        }
    }

    pub fn register(&self, name: &str, v: T) {
        let mut calls = self.calls.lock().unwrap();
        if let Some(calls) = calls.get_mut(name) {
            calls.push(v);
            return;
        }

        calls.insert(name.to_owned(), vec![v]);
    }

    pub fn get(&self, name: &str) -> Vec<T> {
        let mut vec = Vec::new();

        if let Some(calls) = self.calls.lock().unwrap().get(name) {
            for v in calls.iter() {
                vec.push(v.clone());
            }
        }

        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let ct = CallsTracker::new();
        ct.register("arg1", "hi");
        ct.register("arg1", "bye");
        ct.register("arg2", "new");
        assert_eq!(ct.get("arg1").len(), 2);
        assert_eq!(ct.get("arg2").len(), 1);
        assert_eq!(ct.get("arg1")[0], "hi");
        assert_eq!(ct.get("arg1")[1], "bye");
        assert_eq!(ct.get("arg2")[0], "new");
    }
}
