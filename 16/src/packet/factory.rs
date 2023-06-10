use super::*;

pub fn factory(raw: &[bool]) -> Box<dyn Packet> {
    let header = PacketHeader::init(raw);

    match header.id {
        4 => Box::new(PacketLiteral::init(
            header,
            &raw[PacketHeader::VTABLE_END..],
        )),
        _ => Box::new(PacketOperator::init(
            header,
            &raw[PacketHeader::VTABLE_END..],
        )),
    }
}
