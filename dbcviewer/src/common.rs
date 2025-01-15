use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};
use std::fs::File;
use std::io::Read;

pub fn read_to_utf8(file: &str) -> anyhow::Result<Vec<u8>> {
    let mut f = File::open(file)?;

    let mut contents = Vec::new();
    f.read_to_end(&mut contents)?;
    if let Ok(_) = String::from_utf8(contents.clone()) {
        return Ok(contents);
    } else {
        let gbk = GBK
            .decode(&contents, DecoderTrap::Strict)
            .map_err(|e| anyhow::anyhow!("gbk decode: {}", e))?;
        let v = gbk.as_bytes().to_vec();
        println!("gbk");
        Ok(v)
    }
}
