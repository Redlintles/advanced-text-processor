use std::sync::LazyLock;

use regex::Regex;

pub static VAR_REF_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\{\{([a-zA-Z][a-zA-Z0-9]*)\}\}$").expect(
        "VAR_REF regex is a static valid pattern"
    )
});
pub static EXPR_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[\[(.*?)\]\]").expect("EXPR regex is a static valid pattern")
});
