use crate::msg::{MajorCommand, McuMessage};
use crate::parser::Payload;
use byteorder::{BigEndian, ByteOrder};
use pcap_file::pcap::{PcapPacket, PcapReader};
use pdu::Ipv4;
use pdu::{Ethernet, EthernetPdu};
use std::fs::File;
use std::time::Duration;

mod matrix;
mod msg;
mod parser;
mod utils;

#[derive(Debug)]
struct UdpPacket {
    timestamp: Duration,
    pcap_index: Option<usize>,
    src_port: u16,
    dst_port: u16,
    src_address: [u8; 4],
    dst_address: [u8; 4],

    buf: Vec<u8>,
}

impl TryFrom<PcapPacket<'_>> for UdpPacket {
    type Error = anyhow::Error;

    fn try_from(pcap_pkt: PcapPacket) -> Result<Self, Self::Error> {
        let pdu = EthernetPdu::new(&pcap_pkt.data[..])?;
        let eth_pdu = pdu.inner()?;

        if let Ethernet::Ipv4(ipv4_pdu) = eth_pdu {
            //println!("[ipv4] source_address: {:x?}", ipv4_pdu.source_address().as_ref());
            //println!("[ipv4] destination_address: {:x?}", ipv4_pdu.destination_address().as_ref());
            //println!("[ipv4] protocol: 0x{:02x}", ipv4_pdu.protocol());
            if let Ipv4::Udp(udp_pdu) = ipv4_pdu.inner()? {
                let udp_buffer = udp_pdu.into_buffer();
                let contents = &udp_buffer[8..];
                //println!("[udp]: {:?}, size: {}", &contents[..4], contents.len());
                return Ok(UdpPacket {
                    timestamp: pcap_pkt.timestamp,
                    pcap_index: None,
                    src_address: ipv4_pdu.source_address(),
                    dst_address: ipv4_pdu.destination_address(),
                    src_port: BigEndian::read_u16(&udp_buffer[..]),
                    dst_port: BigEndian::read_u16(&udp_buffer[2..]),
                    buf: contents.to_vec(),
                });
            } // end if
        } // end if

        Err(anyhow::Error::msg("invalid udp packet"))
    } // end fn
}

fn get_all_udp_packets(file: &str) -> anyhow::Result<Vec<UdpPacket>> {
    let mut v = vec![];
    let file_in = File::open(file)?;
    let mut pcap_reader = PcapReader::new(file_in)?;

    let mut index = 1;
    while let Some(pkt) = pcap_reader.next_packet() {
        //Check if there is no error
        let pkt = pkt?;
        if let Ok(mut udp) = UdpPacket::try_from(pkt) {
            udp.pcap_index = Some(index);
            v.push(udp);
        }

        index += 1;
    } // end while

    Ok(v)
}

#[allow(dead_code)]
fn main() -> anyhow::Result<()> {
    const FILE: &str = "radar.pcap";
    let udp_pkts = get_all_udp_packets(FILE)?;

    let mat = matrix::Matrix::load("EEA2.0_CAN_Matrix_V6.0.0_20230928_FMRadar_RDCAN1.xlsx")?;
    let meta_data_s = mat.msg_s;

    for udp in udp_pkts {
        if udp.src_port != 30003 || udp.dst_port != 30500 {
            println!(
                "Udp port mismatch, src_addr={:?}, dst_addr: {:?}, src = {:?}, dst= {:?}",
                udp.src_address, udp.dst_address, udp.src_port, udp.dst_port
            );
            continue;
        } // end if

        let mcu_message = McuMessage::try_from(&udp.buf[..])?;
        if mcu_message.major_cmd_ != MajorCommand::Radar {
            println!(
                "Mcu message received: not radar data, {:?}, {:?}",
                mcu_message.major_cmd_, mcu_message.sub_cmd_
            );
            continue;
        }

        let mut payload = Payload::try_from(&mcu_message.payload[..])?;
        for frame in payload.frames_.iter_mut() {
            frame.update_signals(&meta_data_s)?;
        }
        println!("Got udp packet: {:?}", udp);
        println!("payload after parsed: {:?}\n\n", payload);
        static mut COUNT: usize = 0;
        unsafe {
            COUNT += 1;
        }
        unsafe {
            if COUNT > 44444444 {
                //break;
            }
        }
    }

    Ok(())
}

/*
#[allow(dead_code)]
fn main3() {
    let msgs = dbc::all_msgs().unwrap();
    for msg in msgs {
        let ok = MessageMetaData::try_from(&msg).unwrap();
        for sm in ok.signals_meta_data_s {
            if sm.signal_name.ends_with("RelativeVelocity_y")
                || sm.signal_name.ends_with("RelativeVelocity_x")
            {
                let ss = format!(
                    "static_cast<float_t>(motorola_msb_get(data, {}, {}, {})) * {}f + ({}f);",
                    sm.byte_index, sm.start_bit, sm.bit_length, sm.resolution, sm.offset
                );
                println!("{} , {}", sm.signal_name, ss);
            } // end if
        }
    } // end for
}
*/
