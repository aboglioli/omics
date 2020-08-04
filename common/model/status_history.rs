use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StatusItem<S, M> {
    date: DateTime<Utc>,
    meta: Option<M>,
    status: S,
}

impl<S, M> StatusItem<S, M> {
    pub fn new(status: S) -> Self {
        StatusItem {
            date: Utc::now(),
            meta: None,
            status,
        }
    }

    pub fn new_with_meta(status: S, meta: M) -> Self {
        StatusItem {
            date: Utc::now(),
            meta: Some(meta),
            status,
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn meta(&self) -> Option<&M> {
        self.meta.as_ref()
    }

    pub fn status(&self) -> &S {
        &self.status
    }
}

#[derive(Debug, Clone)]
pub struct StatusHistory<S, M> {
    history: Vec<StatusItem<S, M>>,
}

impl<S, M> StatusHistory<S, M> {
    pub fn new() -> Self {
        StatusHistory {
            history: Vec::new(),
        }
    }

    pub fn init(status: S) -> Self {
        let mut sh = StatusHistory::new();
        sh.add_status(status);
        sh
    }

    pub fn add_status(&mut self, status: S) {
        self.history.push(StatusItem::new(status));
    }

    pub fn add_status_with_meta(&mut self, status: S, meta: M) {
        self.history.push(StatusItem::new_with_meta(status, meta));
    }

    pub fn history(&self) -> &[StatusItem<S, M>] {
        &self.history
    }

    pub fn current(&self) -> Option<&StatusItem<S, M>> {
        self.history.last()
    }

    pub fn is_current<P>(&self, predicate: P) -> bool
    where
        P: Fn(&S) -> bool,
    {
        if let Some(current) = self.current() {
            return predicate(current.status());
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Status {
        Open,
        Closed,
    }

    #[test]
    fn create() {
        assert_eq!(StatusHistory::<Status, ()>::new().history().len(), 0);
        assert!(StatusHistory::<Status, ()>::new().current().is_none());

        let mut sh: StatusHistory<_, ()> = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        sh.add_status(Status::Open);

        assert_eq!(sh.history().len(), 3);

        let sh: StatusHistory<_, ()> = StatusHistory::init(Status::Open);
        assert_eq!(sh.history().len(), 1);
        assert_eq!(sh.current().unwrap().status(), &Status::Open);
    }

    #[test]
    fn history() {
        let mut sh: StatusHistory<_, ()> = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);
        sh.add_status(Status::Open);
        sh.add_status(Status::Closed);

        assert_eq!(sh.current().unwrap().status(), &Status::Closed);
    }

    #[test]
    fn meta() {
        let mut sh = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status_with_meta(Status::Closed, "invalid");
        sh.add_status_with_meta(Status::Open, "revalid");

        let history = sh.history();
        assert_eq!(history.len(), 3);
        assert!(history[0].meta().is_none());
        assert_eq!(history[1].meta().unwrap(), &"invalid");
        assert_eq!(history[2].meta().unwrap(), &"revalid");
    }

    #[test]
    fn compare() {
        let mut sh = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status_with_meta(Status::Closed, "invalid");
        sh.add_status_with_meta(Status::Open, "revalid");

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
