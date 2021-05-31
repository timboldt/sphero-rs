use bitflags::bitflags;
use byteorder::{BigEndian, WriteBytesExt};

const ESC: u8 = 0xAB;
const SOP: u8 = 0x8D;
const EOP: u8 = 0xD8;
const ESC_ESC: u8 = 0x23;
const ESC_SOP: u8 = 0x05;
const ESC_EOP: u8 = 0x50;

bitflags! {
    struct Flags: u8 {
        // Packet is a Response
        // True: Packet is a response. This implies that the packet has the error code byte in the header.
        // False: Packet is a command.
        const IS_RESPONSE = 0b00000001;
        // Request Response
        // True: Request response to a command (only valid if the packet is a command).
        // False: Do not request any response.
        const REQUEST_RESPONSE = 0b00000010;
        // Request Only Error Response
        // True: Request response only if command results in an error (only valid if packet is a command and "Request Response" flag is set).
        // False: Do not request only error responses.
        const ONLY_ERROR_RESPONSE = 0b00000100;
        // Packet is Activity
        // True: This packet counts as activity. Reset receiver's inactivity timeout.
        // False: Do not reset receiver's inactivity timeout.
        const IS_ACTIVITY      = 0b00001000;
        // Packet has Target ID
        // True: Packet has Target ID byte in header.
        // False: Packet has no specified target.
        const HAS_TARGET = 0b00010000;
        // Packet has Source ID
        // True: Packet has Source ID byte in header.
        // False: Packet has no specified source.
        const HAS_SOURCE = 0b00100000;
        // Currently Unused
        const UNUSED = 0b01000000;
        // Extended Flags
        // True: The next header byte is extended flags.
        // False: This is the last flags byte.
        const EXTENDED_FLAGS = 0b10000000;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Command {
    SomeCommand1 = 0x01,
}

#[derive(Debug, Copy, Clone)]
pub enum Device {
    SomeDevice1 = 0x01,
}

struct Node {
    port_id: u8,
    node_id: u8,
}

pub struct Packet {
    // is_response: bool,
    // request_response: bool,
    // request_only_error_response: bool,
    // packet_is_activity: bool,

    target: Node,
    // source: Option<Node>,
    device: Device,
    command: Command,
    seq_no: u8,
    // err_code: Option<u8>,
    payload: Vec<u8>,
}

impl Packet {
    pub fn new(device: Device, command: Command, seq_no: u8, payload: Vec<u8>) -> Packet {
        Packet {
            target: Node {
                // TODO: use a proper node id.
                port_id: 0,
                node_id: 0,
            },
            device: device,
            command: command,
            seq_no: seq_no,
            payload: payload,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        // SOP	Start of Packet	Control byte identifying the start of the packet
        // FLAGS	Packet Flags	Bit-flags that modify the behavior of the packet‰
        // TID	Target ID	Address of the target, expressed as a port ID (upper nibble) and a node ID (lower nibble). (Optional)
        // SID	Source ID	Address of the source, expressed as a port ID (upper nibble) and a node ID (lower nibble). (Optional)
        // DID	Device ID	The command group ("virtual device") of the command being sent
        // CID	Command ID	The command to execute
        // SEQ	Sequence Number	The token used to link commands with responses
        // ERR	Error Code	Command error code of the response packet (optional)
        // DATA...	Message Data	Zero or more bytes of message data
        // CHK	Checksum	The sum of all bytes (excluding SOP & EOP) mod 256, bit-inverted
        // EOP	End of Packet	Control byte identifying the end of the packet
        let mut buf = vec![];
        buf.push(SOP);
        buf.push((Flags::REQUEST_RESPONSE | Flags::HAS_TARGET).bits);
        buf.push((self.target.port_id << 4) | self.target.node_id);
        buf.push(self.device as u8);
        buf.push(self.command as u8);
        buf.push(self.seq_no);
        self.add_escaped(&mut buf, &self.payload[..]);
        buf.push(self.checksum(&buf[1..]));
        buf.push(EOP);
        buf
    }

    pub fn add_escaped(&self, buf: &mut Vec<u8>, bytes: &[u8]) {
        for b in bytes {
            match *b {
                ESC => {
                    buf.push(ESC);
                    buf.push(ESC_ESC);
                },
                SOP => {
                    buf.push(ESC);
                    buf.push(ESC_SOP);
                },
                EOP => {
                    buf.push(ESC);
                    buf.push(ESC_EOP);
                },
                _ => {
                    buf.push(*b);
                },
            }
        }
    }

    pub fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut sum : u8 = 0;
        for b in bytes {
            sum += b;
        }
        sum ^ 0xff
    }

    // TODO: add tests
}
