use regex::Regex;

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Permission {
    module: String,
    permissions: String,
}

impl Permission {
    pub fn new<S: Into<String>>(module: S, permissions: S) -> Result<Self> {
        let module = module.into();
        let permissions = permissions.into();

        let mut err = Error::new("permission", "invalid");
        if module.is_empty() {
            err = err.add_context("module", "empty");
        }

        if permissions.is_empty() {
            err = err.add_context("permissions", "empty");
        }

        let permissions = permissions.to_uppercase();

        match Regex::new(r"^[CRUD]+$") {
            Ok(re) => {
                if !re.is_match(&permissions) {
                    err = err.add_context("permissions", "invalid_characters");
                }
            }
            Err(e) => {
                err = err.wrap_raw(e);
            }
        }

        if err.has_context() {
            return Err(err);
        }

        let mut chars: Vec<char> = permissions.to_uppercase().chars().collect();
        chars.sort_unstable();
        chars.dedup();
        let permissions: String = chars.into_iter().collect();

        Ok(Permission {
            module,
            permissions,
        })
    }

    pub fn module(&self) -> &str {
        &self.module
    }

    pub fn permissions(&self) -> &str {
        &self.permissions
    }

    pub fn contains(&self, permissions: &str) -> bool {
        let permissions = permissions.to_uppercase();
        for p in permissions.chars() {
            if !self.permissions.contains(p) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {

    use common::result::Result;

    use super::*;

    #[test]
    fn create_permissions() -> Result<()> {
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
    fn permission_contains() -> Result<()> {
        let p = Permission::new("mod", "CRD")?;
        assert!(p.contains("C"));
        assert!(p.contains("R"));
        assert!(p.contains("CD"));
        assert!(p.contains("CrD"));
        assert!(!p.contains("CRDA"));
        assert!(!p.contains("Ca"));

        Ok(())
    }
}
