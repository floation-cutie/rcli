use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{TextSignFormat, get_reader, process_genpass};

pub trait TextSign {
    /// Dynamic dispatch on reader to support stdin or file input
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Static dispatch on reader to support stdin or file input
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub fn process_text_sign(input: &Path, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &Path,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
    };
    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let ret = blake3::keyed_hash(&self.key, &data);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        // need to have a non-temporary variable to hold the computed hash
        let computed_hash = blake3::keyed_hash(&self.key, &data);
        Ok(computed_hash.as_bytes() == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let key = fs::read(path.as_ref())?;
        let signer = Self::try_new(&key)?;
        Ok(signer)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let key = fs::read(path.as_ref())?;
        let signer = Self::try_new(&key)?;
        Ok(signer)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let key = fs::read(path.as_ref())?;
        let verifier = Self::try_new(&key)?;
        Ok(verifier)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let sig = self.key.sign(&data);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&data, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.into_bytes();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().as_bytes().to_vec();
        let sk = sk.as_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Blake3 { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into().expect("Key must be 32 bytes for Blake3");
        let signer = Self::new(key);
        Ok(signer)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Ed25519Signer { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let signing_key = SigningKey::from_bytes(key.try_into()?);
        Ok(Ed25519Signer::new(signing_key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Ed25519Verifier { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let verifying_key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Ed25519Verifier::new(verifying_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blake3_sign_verify() -> Result<()> {
        let signer = Blake3::load("fixtures/blake3.txt")?;
        let data = b"Hello, world!";
        let sig = signer.sign(&mut data.as_ref())?;
        let ret = signer.verify(&mut data.as_ref(), &sig)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let signer = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let verifier = Ed25519Verifier::load("fixtures/ed25519.pk")?;
        let data = b"Hello, world!";
        let sig = signer.sign(&mut data.as_ref())?;
        let ret = verifier.verify(&mut data.as_ref(), &sig)?;
        assert!(ret);
        Ok(())
    }
}
