use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusItem<S> {
    #[serde(flatten)]
    status: S,
    datetime: DateTime<Utc>,
}

impl<S> StatusItem<S> {
    pub fn new(status: S) -> Self {
        StatusItem {
            status,
            datetime: Utc::now(),
        }
    }

    pub fn build(status: S, datetime: DateTime<Utc>) -> Self {
        StatusItem { status, datetime }
    }

    pub fn datetime(&self) -> &DateTime<Utc> {
        &self.datetime
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

    pub fn build(history: Vec<StatusItem<S>>) -> Self {
        StatusHistory { history }
    }

    pub fn add_status(&mut self, status: S) {
        self.history.push(StatusItem::new(status));
    }

    pub fn history(&self) -> &[StatusItem<S>] {
        &self.history
    }

    pub fn current_item(&self) -> &StatusItem<S> {
        // It's safe because history has at least one status. It's initialized with one status.
        self.history.last().unwrap()
    }

    pub fn current(&self) -> &S {
        self.current_item().status()
    }

    pub fn is_current<P>(&self, predicate: P) -> bool
    where
        P: Fn(&S) -> bool,
    {
        predicate(self.current())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    use serde_json::json;

    use crate::model::StringId;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Comment {
        comment: String,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "status")]
    enum Status {
        #[serde(rename = "init")]
        Init,
        #[serde(rename = "open")]
        Open { prop1: String, prop2: u32 },
        #[serde(rename = "closed")]
        Closed { prop1: bool },
        #[serde(rename = "important")]
        Important {
            user_id: StringId,
            product_id: StringId,
            comment: Comment,
        },
    }

    #[test]
    fn create() {
        assert_eq!(StatusHistory::new(Status::Init).history().len(), 1);
        assert_eq!(StatusHistory::new(Status::Init).history().len(), 1);

        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open {
            prop1: "one".to_owned(),
            prop2: 32,
        });
        sh.add_status(Status::Closed { prop1: true });
        sh.add_status(Status::Open {
            prop1: "two".to_owned(),
            prop2: 64,
        });
        assert_eq!(sh.history().len(), 4);
        assert_eq!(
            sh.current(),
            &Status::Open {
                prop1: "two".to_owned(),
                prop2: 64
            }
        );

        let sh = StatusHistory::new(Status::Open {
            prop1: "three".to_owned(),
            prop2: 125,
        });
        assert_eq!(sh.history().len(), 1);
        assert_eq!(
            sh.current(),
            &Status::Open {
                prop1: "three".to_owned(),
                prop2: 125
            }
        );
    }

    #[test]
    fn history() {
        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open {
            prop1: "one".to_owned(),
            prop2: 1,
        });
        sh.add_status(Status::Closed { prop1: false });
        sh.add_status(Status::Open {
            prop1: "two".to_owned(),
            prop2: 2,
        });
        sh.add_status(Status::Closed { prop1: true });
        assert_eq!(sh.current(), &Status::Closed { prop1: true });
    }

    #[test]
    fn compare() {
        let mut sh = StatusHistory::new(Status::Init);
        sh.add_status(Status::Open {
            prop1: "one".to_owned(),
            prop2: 1,
        });
        sh.add_status(Status::Closed { prop1: true });
        sh.add_status(Status::Open {
            prop1: "two".to_owned(),
            prop2: 2,
        });

        assert!(sh.is_current(|s| match s {
            Status::Open { .. } => true,
            _ => false,
        }));

        assert!(!sh.is_current(|s| match s {
            Status::Closed { .. } => true,
            _ => false,
        }));
    }

    #[test]
    fn serialize() {
        let sh = StatusHistory::build(vec![
            StatusItem::build(
                Status::Init,
                DateTime::from_str("2020-05-01T12:30:00Z").unwrap(),
            ),
            StatusItem::build(
                Status::Open {
                    prop1: "one".to_owned(),
                    prop2: 1,
                },
                DateTime::from_str("2020-05-01T13:30:00Z").unwrap(),
            ),
            StatusItem::build(
                Status::Closed { prop1: true },
                DateTime::from_str("2020-05-01T14:30:00Z").unwrap(),
            ),
            StatusItem::build(
                Status::Open {
                    prop1: "two".to_owned(),
                    prop2: 2,
                },
                DateTime::from_str("2020-05-01T15:30:00Z").unwrap(),
            ),
            StatusItem::build(
                Status::Important {
                    user_id: StringId::new("ID-u01").unwrap(),
                    product_id: StringId::new("ID-p02").unwrap(),
                    comment: Comment {
                        comment: "Excellent!".to_owned(),
                    },
                },
                DateTime::from_str("2020-05-02T03:45:00Z").unwrap(),
            ),
        ]);

        let res = serde_json::to_value(sh.history()).unwrap();
        let expected = json!([{
            "status": "init",
            "datetime": "2020-05-01T12:30:00Z",
        }, {
            "status": "open",
            "prop1": "one",
            "prop2": 1,
            "datetime": "2020-05-01T13:30:00Z",
        }, {
            "status": "closed",
            "prop1": true,
            "datetime": "2020-05-01T14:30:00Z",
        }, {
            "status": "open",
            "prop1": "two",
            "prop2": 2,
            "datetime": "2020-05-01T15:30:00Z",
        }, {
            "status": "important",
            "user_id": { "id": "ID-u01" },
            "product_id": { "id": "ID-p02" },
            "comment": { "comment": "Excellent!" },
            "datetime": "2020-05-02T03:45:00Z",
        }]);

        assert_eq!(
            res,
            expected,
            "{}",
            serde_json::to_string(sh.history()).unwrap(),
        );
    }

    #[test]
    fn deserialize() {
        let items: Vec<StatusItem<Status>> = serde_json::from_str(
            r#"[{
            "status": "init",
            "datetime": "2020-05-01T12:30:00Z"
        }, {
            "status": "open",
            "prop1": "one",
            "prop2": 1,
            "datetime": "2020-05-01T13:30:00Z"
        }, {
            "status": "closed",
            "prop1": true,
            "datetime": "2020-05-01T14:30:00Z"
        }, {
            "status": "open",
            "prop1": "two",
            "prop2": 2,
            "datetime": "2020-05-01T15:30:00-01:00"
        }, {
            "status": "important",
            "user_id": { "id": "ID-u01" },
            "product_id": { "id": "ID-p02" },
            "comment": { "comment": "Excellent!" },
            "datetime": "2020-05-02T03:45:00Z"
        }]"#,
        )
        .unwrap();

        assert_eq!(items.len(), 5);
        assert_eq!(items[0].status(), &Status::Init);
        assert_eq!(
            items[1].status(),
            &Status::Open {
                prop1: "one".to_owned(),
                prop2: 1,
            },
        );
        assert_eq!(items[2].status(), &Status::Closed { prop1: true },);
        assert_eq!(
            items[2].datetime().to_rfc3339(),
            "2020-05-01T14:30:00+00:00"
        );
        assert_eq!(
            items[3].status(),
            &Status::Open {
                prop1: "two".to_owned(),
                prop2: 2,
            },
        );
        assert_eq!(
            items[3].datetime().to_rfc3339(),
            "2020-05-01T16:30:00+00:00"
        );
        assert_eq!(
            items[4].status(),
            &Status::Important {
                user_id: StringId::new("ID-u01").unwrap(),
                product_id: StringId::new("ID-p02").unwrap(),
                comment: Comment {
                    comment: "Excellent!".to_owned(),
                },
            },
        );
        assert_eq!(
            items[4].datetime().to_rfc3339(),
            "2020-05-02T03:45:00+00:00"
        );
    }
}
