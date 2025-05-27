use crate::data::{ AtpToken, TokenMethods };

pub fn parse_token(token: AtpToken, input: &str) -> String {
    match token {
        AtpToken::Atb(x) => x.parse(input),
        AtpToken::Tbs(x) => x.parse(input),
        AtpToken::Tls(x) => x.parse(input),
        AtpToken::Trs(x) => x.parse(input),
        AtpToken::Raw(x) => x.parse(input),
        AtpToken::Rfw(x) => x.parse(input),
        AtpToken::Ate(x) => x.parse(input),
        AtpToken::Dlb(x) => x.parse(input),
        AtpToken::Dla(x) => x.parse(input),
        AtpToken::Dlf(x) => x.parse(input),
        AtpToken::Dll(x) => x.parse(input),
        AtpToken::Dlc(x) => x.parse(input),
        AtpToken::Rtl(x) => x.parse(input),
        AtpToken::Rtr(x) => x.parse(input),
        AtpToken::Rpt(x) => x.parse(input),
    }
}
