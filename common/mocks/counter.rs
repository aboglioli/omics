use std::cell::RefCell;
use std::collections::HashMap;

pub struct Counter {
    counts: RefCell<HashMap<String, u32>>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            counts: RefCell::new(HashMap::new()),
        }
    }

    pub fn inc(&self, name: &str) {
        let mut counts = self.counts.borrow_mut();
        if let Some(count) = counts.get_mut(name) {
            *count += 1;
            return;
        }
        counts.insert(name.to_owned(), 1);
    }

    pub fn count(&self, name: &str) -> u32 {
        if let Some(count) = self.counts.borrow().get(name) {
            *count
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let c = Counter::new();
        c.inc("c1");
        c.inc("c2");
        c.inc("c1");
        c.inc("c1");
        c.inc("c3");
        c.inc("c1");
        c.inc("c3");
        assert_eq!(c.count("c1"), 4);
        assert_eq!(c.count("c2"), 1);
        assert_eq!(c.count("c3"), 2);
        assert_eq!(c.count("c4"), 0);
    }
}
