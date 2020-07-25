use crate::error::Error;

pub trait Transaction {
    fn begin(&mut self) -> Option<Error>;
    fn commit(&mut self) -> Option<Error>;
    fn rollback(&mut self) -> Option<Error>;
}
