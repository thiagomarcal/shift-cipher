use std::collections::HashMap;
use std::env;

enum Operation {
    Encode,
    Decode,
}

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let operation_type = args.iter().nth(1).expect("No operation type provided").clone();

    let caesar_code_arg = extract_caesar_code_from_args(&args).expect("No Caesar code provided");
    let word = extract_word_from_args(args).expect("No word provided");

    let caesar_code_arg = caesar_code_arg
        .trim()
        .parse::<u8>()
        .expect("Failed to parse Caesar code");

    let operation = check_operation(operation_type).expect("Invalid operation type");

    process_operation(&word, caesar_code_arg, operation);
}

fn extract_word_from_args(args: Vec<String>) -> Option<String> {
    let mut word = String::new();

    for i in 0..args.len() {
        if args[i] == "--word" && i + 1 < args.len() {
            if args[i + 1].is_empty() {
                return None;
            }
            word = args[i + 1].clone();
            break;
        }
    }
    Some(word)
}

fn extract_caesar_code_from_args(args: &Vec<String>) -> Option<String> {
    let mut caesar_code_arg = String::new();
    for i in 0..args.len() {
        if args[i] == "--caesar_code" && i + 1 < args.len() {
            if args[i + 1].is_empty() {
                return None;
            }
            caesar_code_arg = args[i + 1].clone();
            break;
        }
    }
    Some(caesar_code_arg)
}

fn process_operation(word: &String, caesar_code_arg: u8, operation: Operation) -> Option<String> {
    match operation {
        Operation::Encode => {
            let result = encode_word(word, caesar_code_arg);
            println!("Caesar code:   {}", caesar_code_arg);
            println!("Word:          {}", word);
            println!("Encoded Word:  {}", result);
            Some(result)
        }
        Operation::Decode => {
            let result = decode_word(word, caesar_code_arg);
            println!("Caesar code:   {}", caesar_code_arg);
            println!("Word:          {}", word);
            println!("Decoded Word:  {}", result);
            Some(result)
        }
    }
}

fn check_operation(operation_type: String) -> Option<Operation> {
    match operation_type {
        ref s if s == "encode" => {
            Some(Operation::Encode)
        }
        ref s if s == "decode" => {
            Some(Operation::Decode)
        }
        _ => {
            None
        }
    }
}

fn ciphered_core(word: &String, caesar_code_arg: u8, cipher_func: &dyn Fn(u8, u8) -> u8) -> String {
    let alphabet_hashmap: HashMap<char, u8> = ALPHABET.chars().enumerate().map(|(i, c)| (c, i as u8)).collect();

    let mut new_ciphered_word = String::new();

    for idx in 0..word.len() {
        let c = word.chars().nth(idx)
            .expect("Error extracting char from word")
            .to_uppercase().next()
            .expect("Error converting char to uppercase");

        let word_index = alphabet_hashmap.get(&c)
            .expect("Error getting index from hashmap");

        let cipher_char = ALPHABET.chars().nth(cipher_func(*word_index, caesar_code_arg) as usize)
            .expect("Error getting ciphered char from alphabet");

        new_ciphered_word.push(cipher_char);
    }
    new_ciphered_word
}

fn encode_word(word: &String, caesar_code_arg: u8) -> String {
    ciphered_core(&word, caesar_code_arg, &|word_index, caesar_code_arg| {
        (word_index + caesar_code_arg) % ALPHABET.len() as u8
    })
}

fn decode_word(word: &String, caesar_code_arg: u8) -> String {
    ciphered_core(word, caesar_code_arg, &|word_index, caesar_code_arg| {
        let alphabet_len = ALPHABET.len() as u8;
        (word_index + alphabet_len - caesar_code_arg) % alphabet_len
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ciphered_word() {
        let word = String::from("HELLO");
        let caesar_code_arg = 3;
        let result = encode_word(&word, caesar_code_arg);
        assert_eq!(result, "KHOOR");
    }

    #[test]
    fn test_generate_ciphered_word_2() {
        let word = String::from("thiago");
        let caesar_code_arg = 3;
        let result = encode_word(&word, caesar_code_arg);
        assert_eq!(result, "WKLDJR");
    }

    #[test]
    fn test_generate_ciphered_word_with_wraparound() {
        let word = String::from("XYZ");
        let caesar_code_arg = 3;
        let result = encode_word(&word, caesar_code_arg);
        assert_eq!(result, "ABC");
    }

    #[test]
    fn test_generate_ciphered_word_with_lowercase() {
        let word = String::from("hello");
        let caesar_code_arg = 3;
        let result = encode_word(&word, caesar_code_arg);
        assert_eq!(result, "KHOOR");
    }

    #[test]
    fn test_generate_un_ciphered_word() {
        let word = String::from("KHOOR");
        let caesar_code_arg = 3;
        let result = decode_word(&word, caesar_code_arg);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_generate_un_ciphered_word_2() {
        let word = String::from("WKLDJR");
        let caesar_code_arg = 3;
        let result = decode_word(&word, caesar_code_arg);
        assert_eq!(result, "THIAGO");
    }
}