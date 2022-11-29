use itertools::Itertools;

use crate::solve;

pub fn run() {
    let day_number = 16;
    solve!(&2021, &day_number, parse, task_1, task_2);
}

fn task_1(data: &ParsedInput) -> String {
    let packets = parse_packets(data.input.clone(), usize::MAX);

    sum_versions(&packets).to_string()
}

fn task_2(data: &ParsedInput) -> String {
    let packets = parse_packets(data.input.clone(), usize::MAX);
    packets.first().unwrap().eval().to_string()
}

fn parse_packets(mut input: String, mut max: usize) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];
    while input.contains('1') && max > 0 {
        max -= 1;
        let version_bits: String = input.drain(..3).collect();
        let type_id_bits: String = input.drain(..3).collect();
        let version: usize = string_to_usize(&version_bits);
        let type_id: usize = string_to_usize(&type_id_bits);

        if type_id == 4 {
            // get chunks of 5. if the if the first bit is a 0  this was the last chunk
            let mut value_bits = String::from("");
            let mut raw_bits = String::from("");
            loop {
                let mut next: String = input.drain(0..5).collect();
                raw_bits = format!("{}{}", raw_bits, next);
                let ending_indicator: String = next.drain(..1).collect();
                value_bits = format!("{}{}", value_bits, next);

                if ending_indicator == "0" {
                    break;
                }
            }
            packets.push(Packet {
                version,
                raw: format!("{}{}{}", version_bits, type_id_bits, raw_bits),
                decimal_value: Some(string_to_usize(&value_bits)),
                operator: None,
                sub_packets: None,
            });
        } else {
            let length_type_id: String = input.drain(..1).collect();
            let length_raw: String;
            let sub_packets: Option<Vec<Packet>>;
            let operator = match type_id {
                0 => Operator::Sum,
                1 => Operator::Product,
                2 => Operator::Minimum,
                3 => Operator::Maximum,
                5 => Operator::Greater,
                6 => Operator::Less,
                7 => Operator::Eq,
                _ => panic!(),
            };
            if length_type_id == "0" {
                // get the next 15 bits. this is the length of the subpackets in this
                // operator packet
                length_raw = input.drain(0..15).collect();
                let sub_packet_bit_length = string_to_usize(&length_raw);

                let sub_packet_input = input.drain(0..sub_packet_bit_length).collect();

                sub_packets = Some(parse_packets(sub_packet_input, usize::MAX));
            } else {
                //  get the nex 11 bits, this represents the number of subpackets
                length_raw = input.drain(0..11).collect();
                let sub_packet_count = string_to_usize(&length_raw);
                sub_packets = Some(parse_packets(input.clone(), sub_packet_count));

                let removed_bit_count = count_bit_length(&sub_packets.clone().unwrap());

                input.drain(..removed_bit_count);
            }
            packets.push(Packet {
                version,
                raw: format!(
                    "{}{}{}{}",
                    version_bits, type_id_bits, length_type_id, length_raw
                ),
                decimal_value: None,
                operator: Some(operator),
                sub_packets,
            });
        }
    }

    packets
}

fn count_bit_length(packets: &[Packet]) -> usize {
    packets.iter().fold(0, |acc, packet| {
        if let Some(sub_packets) = &packet.sub_packets {
            acc + packet.raw.chars().count() + count_bit_length(sub_packets)
        } else {
            acc + packet.raw.chars().count()
        }
    })
}

fn sum_versions(packets: &[Packet]) -> usize {
    packets.iter().fold(0, |acc, packet| {
        if let Some(sub_packets) = &packet.sub_packets {
            acc + packet.version + sum_versions(sub_packets)
        } else {
            acc + packet.version
        }
    })
}

fn string_to_usize(s: &str) -> usize {
    usize::from_str_radix(s, 2).unwrap()
}

#[derive(Debug)]
struct ParsedInput {
    input: String,
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    raw: String,
    decimal_value: Option<usize>,
    operator: Option<Operator>,
    sub_packets: Option<Vec<Packet>>,
}

impl Packet {
    fn eval(&self) -> i64 {
        if let Some(operator) = &self.operator {
            let cloned = self.sub_packets.clone().unwrap();
            let evaluated = cloned.iter().map(|sp| sp.eval());
            match &operator {
                Operator::Sum => evaluated.sum::<i64>(),
                Operator::Product => evaluated.product(),
                Operator::Minimum => evaluated.min().unwrap(),
                Operator::Maximum => evaluated.max().unwrap(),
                Operator::Greater => {
                    let elements: Vec<i64> = evaluated.take(2).collect();
                    if elements[0] > elements[1] {
                        1
                    } else {
                        0
                    }
                }
                Operator::Less => {
                    let elements: Vec<i64> = evaluated.take(2).collect();
                    if elements[0] < elements[1] {
                        1
                    } else {
                        0
                    }
                }
                Operator::Eq => {
                    let elements: Vec<i64> = evaluated.take(2).collect();
                    if elements[0] == elements[1] {
                        1
                    } else {
                        0
                    }
                }
            }
        } else {
            self.decimal_value.unwrap() as i64
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    Greater,
    Less,
    Eq,
}

fn parse(input: &str) -> ParsedInput {
    let parsed = input
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'F' => "1111",
            'E' => "1110",
            _ => "",
        })
        .filter(|s| !s.is_empty())
        .join("");

    ParsedInput { input: parsed }
}

#[test]
fn test() {
    let test_input_1 = String::from("8A004A801A8002F478");

    let parsed_1 = parse(&test_input_1);
    assert_eq!(task_1(&parsed_1), "16");

    let test_input_2 = String::from("620080001611562C8802118E34");
    let parsed_2 = parse(&test_input_2);
    assert_eq!(task_1(&parsed_2), "12");

    let test_input_3 = String::from("C0015000016115A2E0802F182340");
    let parsed_3 = parse(&test_input_3);
    assert_eq!(task_1(&parsed_3), "23");

    let test_input_4 = String::from("A0016C880162017C3686B18A3D4780");
    let parsed_4 = parse(&test_input_4);
    assert_eq!(task_1(&parsed_4), "31");

    let test_input_5 = String::from("C200B40A82");
    let parsed_5 = parse(&test_input_5);
    assert_eq!(task_2(&parsed_5), "3");

    let test_input_6 = String::from("04005AC33890");
    let parsed_6 = parse(&test_input_6);
    assert_eq!(task_2(&parsed_6), "54");

    let test_input_7 = String::from("880086C3E88112");
    let parsed_7 = parse(&test_input_7);
    assert_eq!(task_2(&parsed_7), "7");

    let test_input_8 = String::from("CE00C43D881120");
    let parsed_8 = parse(&test_input_8);
    assert_eq!(task_2(&parsed_8), "9");

    let test_input_9 = String::from("D8005AC2A8F0");
    let parsed_9 = parse(&test_input_9);
    assert_eq!(task_2(&parsed_9), "1");

    let test_input_10 = String::from("F600BC2D8F");
    let parsed_10 = parse(&test_input_10);
    assert_eq!(task_2(&parsed_10), "0");

    let test_input_11 = String::from("9C005AC2F8F0");
    let parsed_11 = parse(&test_input_11);
    assert_eq!(task_2(&parsed_11), "0");

    let test_input_12 = String::from("9C0141080250320F1802104A08");
    let parsed_12 = parse(&test_input_12);
    assert_eq!(task_2(&parsed_12), "1");

    let test_input_13 = String::from("0200840080");
    let parsed_13 = parse(&test_input_13);
    assert_eq!(task_2(&parsed_13), "0");
}
