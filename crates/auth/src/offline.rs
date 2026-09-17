use uuid::Uuid;

pub fn offline(username: &str) -> Uuid {
    let hash = format!("Offline:{}", username);
    let digest = md5::compute(hash.as_bytes());
    let mut bytes = *digest;
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}