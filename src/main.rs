use std::io;
use std::array;
use rand::seq::SliceRandom;

static ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let enigma_key = enigma_key_generation();
    let input = get_input(String::from("Please enter your message!"));
    let message = input.trim();
    println!("{}", message);
    let ciphertext: String= enigma(enigma_key,&message);
    println!(" Enigma ciphertext is:  {}", ciphertext);
}

fn enigma(key: [String;3], plaintext: &str) -> String{
    let mut rotor_1 : String  = key[0].clone();
    let mut rotor_2 : String  = key[1].clone();
    let mut rotor_3 : String  = key[2].clone();
    let mut status : usize = 0;
    let mut ciphertext: String= String::from("");
    for ch in plaintext.chars(){
        let mut ch_1 = rotor_1.chars().nth(index_in_string(ch, ALPHABET)).unwrap();
        let mut ch_2 = rotor_2.chars().nth(index_in_string(ch_1, ALPHABET)).unwrap();
        let mut ch_3 = rotor_3.chars().nth(index_in_string(ch_2, ALPHABET)).unwrap();
        let reflected_char =reflector(ch_3);
        ch_3 = ALPHABET.chars().nth(index_in_string(reflected_char, &rotor_3)).unwrap();
        ch_2 = ALPHABET.chars().nth(index_in_string(ch_3, &rotor_2)).unwrap();
        ch_1 = ALPHABET.chars().nth(index_in_string(ch_2, &rotor_1)).unwrap();
        ciphertext.push(ch_1);
        status += 1;
        rotor_1 = rotate(&rotor_1);
        if status%26 ==0 {
            rotor_2 = rotate(&rotor_2);
        }
        if status%(26*26) ==0 {
            rotor_3 = rotate(&rotor_3);
        }
    }
    return ciphertext;
}

fn rotate(temp_str: &str) -> String {
    let mut str_vec: Vec<char> = temp_str.chars().collect();
    str_vec.rotate_left(1);
    str_vec.iter().collect()
}

fn enigma_key_generation() -> [String;3] {
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
   // let mut result_char : char = ' ';
    let a = match my_string.find(ch) {
        Some(i) => i,
        None => 27
    };
    let index = usize::try_from(a).unwrap();
    return index;
}

fn get_input(message_to_user: String)-> String{
    println!("{:?}", message_to_user);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read user input");
    return input;
}
