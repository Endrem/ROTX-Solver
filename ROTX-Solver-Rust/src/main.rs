use std::vec::Vec;
use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::collections::HashMap;

const MIN_LETTERS: u8 = 2;
const MAX_LETTERS: u8 = 6;

// The dictionary list HashMap will look something like the following:
// {
//  2: {"AT": [19], "BE": [3]...},
//  3: {"AND": [13,16], "THE": [14,23]...},
//  ...
//}

// Loads our dictionary file into memory, adding the distances for each word that we need.
fn load_dictionary(path: String, list: &mut HashMap<u8, HashMap<String, Vec<u8>>>) {
    // Create a HashMap for each number of letters (Ex: 2..6).
    for n in MIN_LETTERS..=MAX_LETTERS {
        list.insert(n, HashMap::new());
    }

    let dictionary_file = File::open(path).unwrap();
    let dictionary_contents = BufReader::new(dictionary_file);
    
    for line in dictionary_contents.lines() {
        let dictionary_word: String = line.unwrap();
        let dict_word_len = dictionary_word.len();

        // Only add dictionary words that are within our MIN_LETTERS and MAX_LETTERS range.
        if (dict_word_len >= MIN_LETTERS.into()) && (dict_word_len <= MAX_LETTERS.into())  {
            let distance_vector: Vec<u8> = get_distance(&dictionary_word);
            // Get the hashtable to the corresponding word length (ex: 3 for "AND").
            // Then insert the word and the distance vector into that hashtable.
            list.get_mut(&(dict_word_len as u8)).unwrap().insert(dictionary_word, distance_vector);
            
        }

    }
}

// Calculates the 'distances' of the (alphabetic) characters and returns a vector of them.
fn get_distance(word: &String) -> Vec<u8> {
    let word = word.to_lowercase();
    let mut num: u8;
    let mut vec = Vec::new();

    for i in 0..=(word.len()-2) {
        let letter1: u8 = word.as_bytes()[i];
        let letter2: u8 = word.as_bytes()[i+1];

        if letter1 <= letter2 {
            num = letter2 - letter1;
        } else {
            num = (123 - letter1) + (letter2 - 97);
        }

        vec.push(num);
    }
    return vec;
}

// Use a ciphered sentence to determine what ROT hits the most.  Values are added to a referenced vector.
fn test_sentence(rot_phrase: &String, list: &mut HashMap<u8, HashMap<String, Vec<u8>>>, hits: &mut Vec<u16>) {
    let mut phrase_distance = Vec::new();
    let mut phrase_ciphered = Vec::new();

    for rot_word in rot_phrase.split(" ") {
        // Only grab alphabetic letters (to filter out punctuation).
        let rot_word: String = rot_word.chars().filter(|c| c.is_ascii_alphabetic()).collect();

        // Skip over words outside of our min and max.
        if (rot_word.len() >= MIN_LETTERS.into()) && (rot_word.len() <= MAX_LETTERS.into())  {
            // Get distance of rot_word and push it onto phrase_distance.
            phrase_distance.push(get_distance(&rot_word));
            phrase_ciphered.push(rot_word);
        }
    }

    let phrase_distance_word_count = phrase_distance.len();

    // Go through all of the ciphered words within the sentence (that were turned into 'distance' vectors).
    for i in 0..=(phrase_distance_word_count-1) {
        // Get the letter length of the word we're currently working on.
        // Add one for the full length (since distance checks *between* letters, we lose 1 length).
        let letter_count: usize = phrase_distance[i].len()+1;
        // Retrieve the HashMap of the letter length that we are currently working on.
        for dictionary_length_hashmap in list.get(&(letter_count as u8)) {
            for dictionary_distance in dictionary_length_hashmap {
                // Compares distance vector from dictionary vectors.
                if &phrase_distance[i] == dictionary_distance.1 {
                    // Combine the first letters of both words so the ASCII difference can be calculated.
                    let mut distance_sample = String::new();
                    distance_sample.push(phrase_ciphered[i].chars().nth(0).unwrap());
                    distance_sample.push(dictionary_distance.0.chars().nth(0).unwrap());

                    let rot: Vec<u8> = get_distance(&distance_sample);
                    // Add a 'hit' to the rot number that we got recieved from the get_distance function.
                    hits[(rot[0] as usize)] += 1;
                }
            }
        }
    }
}

// This function returns a deciphered string when supplied a ciphered string and a rot.
fn rot_decipher(text: &String, rot: u8) -> String {
    let mut deciphered_text: String = String::from(text);
    for i in 0..=(deciphered_text.len()-1){
        let ord: u8 = deciphered_text.as_bytes()[i];

        if (65 <= ord) && (ord <= 90) {
            if (ord + rot) > 90 {
                deciphered_text.replace_range(i..(i+1), &((ord + rot - 26) as char).to_string());
            } else {
                deciphered_text.replace_range(i..(i+1), &((ord + rot) as char).to_string());
            }
        } else if (97 <= ord) && (ord <= 122){
            if (ord + rot) > 122 {
                deciphered_text.replace_range(i..(i+1), &((ord + rot - 26) as char).to_string());
            } else {
                deciphered_text.replace_range(i..(i+1), &((ord + rot) as char).to_string());
            }
        }
    }
    return deciphered_text;
}

// Main function.
fn main() -> Result<(), Error>  {
    // Setup initial rot_hits vector and fill it with 0s.
    let mut rot_hits: Vec<u16> = Vec::with_capacity(26);
    for _i in 0..26 {
        rot_hits.push(0);
    }

    // Our primary dictionary HashMap.
    let mut dictionary_list = HashMap::new();
    load_dictionary(("../Dictionary.txt").to_string(), &mut dictionary_list);

    // Open file containing ROT-ciphered sentences.
    let cipher_file_in = File::open("../ROTSentences.txt").unwrap();
    let cipher_file_contents = BufReader::new(cipher_file_in);

    // Create file to output the plaintext sentences.
    let mut plaintext_file_out = File::create("Plaintext.txt").unwrap();
    
    for cipher_line in cipher_file_contents.lines() {
        // Zero out rot_hits for the next sentence.
        for i in 0..26 {
            rot_hits[i] = 0;
        }

        let cipher_text = cipher_line.unwrap();
        // Pass the ciphered sentence, a mutable reference to our dictionary HashMap;
        // and a mutable reference to rot_hits vector to track which ROT matches up the most.
        test_sentence(&cipher_text, &mut dictionary_list, &mut rot_hits);

        // Get the ROT by finding the max value in rot_hits and then finding the position/index that it's in.
        let rot_max = rot_hits.iter().max().unwrap();
        let rot = rot_hits.iter().position(|r| r == rot_max).unwrap();

        // Decipher the sentence using the rot found above.
        let deciphered_line: String = rot_decipher(&cipher_text, rot as u8);
        // Write the plaintext sentence to our output file.
        writeln!(&mut plaintext_file_out, "{}", deciphered_line)?;
    }
    Ok(())
}
