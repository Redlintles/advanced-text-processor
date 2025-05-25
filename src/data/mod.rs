pub mod tokens;

use tokens::{ Ate, Atb, Tbs, Tls, Trs, Dla, Dlb, Dlc, Dlf, Dll, Raw, Rfw };

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
}
