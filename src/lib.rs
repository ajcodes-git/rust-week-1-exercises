// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }

    //extract the first 8 characters (4 bytes) for the version
    let version_hex = &raw_tx_hex[0..8];

    // Convert the hex string to bytes
    let version_bytes = hex::decode(version_hex).map_err(|_| "Hex decode error".to_string())?;

    // Ensure we have exactly 4 bytes for the version
    if version_bytes.len() != 4 {
        return Err("Invalid version length".to_string());
    }

    // Convert the bytes to a u32
    // The bytes are in little-endian order, folloiwing what I learned from the Btcdemy first week video
    let version_code = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version_code)
}
