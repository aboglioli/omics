use crate::domain::interaction::Stars;

#[derive(Debug, Clone)]
pub enum Kind {
    View,
    Reading,
    Like,
    Review { stars: Stars },
}
