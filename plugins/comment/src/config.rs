use std::collections::HashMap;

use once_cell::sync::Lazy;

static GLOBAL_CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("c".to_string(), "//".to_string());
    map.insert("sh".to_string(), "#".to_string());
    map.insert("cpp".to_string(), "//".to_string());
    map.insert("lua".to_string(), "--".to_string());
    map.insert("sql".to_string(), "--".to_string());
    map.insert("rust".to_string(), "//".to_string());
    map
});


pub fn comment_string(filetype: String) -> Option<String> {
    GLOBAL_CONFIG.get(filetype.as_str()).map(|x| x.clone())
}
