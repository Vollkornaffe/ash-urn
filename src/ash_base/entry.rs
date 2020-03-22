pub struct Entry(ash::Entry);

impl Entry {
    pub fn new() -> Result<Self, ash::LoadingError> {
        Ok(Self(ash::Entry::new()?))
    }
}
