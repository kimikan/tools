use std::str::FromStr;

pub fn motorola_msb_get(
    buf: &[u8],
    byte_index: usize,
    start_bit: usize,
    num_bits: usize,
    _is_signed: bool,
) -> u64 {
    motorola_msb_get_signed(buf, byte_index, start_bit, num_bits)
}

fn motorola_msb_get_signed(
    buf: &[u8],
    byte_index: usize,
    start_bit: usize,
    num_bits: usize,
) -> u64 {
    let mut result: u64 = 0;
    let start_offset = start_bit % 8;
    let len = num_bits + 7 - start_offset;

    let bytes = (len + 7) / 8;
    for &b in &buf[byte_index..byte_index + bytes] {
        result = (result << 8) | u64::from(b);
    }
    result <<= 64 - bytes * 8 + (7 - start_offset);
    result >>= 64 - num_bits;
    result
}

pub fn option_string_to_string(s: Option<String>) -> anyhow::Result<String> {
    let v = s.ok_or(anyhow::Error::msg("non string"))?;
    Ok(v)
}

pub fn option_string_to_t<T: FromStr>(s: Option<String>) -> anyhow::Result<T> {
    let v = option_string_to_string(s)?;
    let v = v.trim();
    let u = v.parse::<T>();
    if let Ok(x) = u {
        return Ok(x);
    }

    Err(anyhow::Error::msg("invalid type convert"))
}

// oxeee to u64
pub fn hex_str_to_u64(msg_id: &String) -> anyhow::Result<u64> {
    let id = msg_id.trim_start_matches("0x");
    let i_id = u64::from_str_radix(id.trim(), 16);
    if let Ok(i) = i_id {
        return Ok(i);
    }

    Err(anyhow::Error::msg("invalid hex to u64"))
}
