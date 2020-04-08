use crate::UrnError;

pub struct Entry(pub ash::Entry);

impl Entry {
    pub fn new() -> Result<Self, UrnError> {
        Ok(Self(ash::Entry::new()?))
    }
}
