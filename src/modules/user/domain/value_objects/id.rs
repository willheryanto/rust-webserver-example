use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Id(pub Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.0.to_string()
    }
}
