pub fn token_array_not_found(identifier: &str) -> impl Fn() -> String {
    let message =
        format!("Token array not found, is {} a valid identifier for this processor?", identifier);
    move || message.clone()
}
