use unicode_segmentation::UnicodeSegmentation;

//A new type to validate the subscriber name.
//We don't need to keep validating it once the name is in this type
// Type-Drive Development =)
#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName{

    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names.
    /// It panics otherwise.
    pub fn parse (s:String) -> Result<SubscriberName, String>{
        //Validation:
        //Can't be empty or whitespaces
        //Max 256 characters (graphemes to be internacional) long
        //Can't have the following character '/', '(', ')', '"', '<', '>', '\', '{', '}'

        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contain_forbidden_character = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contain_forbidden_character{
            Err(format!("{} is not a valid subscriber name.", s))
        } else{
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName{
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests{
    use crate::domain::SubscriberName;
    use claim::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_log_name_is_valid(){
        let name = "a̐".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_then_256_graphemes_is_rejected(){
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn whitespace_only_name_is_rejected(){
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn a_empty_string_is_rejected(){
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}