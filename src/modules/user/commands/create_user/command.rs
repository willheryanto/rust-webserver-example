pub struct CreateUserCommand<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl<'a> CreateUserCommand<'a> {
    pub fn new(username: &'a str, password: &'a str) -> CreateUserCommand<'a> {
        CreateUserCommand { username, password }
    }
}
