pub mod token_defs;

use token_defs::ate::Ate;
use token_defs::atb::Atb;
use token_defs::rtr::Rtr;
use token_defs::trs::Trs;
use token_defs::tls::Tls;
use token_defs::tbs::Tbs;
use token_defs::raw::Raw;
use token_defs::rfw::Rfw;
use token_defs::dlf::Dlf;
use token_defs::dll::Dll;
use token_defs::dlb::Dlb;
use token_defs::dla::Dla;
use token_defs::dlc::Dlc;
use token_defs::rtl::Rtl;
use token_defs::rpt::Rpt;

#[derive(Clone)]
pub enum AtpToken {
    Tbs(Tbs),
    Tls(Tls),
    Trs(Trs),
    Raw(Raw),
    Rfw(Rfw),
    Ate(Ate),
    Atb(Atb),
    Dlb(Dlb),
    Dla(Dla),
    Dlf(Dlf),
    Dll(Dll),
    Dlc(Dlc),
    Rtl(Rtl),
    Rtr(Rtr),
    Rpt(Rpt),
}
pub trait TokenMethods: Sized {
    fn token_from_vec_params(line: Vec<String>) -> Result<Self, String>;
    fn token_to_atp_line(&self) -> String;
    fn get_string_repr() -> String;
    fn new() -> Self;
    fn parse(&self, input: &str) -> String;
}
