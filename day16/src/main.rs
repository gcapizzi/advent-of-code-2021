use anyhow::{anyhow, Result};
use std::fs;

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let bin_input = hex_to_bin(&input.trim())?;
    let (packet, _) = Packet::parse(&bin_input)?;
    dbg!(sum_versions(&packet));
    dbg!(packet.eval());
    Ok(())
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: usize,
    body: Body,
}

#[derive(Debug, PartialEq)]
enum Body {
    Literal(u64),
    Operator(Operation, Vec<Packet>),
}

#[derive(Debug, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl Packet {
    fn parse(s: &str) -> Result<(Packet, &str)> {
        if s.len() < 6 {
            return Err(anyhow!("string too short!"));
        }

        let (version_str, rest) = s.split_at(3);
        let version = usize::from_str_radix(version_str, 2)?;

        let (type_id_str, rest) = rest.split_at(3);

        let (body, rest) = if type_id_str == "100" {
            parse_literal(rest)?
        } else {
            let (subpackets, rest) = parse_subpackets(rest)?;

            let op = match type_id_str {
                "000" => Ok(Operation::Sum),
                "001" => Ok(Operation::Product),
                "010" => Ok(Operation::Minimum),
                "011" => Ok(Operation::Maximum),
                "101" => Ok(Operation::GreaterThan),
                "110" => Ok(Operation::LessThan),
                "111" => Ok(Operation::Equal),
                _ => Err(anyhow!("invalid operation: {}", type_id_str)),
            }?;

            (Body::Operator(op, subpackets), rest)
        };

        Ok((Packet { version, body }, rest))
    }

    fn eval(&self) -> u64 {
        match &self.body {
            Body::Literal(n) => *n,
            Body::Operator(Operation::Sum, ps) => ps.iter().map(|p| p.eval()).sum(),
            Body::Operator(Operation::Product, ps) => ps.iter().map(|p| p.eval()).product(),
            Body::Operator(Operation::Minimum, ps) => ps.iter().map(|p| p.eval()).min().unwrap(),
            Body::Operator(Operation::Maximum, ps) => ps.iter().map(|p| p.eval()).max().unwrap(),
            Body::Operator(Operation::GreaterThan, ps) => {
                if &ps[0].eval() > &ps[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Operator(Operation::LessThan, ps) => {
                if &ps[0].eval() < &ps[1].eval() {
                    1
                } else {
                    0
                }
            }
            Body::Operator(Operation::Equal, ps) => {
                if &ps[0].eval() == &ps[1].eval() {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn parse_all_packets(s: &str) -> Vec<Packet> {
    let mut rest = s;
    let mut packets = vec![];
    loop {
        if let Ok((p, r)) = Packet::parse(rest) {
            packets.push(p);
            rest = r;
        } else {
            return packets;
        }
    }
}

fn parse_packets(n: usize, s: &str) -> Result<(Vec<Packet>, &str)> {
    let mut rest = s;
    let mut packets = vec![];
    for _ in 0..n {
        let (p, r) = Packet::parse(rest)?;
        packets.push(p);
        rest = r;
    }
    Ok((packets, rest))
}

fn parse_literal(s: &str) -> Result<(Body, &str)> {
    let mut cur = s;
    let mut bin = "".to_string();

    loop {
        let (chunk, rest) = cur.split_at(5);
        cur = rest;
        bin.push_str(&chunk[1..]);
        if chunk.chars().nth(0) == Some('0') {
            break;
        }
    }

    let num = u64::from_str_radix(&bin, 2)?;

    Ok((Body::Literal(num), cur))
}

fn parse_subpackets(s: &str) -> Result<(Vec<Packet>, &str)> {
    let (length_id_str, rest) = s.split_at(1);
    match length_id_str {
        "0" => {
            let (length_str, rest) = rest.split_at(15);
            let length = usize::from_str_radix(&length_str, 2)?;
            let (subpackets_str, rest) = rest.split_at(length);
            let subpackets = parse_all_packets(subpackets_str);
            Ok((subpackets, rest))
        }
        "1" => {
            let (n_str, rest) = rest.split_at(11);
            let n = usize::from_str_radix(&n_str, 2)?;
            let (subpackets, rest) = parse_packets(n, rest)?;
            Ok((subpackets, rest))
        }
        _ => Err(anyhow!("invalid length id: {}", length_id_str)),
    }
}

fn sum_versions(p: &Packet) -> usize {
    match p {
        Packet {
            version: v,
            body: Body::Literal(_),
        } => *v,
        Packet {
            version: v,
            body: Body::Operator(_, ps),
        } => *v + ps.iter().map(|p| sum_versions(p)).sum::<usize>(),
    }
}

fn hex_to_bin(hex: &str) -> Result<String> {
    hex.chars()
        .map(|c| match c {
            '0' => Ok("0000"),
            '1' => Ok("0001"),
            '2' => Ok("0010"),
            '3' => Ok("0011"),
            '4' => Ok("0100"),
            '5' => Ok("0101"),
            '6' => Ok("0110"),
            '7' => Ok("0111"),
            '8' => Ok("1000"),
            '9' => Ok("1001"),
            'A' => Ok("1010"),
            'B' => Ok("1011"),
            'C' => Ok("1100"),
            'D' => Ok("1101"),
            'E' => Ok("1110"),
            'F' => Ok("1111"),
            _ => Err(anyhow!("invalid hex char: {}", c)),
        })
        .collect::<Result<String>>()
}

#[cfg(test)]
mod tests {
    use crate::{hex_to_bin, sum_versions, Body, Operation, Packet};
    use anyhow::Result;

    #[test]
    fn test_parse_literal() -> Result<()> {
        let input = "110100101111111000101000";
        let (packet, rest) = Packet::parse(&input)?;

        assert_eq!(
            packet,
            Packet {
                version: 6,
                body: Body::Literal(2021)
            }
        );
        assert_eq!(rest, "000");

        Ok(())
    }

    #[test]
    fn test_parse_operator_length_id_0() -> Result<()> {
        let input = "00111000000000000110111101000101001010010001001000000000";
        let (packet, rest) = Packet::parse(&input)?;

        assert_eq!(
            packet,
            Packet {
                version: 1,
                body: Body::Operator(
                    Operation::LessThan,
                    vec![
                        Packet {
                            version: 6,
                            body: Body::Literal(10)
                        },
                        Packet {
                            version: 2,
                            body: Body::Literal(20)
                        },
                    ]
                )
            }
        );
        assert_eq!(rest, "0000000");

        Ok(())
    }

    #[test]
    fn test_parse_operator_length_id_1() -> Result<()> {
        let input = "11101110000000001101010000001100100000100011000001100000";
        let (packet, rest) = Packet::parse(&input)?;

        assert_eq!(
            packet,
            Packet {
                version: 7,
                body: Body::Operator(
                    Operation::Maximum,
                    vec![
                        Packet {
                            version: 2,
                            body: Body::Literal(1)
                        },
                        Packet {
                            version: 4,
                            body: Body::Literal(2)
                        },
                        Packet {
                            version: 1,
                            body: Body::Literal(3)
                        }
                    ]
                )
            }
        );
        assert_eq!(rest, "00000");

        Ok(())
    }

    #[test]
    fn test_sum_versions() -> Result<()> {
        let (packet, _) = Packet::parse(&hex_to_bin("8A004A801A8002F478")?)?;
        assert_eq!(sum_versions(&packet), 16);
        let (packet, _) = Packet::parse(&hex_to_bin("620080001611562C8802118E34")?)?;
        assert_eq!(sum_versions(&packet), 12);
        let (packet, _) = Packet::parse(&hex_to_bin("C0015000016115A2E0802F182340")?)?;
        assert_eq!(sum_versions(&packet), 23);
        let (packet, _) = Packet::parse(&hex_to_bin("A0016C880162017C3686B18A3D4780")?)?;
        assert_eq!(sum_versions(&packet), 31);

        Ok(())
    }

    #[test]
    fn test_eval() -> Result<()> {
        let (packet, _) = Packet::parse(&hex_to_bin("C200B40A82")?)?;
        assert_eq!(packet.eval(), 3);
        let (packet, _) = Packet::parse(&hex_to_bin("04005AC33890")?)?;
        assert_eq!(packet.eval(), 54);
        let (packet, _) = Packet::parse(&hex_to_bin("880086C3E88112")?)?;
        assert_eq!(packet.eval(), 7);
        let (packet, _) = Packet::parse(&hex_to_bin("CE00C43D881120")?)?;
        assert_eq!(packet.eval(), 9);
        let (packet, _) = Packet::parse(&hex_to_bin("D8005AC2A8F0")?)?;
        assert_eq!(packet.eval(), 1);
        let (packet, _) = Packet::parse(&hex_to_bin("F600BC2D8F")?)?;
        assert_eq!(packet.eval(), 0);
        let (packet, _) = Packet::parse(&hex_to_bin("9C005AC2F8F0")?)?;
        assert_eq!(packet.eval(), 0);
        let (packet, _) = Packet::parse(&hex_to_bin("9C0141080250320F1802104A08")?)?;
        assert_eq!(packet.eval(), 1);

        Ok(())
    }
}
