use chrono::{DateTime, Duration, Utc};

use common::error::Error;
use common::result::Result;

#[derive(Debug, Clone)]
pub struct Birthdate {
    date: DateTime<Utc>,
}

impl Birthdate {
    pub fn new(date: DateTime<Utc>) -> Result<Self> {
        let now = Utc::now();
        let diff = now - date;

        if diff > Duration::weeks(6240) {
            return Err(Error::new("birthdate", "too_old"));
        }

        if diff < Duration::weeks(728) {
            return Err(Error::new("birthdate", "too_young"));
        }

        Ok(Birthdate { date })
    }

    pub fn from_str(s: &str) -> Result<Self> {
        Self::new(
            DateTime::parse_from_rfc3339(s)
                .map(|datetime| DateTime::<Utc>::from(datetime))
                .map_err(|err| Error::bad_format("birthdate").wrap_raw(err))?,
        )
    }

    pub fn date(&self) -> &DateTime<Utc> {
        &self.date
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn too_old() {
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339("1886-12-19T16:39:57+00:00")
            .unwrap()
            .into();
        assert!(Birthdate::new(date).is_err());
    }

    #[test]
    fn too_young() {
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339("2015-12-19T16:39:57+00:00")
            .unwrap()
            .into();
        assert!(Birthdate::new(date).is_err());
    }

    #[test]
    fn from_str() {
        assert!(Birthdate::from_str("1994-12-32T16:39:57+00:00").is_err());
        assert!(Birthdate::from_str("1994-12-20T16:61:57+00:00").is_err());
        assert!(Birthdate::from_str("1994-12-20T16:39:57-03:00").is_ok());
    }
}
