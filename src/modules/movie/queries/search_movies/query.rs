pub struct SearchMoviesQuery<'a> {
    pub title: &'a str,
}

impl<'a> SearchMoviesQuery<'a> {
    pub fn new(title: &'a str) -> SearchMoviesQuery {
        SearchMoviesQuery { title }
    }
}
