use super::model::*;

struct FakeEntityWithIntegerID {
    id: ID<i32>,
}

impl Entity<i32> for FakeEntityWithIntegerID {
    fn id(&self) -> &ID<i32> {
        &self.id
    }
}

struct FakeEntityWithStringID {
    uuid: ID<String>,
}

impl Entity<String> for FakeEntityWithStringID {
    fn id(&self) -> &ID<String> {
        &self.uuid
    }
}

#[test]
fn get_id() {
    let entity = FakeEntityWithIntegerID { id: ID::new(32) };
    assert_eq!(entity.id, ID::new(32));
    assert_eq!(entity.id(), &ID::new(32));

    let entity = FakeEntityWithStringID {
        uuid: ID::new("U002".to_string()),
    };
    assert_eq!(entity.uuid, ID::new("U002".to_string()));
    assert_eq!(entity.id(), &ID::new("U002".to_string()));
}

#[test]
fn equals() {
    let e1 = FakeEntityWithStringID {
        uuid: ID::new("U001".to_string()),
    };
    let e2 = FakeEntityWithStringID {
        uuid: ID::new("U001".to_string()),
    };
    let e3 = FakeEntityWithStringID {
        uuid: ID::new("U002".to_string()),
    };
    assert!(e1.eq(&e2));
    assert_eq!(e1.eq(&e3), false);
    assert!(e1.eq_id("U001".to_string()));
    assert!(!e1.eq_id("U002".to_string()));
}
