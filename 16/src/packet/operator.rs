use super::Packet;
use super::PacketFactory;
use super::PacketHeader;

struct OperatorLength {
    id: bool,
    bitsize: u8, // 11 or 15 bits
    value: u16,
}
pub struct Operator<'a> {
    header: PacketHeader,
    length: OperatorLength,
    children: Vec<Box<dyn Packet + 'a>>,
}

impl Operator<'_> {
    #[allow(dead_code)]
    pub fn parse(raw: &[bool]) -> Self {
        Self::init(PacketHeader::init(raw), &raw[PacketHeader::VTABLE_END..])
    }

    pub fn init(header: PacketHeader, contents: &[bool]) -> Self {
        debug_assert!(header.id != 4);

        let bitsize: u8 = if contents[0] { 11 } else { 15 };
        let length = OperatorLength {
            id: contents[0],
            bitsize,
            value: super::parse_bitstream(&contents[1..=bitsize as usize]),
        };

        Self {
            children: Self::parse_as_children(
                &length,
                &contents[(bitsize as usize + 1)..],
            ),
            header,
            length,
        }
    }

    fn parse_as_children<'a>(
        length: &OperatorLength,
        contents: &[bool],
    ) -> Vec<Box<dyn Packet + 'a>> {
        let mut result: Vec<Box<dyn Packet + 'a>> = Vec::new();
        let mut cursor = 0usize;

        loop {
            // Stop if number of bits or number of packets was achieved, depending
            // on length_type
            if (!length.id && (cursor >= length.value as usize))
                || (length.id && (result.len() >= length.value as usize))
            {
                break;
            }

            // Create a new packet. This depends on the header. Logic was passed
            // down to the PacketFactory module
            result.push(PacketFactory::factory(&contents[cursor..]));

            // Update cursor with the last added packet size
            cursor += result.last().unwrap().len();
        }

        result
    }
}

impl Packet for Operator<'_> {
    fn version(&self) -> u8 {
        self.header.version
    }

    fn id(&self) -> u8 {
        self.header.id
    }

    fn children<'a>(&'a self) -> &'a [Box<dyn Packet + 'a>] {
        &self.children
    }

    fn len(&self) -> usize {
        PacketHeader::VTABLE_END                               // from header
        + 1                                                    // From PacketOperatorLength::id
        + self.length.bitsize as usize                         // from PacketOperatorLength::value
        + self.children.iter().map(|c| c.len()).sum::<usize>() // from children
    }

    fn compute(&self) -> u64 {
        let values = &self.children;
        match self.id() {
            0 => values.iter().map(|c| c.compute()).sum(),
            1 => values.iter().map(|c| c.compute()).product(),
            2 => values.iter().map(|c| c.compute()).min().unwrap(),
            3 => values.iter().map(|c| c.compute()).max().unwrap(),
            5 => {
                debug_assert!(values.len() == 2);
                (values[0].compute() > values[1].compute()) as u64
            }
            6 => {
                debug_assert!(values.len() == 2);
                (values[0].compute() < values[1].compute()) as u64
            }
            7 => {
                debug_assert!(values.len() == 2);
                (values[0].compute() == values[1].compute()) as u64
            }
            _ => panic!("Operation not implemented!"),
        }
    }
}
