use std::cmp::PartialEq;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct StatusItem<S, M> {
    date: DateTime<Utc>,
    motive: Option<M>,
    status: S,
}

impl<S: PartialEq, M> StatusItem<S, M> {
    pub fn new(status: S) -> Self {
        StatusItem {
            date: Utc::now(),
            motive: None,
            status,
        }
    }

    pub fn new_with_motive(status: S, motive: M) -> Self {
        StatusItem {
            date: Utc::now(),
            motive: Some(motive),
            status,
        }
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }

    pub fn motive(&self) -> Option<&M> {
        self.motive.as_ref()
    }

    pub fn status(&self) -> &S {
        &self.status
    }

    pub fn is(&self, status: &S) -> bool {
        &self.status == status
    }
}

#[derive(Debug, Clone)]
pub struct StatusHistory<S, M> {
    history: Vec<StatusItem<S, M>>,
}

impl<S: PartialEq, M> StatusHistory<S, M> {
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

    pub fn add_status_with_motive(&mut self, status: S, motive: M) {
        self.history
            .push(StatusItem::new_with_motive(status, motive));
    }

    pub fn history(&self) -> &[StatusItem<S, M>] {
        &self.history
    }

    pub fn current(&self) -> Option<&StatusItem<S, M>> {
        self.history.last()
    }

    pub fn is_current_any(&self, statuses: &[&S]) -> bool {
        match self.current() {
            Some(status_item) if statuses.iter().any(|s| status_item.is(s)) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    enum Status {
        Open,
        Closed,
        Cancelled,
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
    fn motive() {
        let mut sh = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status_with_motive(Status::Closed, "invalid");
        sh.add_status_with_motive(Status::Open, "revalid");

        let history = sh.history();
        assert_eq!(history.len(), 3);
        assert!(history[0].motive().is_none());
        assert_eq!(history[1].motive().unwrap(), &"invalid");
        assert_eq!(history[2].motive().unwrap(), &"revalid");
    }

    #[test]
    fn compare() {
        let mut sh = StatusHistory::new();
        sh.add_status(Status::Open);
        sh.add_status_with_motive(Status::Closed, "invalid");
        sh.add_status_with_motive(Status::Open, "revalid");

        assert!(sh.is_current_any(&[&Status::Open]));
        assert!(!sh.is_current_any(&[&Status::Closed]));
    }
}
