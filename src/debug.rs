pub async fn debug() -> String {
    let timestamp = std::fs::read_to_string("./store/utc.txt").unwrap_or_default();
    return timestamp
}
