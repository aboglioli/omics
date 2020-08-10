use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StatusItem<S> {
    date: DateTime<Utc>,
    status: S,
}

impl<S> StatusItem<S> {
    fn new(status: S) -> Self {
        StatusItem {
            date: Utc::now(),
            status,
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn status(&self) -> &S {
        &self.status
    }
}

#[derive(Debug, Clone)]
pub struct StatusHistory<S> {
    history: Vec<StatusItem<S>>,
}

impl<S> StatusHistory<S> {
    pub fn new(status: S) -> Self {
        StatusHistory {
            history: vec![StatusItem::new(status)],
        }
    }

    pub fn add_status(&mut self, status: S) {
        self.history.push(StatusItem::new(status));
    }

    pub fn history(&self) -> &[StatusItem<S>] {
        &self.history
    }

    pub fn current(&self) -> &StatusItem<S> {
        // It's safe because history has at least one status. It's initialized with one status.
        self.history.last().unwrap()
    }

    pub fn is_current<P>(&self, predicate: P) -> bool
    where
        P: Fn(&S) -> bool,
    {
        predicate(self.current().status())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Status {
        Init,
        Open,
        Closed,
    }

    #[test]
    fn create() {
        assert_eq!(StatusHistory::new(Status::Init).history().len(), 1);
        assert_eq!(StatusHistory::new(Status::Init).history().len(), 1);

        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        sh.add_status(Status::Open);
        assert_eq!(sh.history().len(), 4);
        assert_eq!(sh.current().status(), &Status::Open);

        let sh = StatusHistory::new(Status::Open);
        assert_eq!(sh.history().len(), 1);
        assert_eq!(sh.current().status(), &Status::Open);
    }

    #[test]
    fn history() {
        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        assert_eq!(sh.current().status(), &Status::Closed);
    }

    #[test]
    fn compare() {
        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        sh.add_status(Status::Open);

        assert!(sh.is_current(|s| match s {
            Status::Open => true,
            _ => false,
        }));

        assert!(!sh.is_current(|s| match s {
            Status::Closed => true,
            _ => false,
        }));
    }
}
