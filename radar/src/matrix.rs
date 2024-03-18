use crate::utils;
use calamine::{Data, DataType, Reader, Xlsx};
use prettytable::{Cell, Table};

fn signal_name(cols: &[Data]) -> Option<String> {
    cols[6].as_string()
}

fn byte_index(cols: &[Data]) -> Option<String> {
    cols[9].as_string()
}

fn start_bit(cols: &[Data]) -> Option<String> {
    cols[10].as_string()
}

fn bit_length(cols: &[Data]) -> Option<String> {
    cols[11].as_string()
}

fn data_type(cols: &[Data]) -> Option<String> {
    cols[13].as_string()
}

fn resolution(cols: &[Data]) -> Option<String> {
    cols[14].as_string()
}

fn offset(cols: &[Data]) -> Option<String> {
    cols[15].as_string()
}

fn min_value(cols: &[Data]) -> Option<String> {
    cols[16].as_string()
}

fn max_value(cols: &[Data]) -> Option<String> {
    cols[17].as_string()
}

#[derive(Debug)]
pub struct Signal {
    pub signal_name: String,
    pub byte_index: u64,
    pub start_bit: u64,
    pub bit_length: u64,

    pub resolution: f64,
    pub offset: f64,

    pub min_value: f64,
    pub max_value: f64,
    pub is_signed: bool,
}

impl TryFrom<&[Data]> for Signal {
    type Error = anyhow::Error;

    fn try_from(value: &[Data]) -> Result<Self, Self::Error> {
        let signal_name = utils::option_string_to_string(signal_name(value))?;
        let start_bit = utils::option_string_to_t::<u64>(start_bit(value))?;
        let byte_index = utils::option_string_to_t::<u64>(byte_index(value))?;

        let bit_length = utils::option_string_to_t::<u64>(bit_length(value))?;
        let resolution = utils::option_string_to_t::<f64>(resolution(value))?;
        let data_type = utils::option_string_to_string(data_type(value))?;
        let offset = utils::option_string_to_t::<f64>(offset(value))?;
        let min_value = utils::option_string_to_t::<f64>(min_value(value))?;
        let max_value = utils::option_string_to_t::<f64>(max_value(value))?;

        Ok(Self {
            signal_name,
            start_bit,
            byte_index,
            bit_length,
            is_signed: data_type != "Unsigned",
            resolution,
            offset,
            min_value,
            max_value,
        })
    }
}

fn is_msg_row(cols: &[Data]) -> bool {
    if cols.len() < 2 {
        return false;
    }

    let c0 = &cols[0];
    let c2 = &cols[2];
    !c0.is_empty() && !c2.is_empty()
}

fn is_signal_row(cols: &[Data]) -> bool {
    !is_msg_row(cols)
}

fn msg_name(cols: &[Data]) -> Option<String> {
    cols[0].as_string()
}

fn msg_id(cols: &[Data]) -> Option<String> {
    cols[2].as_string()
}

fn msg_length(cols: &[Data]) -> Option<String> {
    cols[5].as_string()
}

#[derive(Debug)]
pub struct Message {
    pub msg_name: String,
    pub msg_id_str: String,
    pub msg_id: u64,
    pub msg_len: u64,

    pub signals: Vec<Signal>,
}

impl Message {
    #[allow(dead_code)]
    fn print_table(&self) {
        let mut table = Table::new();

        table.set_titles(prettytable::Row::new(vec![
            Cell::new("MsgName"),
            Cell::new("MsgId"),
            Cell::new("Length"),
        ]));
        // 通过Cell添加行
        table.add_row(prettytable::Row::new(vec![
            Cell::new(&self.msg_name),
            Cell::new(&self.msg_id_str),
            Cell::new(&self.msg_len.to_string()),
        ]));

        let mut table2 = Table::new();
        table2.set_titles(prettytable::Row::new(vec![
            Cell::new("SignalName"),
            Cell::new("SignalStartBit"),
            Cell::new("Resolution"),
            Cell::new("Offset"),
        ]));
        for s in &self.signals {
            table2.add_row(prettytable::Row::new(vec![
                Cell::new(&s.signal_name),
                Cell::new(&s.start_bit.to_string()),
                Cell::new(&s.resolution.to_string()),
                Cell::new(&s.offset.to_string()),
            ]));
        }
        // 打印表格到标准输出
        table.printstd();
        table2.printstd();
        println!("\n");
    }
}

impl TryFrom<(&[Data], Vec<&[Data]>)> for Message {
    type Error = anyhow::Error;

    fn try_from(value: (&[Data], Vec<&[Data]>)) -> Result<Self, Self::Error> {
        if value.0.len() < 18 || value.1.iter().any(|&v| v.len() < 18) {
            return Err(anyhow::Error::msg("invalid data"));
        }

        let msg_name = utils::option_string_to_string(msg_name(value.0))?;
        let msg_id_str = utils::option_string_to_string(msg_id(value.0))?;
        let msg_id = utils::hex_str_to_u64(&msg_id_str)?;
        let msg_len = utils::option_string_to_t::<u64>(msg_length(value.0))?;
        let signals = value
            .1
            .iter()
            .map(|&v| Signal::try_from(v))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self {
            msg_name,
            msg_id_str,
            msg_id,
            msg_len,
            signals,
        })
    } // end try_from
}

fn get_metadata_groups<'a>(rows: &Vec<&'a [Data]>) -> Vec<(&'a [Data], Vec<&'a [Data]>)> {
    let mut v = vec![];
    let mut msg_index = -1;
    let mut signals = vec![];
    for i in 0..rows.len() {
        if msg_index < 0 {
            if is_msg_row(rows[i]) {
                msg_index = i as i32;
            } // end if
        } else {
            if is_signal_row(rows[i]) {
                signals.push(rows[i].clone());
            } else {
                v.push((rows[msg_index as usize], signals.clone()));
                msg_index = i as i32;
                signals.clear();
            }
        }
    }
    v
}

#[derive(Debug)]
pub struct Matrix {
    pub msg_s: Vec<Message>,
}

impl Matrix {
    pub fn load(file: &str) -> anyhow::Result<Self> {
        let mut excel: Xlsx<_> = calamine::open_workbook(file)?;

        // Get worksheet
        let sheet = excel.worksheet_range("Matrix")?;

        // iterate over rows
        let rows = sheet
            .rows()
            .filter(|&v| {
                let r_s = "s".to_owned();
                let s_s = "r".to_owned();

                v[24].as_string() == Some(r_s) || v[24].as_string() == Some(s_s)
            })
            .collect::<Vec<_>>();

        let groups = get_metadata_groups(&rows);
        let msg_s = groups
            .into_iter()
            .map(|v| Message::try_from(v))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(Self { msg_s })
    }
}
