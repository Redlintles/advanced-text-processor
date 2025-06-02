use super::BytecodeTokenMethods;

pub fn parse_token(token: &dyn BytecodeTokenMethods, input: &str) -> String {
    token.parse(input)
}
