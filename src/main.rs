mod common;
mod identity;
mod publishing;

use crate::common::error::Error;

#[derive(Debug)]
struct Omics {
    omics: bool,
}

impl Default for Omics {
    fn default() -> Omics {
        Omics { omics: true }
    }
}

fn main() -> Result<(), Error> {
    println!(
        "{:?}",
        Omics {
            ..Default::default()
        }
    );
    Ok(())
}
