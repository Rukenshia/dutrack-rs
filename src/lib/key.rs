use std::fmt;
use lib::uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Key {
  uuid: Uuid,
}

impl Key {
  pub fn new() -> Self {
    let key = Uuid::new_v4();
    
    Key {
      uuid: key
    }
  }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}