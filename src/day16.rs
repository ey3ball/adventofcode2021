use nom::bits::{bits, complete::take, complete::tag};
use nom::bytes;
use nom::error::Error;
use nom::IResult;

type ParseState<'a> = (&'a[u8], usize);

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: usize,
    op: PacketOp
}

impl Packet {
    fn literal(version: usize, val: u64) -> Packet {
        Packet {
            version,
            op: PacketOp::Literal(val)
        }
    }

    fn op(version: usize, packets: Vec<Packet>) -> Packet {
        Packet {
            version,
            op: PacketOp::Operator(packets)
        }
    }

    fn sum(&self) -> usize {
        self.version + match &self.op {
            PacketOp::Literal(_) => 0,
            PacketOp::Operator(ps) => ps.iter().map(|p| p.sum()).sum()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PacketOp {
    Literal(u64),
    Operator(Vec<Packet>)
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter(|c| ('0'..='9').contains(c) || ('A'..='F').contains(c))
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|byte_chunk| byte_chunk.iter().collect::<String>())
        .map(|byte_hex| u8::from_str_radix(&byte_hex, 16).unwrap())
        .collect()
}

pub fn parse_header(i: ParseState) -> IResult<ParseState, (usize, u8)> {
    let (i, version) = take(3usize)(i)?;
    let (i, type_id) = take(3usize)(i)?;
    Ok((i, (version, type_id)))
}

pub fn parse_operator(i: ParseState) -> IResult<ParseState, PacketOp> {
    let (mut i, mode): (_, u16) = take(1usize)(i)?;
    let mut packets: Vec<Packet> = vec![];

    if mode == 0 {
        let (start, length_bits): (_, usize) = take(15usize)(i)?;
        println!("  > Operator Subpackets with size {}", length_bits);

        i = start;
        loop {
            let (state, packet) = parse_packet(i)?;
            packets.push(packet);
            i = state;

            /* Stop once we're parsed length bits */
            if (start.0.len() * 8 - start.1 - state.0.len() * 8 + state.1) >= length_bits {
                break;
            }
        }
    } else {
        let (start, length_packets): (_, u16) = take(11usize)(i)?;
        i = start;

        println!("  > Operator Subpackets with count {}", length_packets);
        for _j in 0..length_packets {
            let (state, packet) = parse_packet(i)?;
            i = state;
            packets.push(packet);
        }
    };
    Ok((i,PacketOp::Operator(packets)))
}

pub fn parse_literal(i: ParseState) -> IResult<ParseState, PacketOp> {
    let mut value: u64 = 0;
    let mut i = i;
    loop {
        let (state, not_last): (_, usize) = take(1usize)(i)?;
        let (state, add_bits): (_, u8) = take(4usize)(state)?;
        value = (value << 4) + add_bits as u64;

        i = state;
        if not_last == 0 {
            break;
        }
    }
    println!("Parsed Literal Packet {}", value);

    Ok((i,PacketOp::Literal(value)))
}

pub fn parse_packet(i: ParseState) -> IResult<ParseState, Packet> {
    let (i, (version, type_id)) = parse_header(i)?;

    println!("Read Packet Header version={} type={}", version, type_id);

    let (i, op) = if type_id != 4 {
        parse_operator(i)?
    } else {
        parse_literal(i)?
    };

    Ok((i, Packet { version, op }))
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> usize {
    let output: IResult<&[u8], Packet> = bits(parse_packet)(input);
    output.unwrap().1.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_bytes() {
        assert_eq!(generator("00AB01FF"), vec![0x0, 0xAB, 0x01, 0xFF]);
        assert_eq!(generator("00AB\n01FF\n"), vec![0x0, 0xAB, 0x01, 0xFF]);
    }

    #[test]
    fn test_parse_header() {
        let bytes = generator("38");
        let output: IResult<&[u8], (usize, u8)> = bits(parse_header)(&bytes);
        let output = output.unwrap();
    assert_eq!(output.1.0, 1);
    assert_eq!(output.1.1, 6);
    }

    #[test]
    fn parse_literal_packet() {
        let bytes = generator("D2FE28");
        let output: IResult<&[u8], Packet> = bits(parse_packet)(&bytes);
        let output = output.unwrap();
        assert_eq!(output.1, Packet::literal(6,2021));
    }

    #[test]
    fn parse_operator_packet() {
        let bytes = generator("38006F45291200");
        let output: IResult<&[u8], Packet> = bits(parse_packet)(&bytes);
        let output = output.unwrap();
        assert_eq!(output.1, Packet::op(1, vec![Packet::literal(6, 10),Packet::literal(2, 20)]))
    }

    #[test]
    fn parse_operator_packet_2() {
        let bytes = generator("EE00D40C823060");
        let output: IResult<&[u8], Packet> = bits(parse_packet)(&bytes);
        let output = output.unwrap();
        assert_eq!(output.1, Packet::op(7, vec![Packet::literal(2, 1),Packet::literal(4, 2),Packet::literal(1, 3)]))
    }

    fn get_sum(input: &str) -> usize {
        let bytes = generator(input);
        let output: IResult<&[u8], Packet> = bits(parse_packet)(&bytes);
        output.unwrap().1.sum()
    }

    #[test]
    fn calc_sums() {
        assert_eq!(get_sum("8A004A801A8002F478"), 16);
        assert_eq!(get_sum("620080001611562C8802118E34"), 12);
        assert_eq!(get_sum("C0015000016115A2E0802F182340"), 23);
        assert_eq!(get_sum("A0016C880162017C3686B18A3D4780"), 31);
    }
}
