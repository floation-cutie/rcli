use std::io::Read;
use std::path::Path;

use base64::engine::general_purpose::*;
use base64::prelude::*;

use crate::{Base64Format, get_reader};

/// input: filename or '-'(stdin) as a Reader
pub fn process_encode(input: &Path, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(data),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(data),
    };
    Ok(encoded)
}

pub fn process_decode(input: &Path, format: Base64Format) -> anyhow::Result<String> {
    let mut reader = get_reader(input)?;
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    // remove whitespace/newline characters
    data.retain(|c| !c.is_whitespace());
    let decoded_bytes = match format {
        Base64Format::Standard => STANDARD.decode(data)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(data)?,
    };
    let decoded = String::from_utf8(decoded_bytes)?;
    Ok(decoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_std_encode_decode() {
        use std::io::Write;

        use tempfile::NamedTempFile;

        let test_str = "Hello, rcli Base64!";
        let encoded_standard = "SGVsbG8sIHJjbGkgQmFzZTY0IQ==";
        let encoded_urlsafe = "SGVsbG8sIHJjbGkgQmFzZTY0IQ";

        // Create temporary file with test data
        let mut input_file = NamedTempFile::new().unwrap();
        input_file.write_all(test_str.as_bytes()).unwrap();
        let input_path = input_file.path();

        // Test Standard format encode
        let result_standard = process_encode(input_path, Base64Format::Standard).unwrap();
        assert_eq!(result_standard, encoded_standard);

        // Test UrlSafe format encode
        let result_urlsafe = process_encode(input_path, Base64Format::UrlSafe).unwrap();
        assert_eq!(result_urlsafe, encoded_urlsafe);

        // Test Standard format decode
        let mut encoded_file_std = NamedTempFile::new().unwrap();
        encoded_file_std.write_all(encoded_standard.as_bytes()).unwrap();
        let decoded_standard =
            process_decode(encoded_file_std.path(), Base64Format::Standard).unwrap();
        assert_eq!(decoded_standard, test_str);

        // Test UrlSafe format decode
        let mut encoded_file_url = NamedTempFile::new().unwrap();
        encoded_file_url.write_all(encoded_urlsafe.as_bytes()).unwrap();
        let decoded_urlsafe =
            process_decode(encoded_file_url.path(), Base64Format::UrlSafe).unwrap();
        assert_eq!(decoded_urlsafe, test_str);

        // Test round-trip: encode then decode should return original
        let encoded = process_encode(input_path, Base64Format::Standard).unwrap();
        let mut roundtrip_file = NamedTempFile::new().unwrap();
        roundtrip_file.write_all(encoded.as_bytes()).unwrap();
        let decoded = process_decode(roundtrip_file.path(), Base64Format::Standard).unwrap();
        assert_eq!(decoded, test_str);
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(Path::new(input), format).is_ok());
    }
}
