use nom::bits::{bits, complete::take, complete::tag};
use nom::bytes;
use nom::error::Error;
use nom::IResult;

type ParseState<'a> = (&'a[u8], usize);

pub struct Packet {
    version: u8,
    type_id: u8,
    payload: Vec<u8>,
    offset: usize
}

#[aoc_generator(day16)]
pub fn generator(input: &str) -> Vec<u8> {
    input
        .chars()
        .filter(|c| ('0'..='9').contains(c) || ('A'..='F').contains(c))
        .collect::<Vec<char>>()
        .chunks(2)
        .inspect(|x| println!("{:?}", x))
        .map(|byte_chunk| byte_chunk.iter().collect::<String>())
        .map(|byte_hex| u8::from_str_radix(&byte_hex, 16).unwrap())
        .collect()
}

pub fn parse_header(i: ParseState) -> IResult<ParseState, (u8, u8)> {
    let (i, version) = take(3usize)(i)?;
    let (i, type_id) = take(3usize)(i)?;
    Ok((i, (version, type_id)))
}

pub fn parse_packet(i: ParseState) {

}

pub fn parse_operator(i: ParseState) -> IResult<ParseState, ()> {
    let (i, mode): (_, u16) = take(1usize)(i)?;
    if mode == 0 {
        let (i, length_bits): (_, u16) = take(15usize)(i)?;
        println!("Subpackets with size {}", length_bits);
        let (i, data): (_, &[u8]) = take(nom::bytes::bytes(length_bits))(i)?;
        // parse_buffer()
    } else {
        let (i, length_packets): (_, u16) = take(11usize)(i)?;
        println!("Subpackets with count {}", length_packets);
        for j in 0..length_packets {
            parse_packet(i);
        }
    };
    Ok((i,()))
}

pub fn parse_buffer(i: ParseState) -> IResult<ParseState, Packet> {
    let (i, (version, type_id)) = parse_header(i)?;
    let payload = i.0.iter().copied().collect();
    let offset = i.1;

    println!("Found packet un buffer version={} type={}", version, type_id);

    if type_id != 4 {
        parse_operator(i)?;
    }

    Ok((
        (i.0, i.0.len()),
        Packet {
            version,
            type_id,
            payload,
            offset
        }
    ))
}

#[aoc(day16, part1)]
pub fn part1(input: &[u8]) -> i32 {
    let output: IResult<&[u8], Packet> = bits(parse_buffer)(input);
    0
}


//pub fn parser(input: &[u8]) {
//    bits(parse_header)(input);
//}

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
        let output: IResult<&[u8], (u8, u8)> = bits(parse_header)(&bytes);
        let output = output.unwrap();
        assert_eq!(output.1.0, 1);
        assert_eq!(output.1.1, 6);
    }
}
