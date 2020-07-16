use crate::common::error::Error;

use super::*;

#[test]
fn create_permissions() -> Result<(), Error> {
    assert!(Permission::new("", "").is_err());
    assert!(Permission::new("module", "").is_err());
    assert!(Permission::new("module", "A").is_err());
    assert!(Permission::new("module", "ASD").is_err());
    assert!(Permission::new("module", "CRUX").is_err());
    assert!(Permission::new("module", "CRUD").is_ok());
    assert!(Permission::new("module", "cRuD").is_ok());
    assert!(Permission::new("module", "crudCCrR").is_ok());

    let p = Permission::new("module", "cCruD")?;
    assert_eq!(p.module(), "module");
    assert_eq!(p.permissions(), "CDRU");

    Ok(())
}

#[test]
fn permission_contains() -> Result<(), Error> {
    let p = Permission::new("mod", "CRD")?;
    assert!(p.contains("C"));
    assert!(p.contains("R"));
    assert!(p.contains("CD"));
    assert!(p.contains("CrD"));
    assert!(!p.contains("CRDA"));
    assert!(!p.contains("Ca"));

    Ok(())
}
