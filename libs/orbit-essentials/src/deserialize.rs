/// Efficient deserializer for `Option<Vec<u8>>` using `serde_bytes::ByteBuf` internally
/// to speed up deserialization.
pub fn deserialize_option_blob<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let s: Option<serde_bytes::ByteBuf> = Option::deserialize(deserializer)?;
    Ok(s.map(|b| b.to_vec()))
}

/// Efficient deserializer for `Vec<Vec<u8>>` using `serde_bytes::ByteBuf` internally
/// for the inner vector to speed up deserialization.
pub fn deserialize_vec_blob<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;

    let s: Vec<serde_bytes::ByteBuf> = Vec::deserialize(deserializer)?;
    Ok(s.into_iter().map(|b| b.to_vec()).collect())
}
