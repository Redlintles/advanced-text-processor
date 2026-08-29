use std::{ fs::OpenOptions, io::{ BufRead, BufReader }, path::Path };

use crate::{
    globals::{
        table::{ QuerySource, QueryTarget, TOKEN_TABLE, TargetValue },
        var::{ TokenWrapper },
    },
    utils::{
        errors::{ TextForgeError, TextForgeErrorCode },
        params::TextForgeParamTypes,
        validations::check_file_path,
    },
};

pub fn read_from_text(token_string: &str) -> Result<TokenWrapper, TextForgeError> {
    let line = token_string.trim();

    if token_string.is_empty() || token_string.starts_with("//") {
        return Ok(TokenWrapper::default());
    }
    let chunks = match
        shell_words::split(
            &line
                .trim_end()
                .strip_suffix(";")
                .ok_or_else(|| {
                    TextForgeError::new(
                        TextForgeErrorCode::TextParsingError(
                            "An ATP Parsing error ocurred: Error splitting file line".into()
                        ),
                        "shell words split",
                        token_string.to_string()
                    )
                })?
        )
    {
        Ok(x) => x,
        Err(_) => {
            return Err(
                TextForgeError::new(
                    TextForgeErrorCode::TextParsingError(
                        "An ATP Parsing error ocurred: Error splitting file line".into()
                    ),
                    "shell words split",
                    token_string.to_string()
                )
            );
        }
    };

    let token_query = TOKEN_TABLE.find((
        QuerySource::Identifier(chunks[0].clone().into()),
        QueryTarget::Token,
    ))?;

    let token_param_types = match
        TOKEN_TABLE.find((QuerySource::Identifier(chunks[0].clone().into()), QueryTarget::Syntax))?
    {
        TargetValue::Syntax(p) => p,
        _ => unreachable!(" Invalid Query result"),
    };

    match token_query {
        TargetValue::Token(token_ref) => {
            let token = token_ref.into_box();

            let parsed_params = TextForgeParamTypes::from_expected(
                token_param_types,
                &chunks[1..]
            )?;

            let wrapper = TokenWrapper::new(token, Some(parsed_params));

            Ok(wrapper)
        }
        _ => unreachable!("Invalid query result!"),
    }
}

pub fn read_from_file(path: &Path) -> Result<Vec<TokenWrapper>, TextForgeError> {
    check_file_path(path, Some("textforge"))?;
    let mut result = Vec::new();

    let file = match OpenOptions::new().read(true).open(path) {
        Ok(x) => x,
        Err(_) => {
            return Err(
                TextForgeError::new(
                    crate::utils::errors::TextForgeErrorCode::FileOpeningError(
                        "Failed opening File".into()
                    ),
                    "",
                    format!("{:?}", path)
                )
            );
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                let trimmed = l.trim();
                if trimmed.is_empty() || trimmed.starts_with("//") {
                    continue;
                }
                result.push(read_from_text(&l)?);
            }
            Err(_) => {
                return Err(
                    TextForgeError::new(
                        crate::utils::errors::TextForgeErrorCode::FileReadingError(
                            "Failed reading file line".into()
                        ),
                        "",
                        ""
                    )
                );
            }
        }
    }

    Ok(result)
}
