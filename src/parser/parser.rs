use crate::data::{ TokenMethods };

pub fn parse_token(token: &Box<dyn TokenMethods>, input: &str) -> String {
    return token.parse(input);
}
