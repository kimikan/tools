use crate::matrix::Message;
use crate::utils;
use byteorder::{ByteOrder, LittleEndian};
use std::u64;

#[derive(Debug)]
pub enum RadarType {
    Left = 1,
    Front = 2,
    Right = 3,
    Rear = 4,
    LeftFront = 5,
    RightFront = 6,
    LeftRear = 7,
    RightRear = 8,
    USS = 9,
}

impl TryFrom<u8> for RadarType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(RadarType::Left),
            2 => Ok(RadarType::Front),
            3 => Ok(RadarType::Right),
            4 => Ok(RadarType::Rear),
            5 => Ok(RadarType::LeftFront),
            6 => Ok(RadarType::RightFront),
            7 => Ok(RadarType::LeftRear),
            8 => Ok(RadarType::RightRear),
            9 => Ok(RadarType::USS),
            _ => Err(anyhow::Error::msg("invalid radar type")),
        }
    } // end fn
}

#[derive(Debug)]
pub struct Packet<'a> {
    pub type_: RadarType,
    pub can_id_: u32,
    //pub can_len_: u16,
    pub can_buf_: Vec<u8>,
    pub signals: Vec<f64>,

    pub meta_data: Option<&'a Message>,
}

impl<'a> Packet<'a> {
    pub fn try_from(buf: &[u8]) -> anyhow::Result<(Self, usize)> {
        if buf.len() < 7 {
            return Err(anyhow::Error::msg("invalid packet header"));
        }
        //println!("{:?}", buf);
        let t = RadarType::try_from(buf[0])?;
        let msg_id = LittleEndian::read_u32(&buf[1..]);
        let can_len = LittleEndian::read_u16(&buf[5..]);

        let size = can_len as usize + 7;
        if buf.len() < size {
            return Err(anyhow::Error::msg("invalid packet buf"));
        }

        let can_buf = (&buf[7..size]).to_vec();
        //println!("got can frame: {:?}", can_buf);
        Ok((
            Self {
                type_: t,
                can_id_: msg_id,
                can_buf_: can_buf,
                signals: vec![],
                meta_data: None,
            },
            can_len as usize,
        ))
    } // end fn

    pub fn update_signals(&mut self, meta_data_s: &'a Vec<Message>) -> anyhow::Result<()> {
        let can_buf = &self.can_buf_[..];

        let msg_meta_data = meta_data_s
            .iter()
            .find(|&v| v.msg_id == self.can_id_ as u64);
        if let Some(m) = msg_meta_data {
            self.meta_data = Some(m);

            for signal in m.signals.iter() {
                let v = utils::motorola_msb_get(
                    can_buf,
                    signal.byte_index as usize,
                    signal.start_bit as usize,
                    signal.bit_length as usize,
                    signal.is_signed,
                );
                if signal.is_signed {
                    println!("{:?}-----------------------------------", signal);
                }
                //let v = bt.read_bits::<u64>(signal.bit_length  as usize);
                let last = (v as f64) * signal.resolution + signal.offset;
                if last >= signal.min_value && last <= signal.max_value {
                    self.signals.push(last);
                } else {
                    let e = format!("invalid value should in [min, max]: msg_id: 0x{:X}, signal_meta_data: {:?}, value:{}, can_buf: {:?}", m.msg_id, signal, last, can_buf);
                    println!("+++++++++++++++++++ {}", e);
                    //return Err(anyhow::Error::msg(e));
                } // end if
            }

            return Ok(());
        } // end if

        Err(anyhow::Error::msg(
            "msg NOT support, no related metadata found!",
        ))
    }
}

#[derive(Debug)]
pub struct Payload<'a> {
    pub project_: u8,
    pub frames_: Vec<Packet<'a>>,
    pub time_stamp_: u64,
}

impl TryFrom<&[u8]> for Payload<'_> {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 2 {
            return Err(anyhow::Error::msg("empty payload"));
        }

        let mut v = vec![];
        let count = value[1];

        let mut offset = 2;
        for _ in 0..count as usize {
            //println!("packet: {:?}", &value[offset..]);
            let (packet, size) = Packet::try_from(&value[offset..])?;
            if size <= 8 {
                offset += 23;
            } else if size <= 64 {
                offset += 79;
            } // end

            v.push(packet);
        } // end for

        Ok(Self {
            project_: value[0],
            frames_: v,
            time_stamp_: 0u64,
        })
    } // end fn
}

#[cfg(test)]
mod tests {
    use crate::parser::Payload;

    #[test]
    fn parser_test() -> anyhow::Result<()> {
        let mut v = Vec::new();
        v.resize(10000, 1);
        for i in 0..100 {
            v[i] = (i + 1) as u8;
        }
        let payload = Payload::try_from(&v[..])?;
        println!("{:?}", payload);
        Ok(())
    }
}
