use crate::libs::constant::{MAGIC};

fn load_4_chars(key : &Vec<u8>) -> u32 {
    let len : usize = key.len();

    // Take first 2 byte of key
    let first_2_byte : u32 = (key[0] as u32) | (key[1] as u32) << 8;

    // Take last 2 byte of key
    let last_2_byte : u32 = (key[len-2] as u32) | (key[len - 1] as u32) << 8;

    last_2_byte << 16 | first_2_byte
}

fn mix_4_byte(char_4_byte : u32 , seed : u64) -> u32 {
    let x = (char_4_byte as u64) ^ seed;
    let res = ((x as u128 * MAGIC as u128) >> 64) as u64;
    res as u32
}

pub fn hash(key : &Vec<u8>,table_size : usize,seed : u64) -> Result<usize,&'static str> {
    if key.len() < 2 || key.len() > 11 {
        return Err("Not support keyword");
    }

    let char_4_byte : u32 = load_4_chars(key);
    let mixed : u32 = mix_4_byte(char_4_byte, seed);
    let index : usize = mixed as usize % table_size;
    Ok(index)
}

pub fn ihash(key : &usize,table_size : &usize,seed : &u64) -> usize {
    let mixed : u32 = mix_4_byte(*key as u32, *seed);
    let index : usize = mixed as usize % table_size;
    index
}