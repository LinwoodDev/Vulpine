#[allow(dead_code)]
pub fn unique_name(name: &str, exist: impl Fn(&String) -> bool) -> String {
    let mut key = name.to_string();
    let mut count = 0;
    while exist(&key) {
        count += 1;
        key = format!("{} ({})", name, count);
    }
    key
}

#[allow(dead_code)]
pub fn unique_name_from_list(name: &str, list: &Vec<&String>) -> String {
    unique_name(name, |key| list.contains(&key))
}
