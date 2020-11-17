pub mod application;
pub mod container;
pub mod domain;
pub mod infrastructure;
pub mod mocks;

pub type UserIdAndRole = (domain::user::UserId, domain::role::Role);
