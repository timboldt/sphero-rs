use bitflags::bitflags;
//use byteorder::{BigEndian, WriteBytesExt};

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Command {
    InvalidCommand = 0x00,
    SomeCommand1 = 0x01,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Device {
    InvalidDevice = 0x00,
    SomeDevice1 = 0x01,
}

#[derive(Debug)]
struct Node {
    port_id: u8,
    node_id: u8,
}

#[derive(Debug)]
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

    pub fn deserialize(bytes: &[u8]) -> Result<Packet, &'static str> {
        #[derive(Debug, Copy, Clone, PartialEq)]
        enum State {
            AtStart,
            AtFlags,
            AtTarget,
            AtSource,
            AtDevice,
            AtCommand,
            AtSeqNo,
            AtErr,
            AtData,
            AtChecksum,
            AtEnd,
            Validated,
        }
        let mut n = 0;
        let mut state = State::AtStart;
        let mut flags = Flags::empty();
        while n < bytes.len() {
            match state {
                State::AtStart => {
                    if bytes[n] == SOP {
                        state = State::AtFlags;
                    } else {
                        return Err("SOP not found");
                    }
                }
                State::AtFlags => {
                    // TODO: don't panic here.
                    flags = Flags::from_bits(bytes[n]).unwrap();
        // // Packet is a Response
        // // True: Packet is a response. This implies that the packet has the error code byte in the header.
        // // False: Packet is a command.
        // const IS_RESPONSE = 0b00000001;
        // // Request Response
        // // True: Request response to a command (only valid if the packet is a command).
        // // False: Do not request any response.
        // const REQUEST_RESPONSE = 0b00000010;
        // // Request Only Error Response
        // // True: Request response only if command results in an error (only valid if packet is a command and "Request Response" flag is set).
        // // False: Do not request only error responses.
        // const ONLY_ERROR_RESPONSE = 0b00000100;

        // // Extended Flags
        // // True: The next header byte is extended flags.
        // // False: This is the last flags byte.
        // const EXTENDED_FLAGS = 0b10000000;
                    // TODO: save flags?
                    // TODO: determine next step based on flags
                    if flags.contains(Flags::HAS_TARGET) {
                        state = State::AtTarget;
                    } else if flags.contains(Flags::HAS_SOURCE) {
                        state = State::AtSource;
                    } else {
                        state = State::AtDevice;
                    }
                }
                State::AtTarget  => {
                    if flags.contains(Flags::HAS_SOURCE) {
                        state = State::AtSource;
                    } else {
                        state = State::AtDevice;
                    }
                },
                State::AtSource  => {
                    state = State::AtDevice;
                },
                State::AtDevice  => {
                    state = State::AtCommand;
                },
                State::AtCommand  => {
                    state = State::AtSeqNo;
                },
                State::AtSeqNo  => {
                    if flags.contains(Flags::IS_RESPONSE) {
                        state = State::AtErr;
                    } else {
                        state = State::AtData;
                    }
                },
                State::AtErr  => {
                    state = State::AtData;
                },
                State::AtData  => {
                    state = State::AtChecksum;
                },
                State::AtChecksum  => {
                    state = State::AtEnd;
                },
                State::AtEnd => {
                    if bytes[n] == EOP {
                        state = State::Validated;
                    } else {
                        return Err("EOP not found");
                    }
                }
                State::Validated => {
                    return Err("Unexpected bytes after end of packet");
                }
            };
            n += 1;
        }
        if state != State::Validated {
            return Err("Unexpected end of packet");
        }
        Ok(Packet {
            target: Node {
                // TODO: use a proper node id.
                port_id: 0,
                node_id: 0,
            },
            device: Device::InvalidDevice,
            command: Command::InvalidCommand,
            seq_no: 0x00,
            payload: vec![],
        })
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

    fn add_escaped(&self, buf: &mut Vec<u8>, bytes: &[u8]) {
        for b in bytes {
            match *b {
                ESC => {
                    buf.push(ESC);
                    buf.push(ESC_ESC);
                }
                SOP => {
                    buf.push(ESC);
                    buf.push(ESC_SOP);
                }
                EOP => {
                    buf.push(ESC);
                    buf.push(ESC_EOP);
                }
                _ => {
                    buf.push(*b);
                }
            }
        }
    }

    fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut sum: u8 = 0;
        for b in bytes {
            sum += b;
        }
        sum ^ 0xff
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p = Packet::new(
            Device::SomeDevice1,
            Command::SomeCommand1,
            22,
            vec![1, 2, 3],
        );
        assert_eq!(p.target.node_id, 0);
        assert_eq!(p.target.port_id, 0);
        assert_eq!(p.device, Device::SomeDevice1);
        assert_eq!(p.command, Command::SomeCommand1);
        assert_eq!(p.seq_no, 22);
        assert_eq!(p.payload, vec![1, 2, 3]);
    }

    #[test]
    fn test_serialize() {
        let p = Packet::new(
            Device::SomeDevice1,
            Command::SomeCommand1,
            22,
            vec![1, 2, 3],
        );
        let s = p.serialize();
        assert_eq!(
            s,
            vec![
                SOP,
                0b00010010,
                0x00,
                Device::SomeDevice1 as u8,
                Command::SomeCommand1 as u8,
                22,
                1,
                2,
                3,
                p.checksum(&s[1..=8]),
                EOP
            ]
        );
    }

    #[test]
    fn test_deserialize() {
        let p = Packet::deserialize(&vec![
            SOP,
            0b00010010,
            0x00,
            Device::SomeDevice1 as u8,
            Command::SomeCommand1 as u8,
            22,
            1,
            123,
            EOP
        ]);
        println!("{:?}", p);
        assert!(p.is_ok());
    }

    // TODO: test escape sequences, etc.
}
