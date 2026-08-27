use uuid::Uuid;
use md5;

pub fn auth(username: &str) -> Uuid {
    let hash_input = format!("OfflinePlayer: {}", username);
    let digest = md5::compute(hash_input.as_bytes());
    let mut bytes = *digest;
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    Uuid::from_bytes(bytes)
}