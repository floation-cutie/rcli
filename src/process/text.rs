use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{TextSignFormat, get_reader};

pub trait TextSign {
    /// Dynamic dispatch on reader to support stdin or file input
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Static dispatch on reader to support stdin or file input
    fn verify<R: Read>(&self, reader: &mut R, sig: &[u8]) -> Result<bool>;
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

pub fn process_text_sign(input: &Path, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("{}", signed);
    Ok(())
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

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let key = fs::read(path.as_ref())?;
        Self::try_new(&key)
    }
}
