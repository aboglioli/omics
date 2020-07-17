use regex::Regex;

use crate::common::error::Error;

pub struct Permission {
    module: String,
    permissions: String,
}

impl Permission {
    pub fn new(module: &str, permissions: &str) -> Result<Permission, Error> {
        let mut err = Error::application();
        if module.is_empty() {
            err.add_context("module", "empty");
        }

        if permissions.is_empty() {
            err.add_context("permissions", "empty");
        }

        let permissions = permissions.to_uppercase();

        match Regex::new(r"^[CRUD]+$") {
            Ok(re) => {
                if !re.is_match(&permissions) {
                    err.add_context("permissions", "invalid_characters");
                }
            }
            Err(e) => {
                err.wrap_raw(e);
            }
        }

        if err.has_context() {
            return Err(err);
        }

        let mut chars: Vec<char> = permissions.to_uppercase().chars().collect();
        chars.sort();
        chars.dedup();
        let permissions: String = chars.into_iter().collect();

        Ok(Permission {
            module: String::from(module),
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
