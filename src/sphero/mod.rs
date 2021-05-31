
pub enum Command {
    SomeCommand1,
}

pub enum Device {
    SomeDevice1,
}

struct Node {
    port_id: u8,
    node_id: u8,
}



//     0	Packet is a Response	Packet is a response. This implies that the packet has the error code byte in the header.	Packet is a command.
// 1	Request Response	Request response to a command (only valid if the packet is a command).	Do not request any response.
// 2	Request Only Error Response	Request response only if command results in an error (only valid if packet is a command and "Request Response" flag is set).	Do not request only error responses.
// 3	Packet is Activity	This packet counts as activity. Reset receiver's inactivity timeout.	Do not reset receiver's inactivity timeout.
// 4	Packet has Target ID	Packet has Target ID byte in header.	Packet has no specified target.
// 5	Packet has Source ID	Packet has Source ID byte in header.	Packet has no specified source.
// 6	Currently Unused	n/a	n/a
// 7	Extended Flags	The next header byte is extended flags.	This is the last flags byte.

pub struct Packet {
    // is_response: bool,
    // request_response: bool,
    // request_only_error_response: bool,
    // packet_is_activity: bool,
    
    // target: Option<Node>,
    // source: Option<Node>,
    device: Device,
    command: Command,
    // seq_no: u8,
    // err_code: Option<u8>,
    payload: Vec<u8>,
}

impl Packet {
    pub fn new(device: Device, command: Command, payload: Vec<u8>) -> Packet {
        Packet{
            device: device,
            command: command,
            payload: payload,
        }
    }
}

// SOP	Start of Packet	Control byte identifying the start of the packet
// FLAGS	Packet Flags	Bit-flags that modify the behavior of the packet
// TID	Target ID	Address of the target, expressed as a port ID (upper nibble) and a node ID (lower nibble). (Optional)
// SID	Source ID	Address of the source, expressed as a port ID (upper nibble) and a node ID (lower nibble). (Optional)
// DID	Device ID	The command group ("virtual device") of the command being sent
// CID	Command ID	The command to execute
// SEQ	Sequence Number	The token used to link commands with responses
// ERR	Error Code	Command error code of the response packet (optional)
// DATA...	Message Data	Zero or more bytes of message data
// CHK	Checksum	The sum of all bytes (excluding SOP & EOP) mod 256, bit-inverted
// EOP	End of Packet	Control byte identifying the end of the packet
