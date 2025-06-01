use crate::data::{ TokenMethods };

pub fn parse_token(token: &dyn TokenMethods, input: &str) -> String {
    token.parse(input)
}
