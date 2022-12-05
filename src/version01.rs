#![allow(unused)]

use std::array;
use rand::seq::SliceRandom;
// Globals are declared outside all other scopes.
static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    //println!(ALPHABET.find('a'));
    let enigma_key = enigma_key_generation();
    let mut rotor_1 : String  = enigma_key[0].clone();
    let mut rotor_2 : String  = enigma_key[1].clone();
    let mut rotor_3 : String  = enigma_key[2].clone();

    rotor_1 = enigma_key[0].clone();
    println!("the first rotor {}", rotor_1);
//    let rotor_2 = enigma_key[1].clone();
    //let rotor_3 = enigma_key[2].clone();
    
    for i in 0..3{
        println!(" {}: {}", i , enigma_key[i]);
    }
        for i in ALPHABET.chars(){
        println!("This {} reflect to {}", i , reflector(i));
    }
    
    let i = index_in_string('z', ALPHABET);
    println!("index is {}, char is {}",i,  ALPHABET.chars().nth(25 - i).unwrap());
}

/* 
fn enigma(ch: char, rotor_1, rotor_2, rotor_3) -> char
{ 
    let mut char_1 = rotor_1[index_in]
}
*/

fn enigma_key_generation() -> [String;3] {
    println!(" this is my key");
    let mut key : [String;3] = Default::default();
    for i in 0..3{
        let mut njm_rng = rand::thread_rng();
        let mut bytes = ALPHABET.to_string().into_bytes();
        bytes.shuffle(&mut njm_rng);
        let r = String::from_utf8(bytes).unwrap();
        key[i] = r;
    }
    return key;
}

fn reflector(ch: char) -> char{
    let mut reflected_char : char = ' ';
    let a = index_in_string(ch, ALPHABET );
    reflected_char = ALPHABET.chars().nth(26 - a - 1).unwrap();
    return reflected_char;
}
fn index_in_string(ch: char, my_string: &str ) -> usize
{
    let mut result_char : char = ' ';
        let a = match my_string.find(ch) {
            Some(i) => i,
            None => 27
            };
        let index = usize::try_from(a).unwrap();
        return index;
        }

fn rotate(temp_str: &str) -> String {
    let mut str_vec: Vec<char> = temp_str.chars().collect();
    str_vec.rotate_left(count);
    str_vec.iter().collect()
}
