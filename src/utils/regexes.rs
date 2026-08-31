use std::sync::LazyLock;

use regex::Regex;

pub static VAR_REF_RE: LazyLock<Regex> = LazyLock::new(||
    Regex::new(r"^\{\{([a-zA-Z][a-zA-Z0-9]*)\}\}$").expect(
        "var ref regex is a static valid pattern"
    )
);
