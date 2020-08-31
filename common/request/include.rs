use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncludeParams {
    pub include: Option<String>,
}

impl From<IncludeParams> for Option<String> {
    fn from(params: IncludeParams) -> Self {
        params.include
    }
}

pub struct Include {
    fields: HashMap<String, bool>,
}

impl Include {
    pub fn new(include: &str) -> Self {
        let mut fields = HashMap::new();

        let field_strs: Vec<String> = include
            .split(",")
            .map(|field| field.trim().to_lowercase())
            .filter(|field| !field.is_empty())
            .collect();

        for field in field_strs.into_iter() {
            fields.insert(field, true);
        }

        Include { fields }
    }

    pub fn has(&self, field: &str) -> bool {
        self.fields.get(field).is_some()
    }

    pub fn add<S: Into<String>>(mut self, field: S) -> Self {
        let field = field.into().trim().to_lowercase();

        if !field.is_empty() {
            self.fields.insert(field.into(), true);
        }

        self
    }
}

impl Default for Include {
    fn default() -> Self {
        Include {
            fields: HashMap::new(),
        }
    }
}

impl From<&str> for Include {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<Option<String>> for Include {
    fn from(s: Option<String>) -> Self {
        if let Some(s) = s {
            Self::new(&s)
        } else {
            Self::default()
        }
    }
}

impl From<IncludeParams> for Include {
    fn from(q: IncludeParams) -> Self {
        Self::from(q.include)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let include = Include::new("users,roles,name");
        assert_eq!(include.fields.len(), 3);
        assert!(include.has("users"));
        assert!(include.has("roles"));
        assert!(include.has("name"));
        assert!(!include.has("non-existing"));
    }

    #[test]
    fn none_include() {
        let include = Include::from(None);
        assert_eq!(include.fields.len(), 0);
        assert!(!include.has("users"));
        assert!(!include.has("roles"));
    }

    #[test]
    fn with_withespace() {
        let include = Include::from(Some(" ,  users, roles ,name ".to_owned()));
        assert_eq!(include.fields.len(), 3);
        assert!(include.has("users"));
        assert!(include.has("roles"));
        assert!(include.has("name"));
        assert!(!include.has(" roles "));
    }

    #[test]
    fn to_lowercase() {
        let include = Include::new(" ,  useRS, rOLes ,nAMe ");
        assert_eq!(include.fields.len(), 3);
        assert!(include.has("users"));
        assert!(include.has("roles"));
        assert!(include.has("name"));
    }
}
