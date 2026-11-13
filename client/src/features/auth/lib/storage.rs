pub const TOKEN_KEY: &str = "access_token";

pub fn save_token(token: &str) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.set_item(TOKEN_KEY, token);
        }
    }
}

pub fn get_token() -> Option<String> {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item(TOKEN_KEY).ok().flatten())
}

pub fn remove_token() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item(TOKEN_KEY);
        }
    }
}
