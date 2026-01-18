#[macro_export]
macro_rules! parse_args {
    ($params:expr, $idx:expr, String, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match &$params[$idx] {
            AtpParamTypes::String(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($params:expr, $idx:expr, Usize, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match &$params[$idx] {
            AtpParamTypes::Usize(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($params:expr, $idx:expr, Token, $msg:expr) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match &$params[$idx] {
            AtpParamTypes::Token(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters($msg.into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($param:expr, String) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match &$param {
            AtpParamTypes::String(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters("Param must be of type Sring".into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($param:expr, Usize) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match $param {
            AtpParamTypes::Usize(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters("Param must be of type Usize".into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
    ($param:expr, Token) => {
        {
        use crate::utils::params::AtpParamTypes;
        use crate::utils::errors::{AtpError, AtpErrorCode};
        match $param {
            AtpParamTypes::Token(payload) => payload.clone(),
            _ => {
                return Err(AtpError::new(
                    AtpErrorCode::InvalidParameters("Param must be of type Token".into()),
                    "",
                    "",
                ));
            }
        }
        }
    };
}
