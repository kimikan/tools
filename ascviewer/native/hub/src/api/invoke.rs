
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::api::{matrix, utils};
use crate::messages::frames::FrameItem;

fn get_all_frames() -> anyhow::Result<Vec<String>> {
    let mut v = vec![];
    let f = File::open("./Q3312_RADAR_0523.asc")?;
    let mut br = BufReader::new(f);
    let mut start = false;
    loop {
        let mut line = String::new();
        let size = br.read_line(&mut line)?;

        if size <= 0 {
            break;
        }

        if start {
            if line.contains("Rx") || line.contains("Tx") {
                v.push(line);
            }
        } else {
            if line.contains("Start of measurement") {
                start = true;
            }
        }
    }
    Ok(v)
}

fn to_u8(s: &str) -> anyhow::Result<u8> {
    let v = u8::from_str_radix(&s, 16);
    if let Ok(v) = v {
        return Ok(v);
    }

    return Err(anyhow::Error::msg("fucking invalid u8"));
}

#[derive(Debug)]
struct Frame {
    is_rx: bool,
    msg_id: usize,
    msg_name: String,
    data: Vec<u8>,
}

impl TryFrom<&String> for Frame {
    type Error = anyhow::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let items = value.split_whitespace().collect::<Vec<_>>();

        if items.len() <= 10 {
            println!("{}", value);
            return Err(anyhow::Error::msg("fucking invalid format"));
        }
        let is_rx = items[3] == "Rx";
        let msg_id = usize::from_str_radix(items[4], 16)?;
        let msg_name = items[5].to_owned();

        let len: usize = items[9].parse()?;
        if items.len() < len + 10 {
            return Err(anyhow::Error::msg("invalid buf"));
        }

        let buf = &items[10..10 + len];
        let data = buf
            .into_iter()
            .map(|&v| to_u8(v))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            is_rx,
            msg_id,
            msg_name,
            data,
        })
    }
}

fn update_signals(
    frame: &Frame,
    meta_data_s: &Vec<matrix::Message>,
) -> anyhow::Result<(usize, String, String, HashMap<String, String>)> {
    let mut map = HashMap::new();
    //map.insert("Id".to_owned(), frame.msg_id.to_string());
    //map.insert("msg_name".to_string(), frame.msg_name.clone());
    let direction = match frame.is_rx {
        true => "Rx",
        false => "Tx",
    };

    let msg_meta_data = meta_data_s
        .iter()
        .find(|&v| v.msg_id == frame.msg_id as u64);

    let can_buf = &frame.data[..];
    if let Some(m) = msg_meta_data {
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
                map.insert(signal.signal_name.clone(), last.to_string());
            } else {
                let e = format!("invalid value should in [min, max]: msg_id: 0x{:X}, signal_meta_data: {:?}, value:{}, can_buf: {:?}", m.msg_id, signal, last, can_buf);
                println!("+++++++++++++++++++ {}", e);
                // return Err(anyhow::Error::msg(e));
            } // end if
        }
    } // end if
    return Ok((
        frame.msg_id,
        frame.msg_name.clone(),
        direction.to_string(),
        map,
    ));
}

pub fn asc_to_string() -> anyhow::Result<String> {
    let mut s = String::new();
    let ss = &get_all_frames()?;

    let frames = ss
        .iter()
        .map(|v| Frame::try_from(v))
        .collect::<Result<Vec<_>, _>>()?;

    let meta_data =
        matrix::Matrix::load("./EEA2.0_CAN_Matrix_V6.0.0_20230928_FMRadar_RDCAN1.xlsx")?;
    for frame in frames {
        let result = update_signals(&frame, &meta_data.msg_s)?;

        s += &(format!("{:?}\n", result));
    }
    Ok(s)
}

pub fn asc_to_frame_strings()->anyhow::Result<Vec<FrameItem>> {
    let ss = &get_all_frames()?;
    let mut v = vec![];
    let frames = ss
        .iter()
        .map(|v| Frame::try_from(v))
        .collect::<Result<Vec<_>, _>>()?;

    let meta_data =
        matrix::Matrix::load("./EEA2.0_CAN_Matrix_V6.0.0_20230928_FMRadar_RDCAN1.xlsx")?;
    for frame in frames {
        let result = update_signals(&frame, &meta_data.msg_s)?;
        let mut item = FrameItem{
            msg_id: result.0 as i32, 
            msg_name: result.1,
            direction: result.2,
            signals: vec![],
        };
        
        for x in result.3 {
            use crate::messages::frames::Signal;
            item.signals.push(Signal{sig_name: x.0, sig_value: x.1})
        }
        v.push(item);
    }
    Ok(v)
}
