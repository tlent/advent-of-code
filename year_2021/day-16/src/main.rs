const INPUT: &str = include_str!("../input");

fn main() {
    let bits = Bits::from_hex_string(INPUT.trim());
    let root_packet = Packet::from_bits(bits);
    let mut version_sum = 0;
    let mut stack = vec![&root_packet];
    while let Some(p) = stack.pop() {
        version_sum += p.version as u32;
        stack.extend(&p.subpackets);
    }
    println!("{}", version_sum);
    println!("{}", root_packet.value);
}

#[derive(Debug, Clone)]
struct Bits {
    bytes: Vec<u8>,
    index: u32,
}

impl Bits {
    pub fn from_hex_string(hex_string: &str) -> Self {
        let mut hex_chars = hex_string.chars();
        let mut bytes = vec![];
        while let Some(c) = hex_chars.next() {
            let a = c.to_digit(16).unwrap() as u8;
            let b = hex_chars
                .next()
                .map(|c| c.to_digit(16).unwrap() as u8)
                .unwrap_or_default();
            let mut byte = 0;
            byte |= a << 4;
            byte |= b;
            bytes.push(byte);
        }
        Self { bytes, index: 0 }
    }

    pub fn next_bits(&mut self, n: u32) -> u32 {
        let mut v = 0;
        for i in self.index..self.index + n {
            let byte_i = (i / 8) as usize;
            let bit = (self.bytes[byte_i] >> (7 - (i % 8))) & 1;
            v <<= 1;
            v |= bit as u32;
        }
        self.index += n;
        v
    }

    pub fn bits_read(&self) -> u32 {
        self.index
    }
}

#[derive(Debug)]
struct Packet {
    version: u8,
    value: u64,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn from_bits(mut bits: Bits) -> Self {
        Self::_from_bits(&mut bits)
    }

    fn _from_bits(bits: &mut Bits) -> Self {
        let version = bits.next_bits(3) as u8;
        let packet_type_id = bits.next_bits(3);
        if packet_type_id == 4 {
            {
                let mut value = 0;
                let mut is_last_byte = false;
                while !is_last_byte {
                    is_last_byte = bits.next_bits(1) == 0;
                    value <<= 4;
                    value |= bits.next_bits(4) as u64;
                }
                return Self {
                    version,
                    value,
                    subpackets: vec![],
                };
            }
        }
        let length_type = bits.next_bits(1);
        let subpackets = match length_type {
            0 => {
                let subpackets_bit_length = bits.next_bits(15);
                let initial_bits_read = bits.bits_read();
                let mut packets = vec![];
                while bits.bits_read() - initial_bits_read < subpackets_bit_length {
                    packets.push(Packet::_from_bits(bits));
                }
                packets
            }
            1 => {
                let subpacket_count = bits.next_bits(11);
                let mut packets = vec![];
                while (packets.len() as u32) < subpacket_count {
                    packets.push(Packet::_from_bits(bits));
                }
                packets
            }
            _ => unreachable!(),
        };
        let value = match packet_type_id {
            0 => subpackets.iter().map(|p| p.value).sum(),
            1 => subpackets.iter().map(|p| p.value).product(),
            2 => subpackets.iter().map(|p| p.value).min().unwrap(),
            3 => subpackets.iter().map(|p| p.value).max().unwrap(),
            5 => {
                if subpackets[0].value > subpackets[1].value {
                    1
                } else {
                    0
                }
            }
            6 => {
                if subpackets[0].value < subpackets[1].value {
                    1
                } else {
                    0
                }
            }
            7 => {
                if subpackets[0].value == subpackets[1].value {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        };
        Self {
            version,
            value,
            subpackets,
        }
    }
}
