use super::Packet;
use super::PacketHeader;

pub struct Literal {
    header: PacketHeader,
    length: usize,
    value: u64,
}

impl Literal {
    const GROUP_LENGTH: usize = 5;

    #[allow(dead_code)]
    pub fn parse(raw: &[bool]) -> Self {
        Self::init(PacketHeader::init(raw), &raw[PacketHeader::VTABLE_END..])
    }

    pub fn init(header: PacketHeader, contents: &[bool]) -> Self {
        debug_assert!(header.id == 4);

        let mut count = 0;
        let mut data = Vec::new();
        // Extract one group of Self::GROUP_LENGTH bytes in sequence
        // First bit encodes if this is the first or last group
        // Remaining four bits code what value is represented by the packet
        // literal
        // Parsing stops when the first bit encodes the "last" group
        loop {
            let bitselect = (count * Self::GROUP_LENGTH + 1)
                ..((count + 1) * Self::GROUP_LENGTH);

            data.extend_from_slice(&contents[bitselect]);

            if contents[count * Self::GROUP_LENGTH] {
                count += 1;
            } else {
                break;
            }
        }

        // Note: Packet length is the some of the length of the packet header
        // plus all the bits we decoded in order to extract 'value
        Self {
            header,
            value: super::parse_bitstream(&data),
            length: PacketHeader::VTABLE_END + (count + 1) * Self::GROUP_LENGTH,
        }
    }
}

impl Packet for Literal {
    fn version(&self) -> u8 {
        self.header.version
    }

    fn id(&self) -> u8 {
        self.header.id
    }

    fn children(&self) -> &[Box<dyn Packet>] {
        &[]
    }

    fn len(&self) -> usize {
        self.length
    }

    fn compute(&self) -> u64 {
        self.value
    }
}
