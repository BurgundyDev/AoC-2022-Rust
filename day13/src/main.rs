use std::cmp::Ordering;
use std::num::ParseIntError;
use std::str::FromStr;


use aoc2022::read_file;

fn main() {
    day13("test_input.txt");
    day13("input.txt");
}

#[derive(Debug, Clone, Eq)]
enum Packet {
    Number(u32),
    Values(Vec<Self>),
}

impl Packet {
    pub fn as_slice(&self) -> &[Self] {
        if let Self::Values(values) = self {
            values.as_slice()
        } else {
            std::slice::from_ref(self)
        }
    }

    fn parse_one(s: &str) -> (Self, &str) {
        if let Some(mut s) = s.strip_prefix('[') {
            let mut values = vec![];
            if let Some(trailing) = s.strip_prefix(']') {
                return (Self::Values(values), trailing);
            }
            loop {
                let (value, trailing) = Self::parse_one(s);
                values.push(value);
                let (c, trailing) = {
                    let mut chars = trailing.chars();
                    (chars.next(), chars.as_str())
                };
                match c {
                    Some(',') => (),
                    Some(']') => return (Self::Values(values), trailing),
                    _ => ()
                };
                s = trailing;
            }
        } else {
            let terminator = s.find([',', ']']).unwrap_or(s.len());
            let (s, trailing) = s.split_at(terminator);
            (Self::Number(s.parse().unwrap()), trailing)
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if let (Self::Number(a), Self::Number(b)) = (self, other) {
            a.cmp(b)
        } else {
            self.as_slice().cmp(other.as_slice())
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl FromStr for Packet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Packet, ParseIntError> {
        let (value, _trailing) = Self::parse_one(s);
        return Ok(value);
    }
}

#[macro_export]
macro_rules! packet {
    ($n:literal) => {
        $crate::Packet::Number($n)
    };
    ([$($i:tt),*]) => {
        $crate::Packet::Values(vec![
            $(
                $crate::packet!($i)
            ),*
        ])
    };
}

fn day13(file_path: &str) {
    let divider_a = packet!([[2]]);
    let divider_b = packet!([[6]]);
    let mut all_packets = vec![divider_a.clone(), divider_b.clone()];
    let input = read_file(file_path);
    let mut lines = input.lines();
    let mut right_order_pairs = 0usize;
    
    for pair_num in 1.. {
        let a = lines.next().unwrap().parse().unwrap();
        let b = lines.next().unwrap().parse().unwrap();

        if a <= b {
            right_order_pairs += pair_num;
        }
        all_packets.push(a);
        all_packets.push(b);

        match lines.next() {
            None => break,
            Some("") => continue,
            Some(other) => panic!("Expected blank line, not {other:?}"),
        }
    }
    println!("Correctly ordered pairs: {right_order_pairs}");
    

    all_packets.sort();
    let mut decoder_key = 1;
    for (i, packet) in all_packets.iter().enumerate() {
        if packet == &divider_a || packet == &divider_b {
            decoder_key *= i + 1;
        }
    }

    println!("Decoder key: {decoder_key}");
}