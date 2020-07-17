use crate::common::error::Error;
use crate::common::model::{Entity, ID};

use super::*;

#[test]
fn create_role() -> Result<(), Error> {
    let r = Role::new(RoleID::from("admin"), "Administrator")?;
    assert_eq!(r.id(), &ID::new(RoleID::from("admin")));
    assert_eq!(r.name(), "Administrator");
    assert!(r.eq_id(RoleID::from("admin")));

    Ok(())
}

#[test]
fn permissions() -> Result<(), Error> {
    let pmod1 = Permission::new("mod1", "CRUD")?;
    let pmod2 = Permission::new("mod2", "CRD")?;
    let pmod3 = Permission::new("mod3", "R")?;
    let mut r = Role::new(RoleID::from("user"), "User")?;
    r.add_permissions(pmod1);
    r.add_permissions(pmod2);
    r.add_permissions(pmod3);
    assert!(r.has_permissions("mod1", "cD"));
    assert!(r.has_permissions("mod1", "crud"));
    assert!(r.has_permissions("mod2", "Cd"));
    assert!(!r.has_permissions("mod2", "CdU"));
    assert!(!r.has_permissions("mod3", "C"));
    assert!(r.has_permissions("mod3", "r"));

    let pmod4 = Permission::new("mod3", "c")?;
    r.add_permissions(pmod4);
    assert!(r.has_permissions("mod3", "C"));

    Ok(())
}
