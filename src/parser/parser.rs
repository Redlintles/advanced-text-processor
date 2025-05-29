use crate::data::{ TokenMethods };

pub fn parse_token(token: impl TokenMethods, input: &str) -> String {
    return token.parse(input);
}
