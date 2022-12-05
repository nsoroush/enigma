#![allow(unused)]
//use shuffle::shuffler;
use rand::Rng;
use rand::thread_rng;
use rand::seq::SliceRandom;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";


fn main(){
    let enigma_key = enigma_key_generation();
    let mut rotor_1 : String  = enigma_key[0].clone();
    let mut rotor_2 : String  = enigma_key[1].clone();
    let mut rotor_3 : String  = enigma_key[2].clone();

    println!("the first rotor {}", rotor_1);

    for i in 0..3{
        println!(" {}: {}", i , enigma_key[i]);
    }
}

fn encryption(key:[String;3], message : &str) -> String{
    let status :usize =0;
    let l :usize = message.len();

    for i in 0..l{


    }



}

fn reflector(ch: char) -> char{
    let mut reflected_char : char = ' ';
    let a = index_in_string(ch, ALPHABET);
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
