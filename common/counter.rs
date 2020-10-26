use std::collections::HashMap;

#[derive(Default)]
pub struct Counter {
    counts: HashMap<String, usize>,
}

impl Counter {
    pub fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn inc<S: Into<String>>(&mut self, name: S) {
        let name = name.into();
        if let Some(count) = self.counts.get_mut(&name) {
            *count += 1;
            return;
        }
        self.counts.insert(name, 1);
    }

    pub fn dec<S: Into<String>>(&mut self, name: S) {
        let name = name.into();
        if let Some(count) = self.counts.get_mut(&name) {
            if count > &mut 0 {
                *count -= 1;
            }
        }
    }

    pub fn count(&self, name: &str) -> usize {
        if let Some(count) = self.counts.get(name) {
            *count
        } else {
            0
        }
    }

    pub fn into(self) -> HashMap<String, usize> {
        self.counts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut c = Counter::new();
        c.inc("c1");
        c.inc("c2");
        c.inc("c1");
        c.inc("c1");
        c.inc("c3");
        c.inc("c1");
        c.inc("c3");
        c.inc("c5");
        c.dec("c5");
        c.dec("c5");
        assert_eq!(c.count("c1"), 4);
        assert_eq!(c.count("c2"), 1);
        assert_eq!(c.count("c3"), 2);
        assert_eq!(c.count("c4"), 0);
        assert_eq!(c.count("c5"), 0);
    }
}
