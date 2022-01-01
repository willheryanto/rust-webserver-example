use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Title(String);

impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Title {
    pub fn parse(s: String) -> Result<Title, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            return Err(format!("Invalid movie title: {}", s));
        }

        Ok(Self(s))
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::movie::domain::value_objects::title::Title;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_title_is_vlaid() {
        let title = "a".repeat(256);
        assert_ok!(Title::parse(title));
    }

    #[test]
    fn a_256_grapheme_long_title_is_valid() {
        let title = "a̐".repeat(256);
        assert_ok!(Title::parse(title));
    }

    #[test]
    fn a_title_longer_than_256_graphemes_is_rejected() {
        let title = "a".repeat(257);
        assert_err!(Title::parse(title));
    }

    #[test]
    fn whitespace_only_titles_are_rejected() {
        let title = " ".to_string();
        assert_err!(Title::parse(title));
    }

    #[test]
    fn empty_string_is_rejected() {
        let title = "".to_string();
        assert_err!(Title::parse(title));
    }

    #[test]
    fn titles_containing_an_invalid_character_are_rejected() {
        for title in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let title = title.to_string();
            assert_err!(Title::parse(title));
        }
    }

    #[test]
    fn a_valid_title_is_parsed_successfully() {
        let title = "Venom The Movie".to_string();
        assert_ok!(Title::parse(title));
    }
}
