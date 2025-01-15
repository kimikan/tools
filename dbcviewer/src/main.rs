mod common;
mod ui;

use can_dbc::{Error, DBC};

#[allow(dead_code)]
fn parse() -> anyhow::Result<()> {
    let buffer = common::read_to_utf8("./wm_ase3_megvii_pp.dbc")?;

    let dbc = DBC::from_slice(&buffer);

    if let Err(e) = dbc {
        if let Error::Incomplete(dbc, ..) = e {
            println!("incomplete dbc: {:?}", dbc.messages());
        }
    } else {
        let dbc = dbc.unwrap();

        for message in dbc.messages() {
            println!("------------ {:?}", message.message_name());
            for signal in message.signals() {
                println!("{:#?}", signal);
            }
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    //parse()?;
    ui::run()?;
    Ok(())
}
