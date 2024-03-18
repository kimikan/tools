use byteorder::{ByteOrder, LittleEndian};

#[repr(u8)]
pub enum BoardId {
    Undefined = 1,
    Soc = 2,
    Mcu = 3,
}

impl TryFrom<u8> for BoardId {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BoardId::Undefined),
            2 => Ok(BoardId::Soc),
            3 => Ok(BoardId::Mcu),
            _ => Err(anyhow::Error::msg("invalid board id")),
        }
    }
}

#[repr(u8)]
pub enum MessageType {
    ReqResp = 1,
    MeMessage = 2,
    Event = 3,
    Ack = 4,
    Result = 5,
}

impl TryFrom<u8> for MessageType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(MessageType::ReqResp),
            2 => Ok(MessageType::MeMessage),
            3 => Ok(MessageType::Event),
            4 => Ok(MessageType::Ack),
            5 => Ok(MessageType::Result),
            _ => Err(anyhow::Error::msg("invalid message type")),
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum MajorCommand {
    Chassis = 0x31,
    Radar = 0x32,
}

impl TryFrom<u8> for MajorCommand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x31 => Ok(MajorCommand::Chassis),
            0x32 => Ok(MajorCommand::Radar),
            _ => Err(anyhow::Error::msg("invalid major command")),
        }
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum SubCommand {
    Chassis = 1,
    Control = 2,
    State = 3,
    CanDebug = 4,
    ChassisDetail = 5,
}

impl TryFrom<u8> for SubCommand {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(SubCommand::Chassis),
            2 => Ok(SubCommand::Control),
            3 => Ok(SubCommand::State),
            4 => Ok(SubCommand::CanDebug),
            5 => Ok(SubCommand::ChassisDetail),
            _ => Err(anyhow::Error::msg("invalid sub command")),
        }
    }
}

#[repr(packed)]
pub struct McuMessage<'a> {
    pub board_id_: BoardId,
    pub msg_type_: MessageType,
    pub seq_: u8,
    pub version_: u8,
    pub length_: u16,

    pub sub_cmd_: SubCommand,
    pub major_cmd_: MajorCommand,

    pub payload: &'a [u8],
}

impl<'a> TryFrom<&'a [u8]> for McuMessage<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(anyhow::Error::msg("invalid mcu buffer"));
        }

        let board = value[0].try_into()?;
        let msg_type = value[1].try_into()?;
        let seq = value[2];
        let version = value[3];
        let length = LittleEndian::read_u16(&value[4..]);
        let sub_cmd = value[6].try_into()?;
        let major_cmd = value[7].try_into()?;

        Ok(Self {
            board_id_: board,
            msg_type_: msg_type,
            seq_: seq,
            version_: version,
            length_: length,
            sub_cmd_: sub_cmd,
            major_cmd_: major_cmd,
            payload: &value[8..length as usize + 4],
        })
    }
}

/*
项目	字节数	数据格式	备注
项目	1	uint8	2：毫米波雷达
数据包数量	1	uint8
雷达编号	1	uint8	1：毫米波雷达-左
            2：毫米波雷达-前
            3：毫米波雷达-右
            4：毫米波雷达-后
            5：毫米波雷达-左前
            6：毫米波雷达-右前
            7：毫米波雷达-左后
            8：毫米波雷达-右后
            9：超声波雷达
CAN数据ID	4字节	uint32	CAN ID
CAN数据大小	2字节	uint16	CAN数据实际长度(0~64)
CAN数据	8字节/64字节	uint8数组	CAN 数据,
            数据大小<=8时，8字节
            数据大小>8时，64字节
时间戳（保留）	8字节	uint64	保留
*/
