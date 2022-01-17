use std::fs::File;
use std::io::{self, BufRead};
use std::convert::TryFrom;

#[derive(Debug)]
struct Packet {
    version: u8,
    r#type: u8,
    length_id: bool,
}

fn take_bits(vec: &[u8], start_bit: usize, size: usize) -> u64 {
    let mut result: u64 = 0;

    for i in start_bit..start_bit + size {
        let val = (((vec[i/4] as usize & (1 << (3 - (i % 4))) != 0) as u64) << (size - (i - start_bit) - 1)) as u64;
        result |= val;
    }
    result
}

fn read_packet(data: &[u8], start_offset: usize) -> (u64, usize) {

    let p = Packet {
        version: take_bits(data, start_offset, 3) as u8,
        r#type: take_bits(data, start_offset + 3, 3) as u8,
        length_id: take_bits(data, start_offset + 6, 1) != 0,
    };

    let mut value: u64 = 0;
    let mut bits_read: usize = 0;

    if p.r#type == 4 { // literal value
        loop {
            let last_group: bool = take_bits(data, start_offset + 6 + bits_read, 1) == 0;
            let v = take_bits(data, start_offset + 6 + 1 + bits_read, 4);
            value <<= 4;
            value |= v;
            bits_read += 5;
            if last_group {
                bits_read += 6;
                break;
            }
        }
    } else {
        let length: u64;
        let mut values: Vec<u64> = Vec::new();
        match p.length_id {
            true => {
                length = take_bits(data, start_offset + 7, 11);
                // for each subpacket
                for _ in 0..length {
                    let res = read_packet(data, start_offset + 18 + bits_read);
                    values.push(res.0);

                    bits_read += usize::try_from(res.1).unwrap();
                }
                bits_read += 18;
            },
            false => {
                length = take_bits(data, start_offset + 7, 15);

                let mut packet_bits_read = 0;
                loop {
                    let res = read_packet(data, start_offset + 22 + packet_bits_read);
                    values.push(res.0);
                    packet_bits_read += usize::try_from(res.1).unwrap();


                    if packet_bits_read >= usize::try_from(length).unwrap() {
                        break;
                    }                }
                bits_read = packet_bits_read + 22;
            },
        }
        match p.r#type {
            0 => value = values.iter().fold(0, |acc, x| acc + x),
            1 => value = values.iter().fold(1, |mul, x| mul * x),
            2 => value = *values.iter().min().unwrap(),
            3 => value = *values.iter().max().unwrap(),
            5 => {
                if values.len() != 2 {
                    panic!("packed with ID 5 must have 2 values");
                } else {
                    value = (values[0] > values[1]) as u64;
                }
            },
            6 => {
                if values.len() != 2 {
                    panic!("packed with ID 6 must have 2 values");
                } else {
                    value = (values[0] < values[1]) as u64;
                }
            },
            7 => {
                if values.len() != 2 {
                    panic!("packed with ID 7 must have 2 values");
                } else {
                    value = (values[0] == values[1]) as u64;
                }
            },
            _ => unreachable!(),
        }
    }

    (value, bits_read)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let data: Vec<Vec<u8>> = lines
        .map(|line| line
             .unwrap()
             .chars()
             .map(|x| x.to_digit(16).unwrap() as u8)
             .collect()
             )
        .collect();
    

    let res = read_packet(&data[0], 0);

    println!("Result: {}", res.0);

    Ok(())
}

