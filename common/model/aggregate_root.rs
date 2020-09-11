use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct AggregateRoot<ID> {
    id: ID,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl<ID> AggregateRoot<ID> {
    pub fn new(id: ID) -> AggregateRoot<ID> {
        AggregateRoot {
            id,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }

    pub fn build(
        id: ID,
        created_at: DateTime<Utc>,
        updated_at: Option<DateTime<Utc>>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        AggregateRoot {
            id,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> &ID {
        &self.id
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> Option<&DateTime<Utc>> {
        self.updated_at.as_ref()
    }

    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> {
        self.deleted_at.as_ref()
    }

    pub fn update(&mut self) {
        self.updated_at = Some(Utc::now());
    }

    pub fn delete(&mut self) {
        self.deleted_at = Some(Utc::now());
    }
}

impl<ID: PartialEq> PartialEq for AggregateRoot<ID> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<ID: Clone> Clone for AggregateRoot<ID> {
    fn clone(&self) -> Self {
        AggregateRoot {
            id: self.id.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type AggRootID = String;

    #[derive(Debug)]
    struct AggRoot {
        base: AggregateRoot<AggRootID>,
    }

    impl AggRoot {
        fn new(id: AggRootID) -> AggRoot {
            AggRoot {
                base: AggregateRoot::new(id),
            }
        }

        fn base(&self) -> &AggregateRoot<AggRootID> {
            &self.base
        }

        fn base_mut(&mut self) -> &mut AggregateRoot<AggRootID> {
            &mut self.base
        }
    }

    #[test]
    fn create() {
        let e = AggRoot::new(AggRootID::from("AR_022"));
        assert_eq!(e.base().id(), "AR_022");
    }

    #[test]
    fn properties() {
        let mut e = AggRoot::new(AggRootID::from("AR_022"));
        assert_eq!(e.base().id(), "AR_022");
        assert!(e.base().created_at() < &Utc::now());
        assert!(e.base().updated_at().is_none());
        assert!(e.base().deleted_at().is_none());

        e.base_mut().update();
        assert!(e.base().updated_at().is_some());
        assert!(e.base().updated_at().unwrap() < &Utc::now());

        e.base_mut().delete();
        assert!(e.base().deleted_at().is_some());
        assert!(e.base().deleted_at().unwrap() < &Utc::now());
    }

    #[test]
    fn equals() {
        let ag1 = AggRoot::new(AggRootID::from("AR_101"));
        let ag2 = AggRoot::new(AggRootID::from("AR_101"));

        assert_eq!(ag1.base(), ag2.base());
    }
}
