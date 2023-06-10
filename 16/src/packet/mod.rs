pub mod factory;
mod header;
mod literal;
mod operator;
mod traits;

pub use factory as PacketFactory;
pub use header::Header as PacketHeader;
pub use literal::Literal as PacketLiteral;
pub use operator::Operator as PacketOperator;
pub use traits::Packet;

fn parse_bitstream<T>(data: &[bool]) -> T
where
    T: From<bool>
        + std::ops::Shl<usize, Output = T>
        + std::ops::BitOr<Output = T>,
{
    data.into_iter()
        .rev()
        .enumerate()
        .map(|(i, d)| T::from(*d) << i)
        .reduce(|acc, e| acc.bitor(e))
        .unwrap()
}
