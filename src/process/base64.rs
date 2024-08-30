use crate::cli::Base64Format;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use std::{fs::File, io::Read};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_read(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_read(input)?;

    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // avoid accidental newlines
    let buf = buf.trim();

    let buf = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    // TODO: decode data might not be string (but for this example, we assume it is)
    let decoded = String::from_utf8(buf)?;
    println!("{}", decoded);
    Ok(())
}

fn get_read(input: &str) -> Result<Box<dyn Read>> {
    if input == "-" {
        Ok(Box::new(std::io::stdin()))
    } else {
        Ok(Box::new(File::open(input)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/tmp.txt";
        let format = Base64Format::UrlSafe;
        assert!(process_decode(input, format).is_ok());
    }
}
