use uuid::Uuid;

pub struct FindUserQuery<'a> {
    pub id: &'a Uuid,
}

impl<'a> FindUserQuery<'a> {
    pub fn new(id: &'a Uuid) -> FindUserQuery<'a> {
        FindUserQuery { id }
    }
}
