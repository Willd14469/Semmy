const PREFIX: &str = "svh";

fn prefix_key(key: &str) -> String {
    format!("{PREFIX}_{key}")
}

pub fn set_bool(key: &str, value: bool) {
    let prefixed_key = prefix_key(key);

    match value {
        true => {
            std::env::set_var(prefixed_key, "true");
        }
        false => {
            std::env::set_var(prefixed_key, "false");
        }
    }
}

pub fn get_bool(key: &str) -> Option<bool> {
    let prefixed_key = prefix_key(key);

    let result = std::env::var(prefixed_key).ok()?;

    match result.as_str() {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

pub fn get_bool_with_default(key: &str, default: bool) -> bool {
    let prefixed_key = prefix_key(key);

    let result_opt = std::env::var(prefixed_key).ok();

    if result_opt.is_none() {
        return default;
    }

    let result = result_opt.unwrap();

    match result.as_str() {
        "true" => true,
        "false" => false,
        _ => default,
    }
}
