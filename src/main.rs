use std::{fs, path::Path, process::exit};

const ALPHABET:&str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ";

const TARGET: &str = "a1525fd30074da00e777a55e07107910";
const GUESS: &str = "FIND US ALONE";

const NUM_WORDS: usize = 3;
const WORDS:[&str; NUM_WORDS] = ["FIND", "US", "ALONE"];

fn main() {
    let mut guess: String = GUESS.to_string();
    mess_with_chars(&mut guess, 2, &check_correct);

    let dict = read_lines_uppercase("words.txt");
    substitute_words(&dict, 2, WORDS);
}

fn check_correct(input: &String) {
    let hashed = format!("{:?}", md5::compute(input));
    if TARGET == hashed.as_str() {
        println!("{input}");
        exit(0);
    }
}

fn mess_with_chars(input: &mut String, depth:u8, callback: &dyn Fn(&String)) {
    for i in 0..input.chars().count() {
        let initial = input.chars().nth(i).unwrap();
        for char in ALPHABET.chars() {
            input.replace_range(i..i+1, &char.to_string());
            if depth - 1 == 0 {
                callback(&input)
            } else {
                mess_with_chars(input, depth-1, callback);
            }
        }
        input.replace_range(i..i+1, "");
        if depth - 1 == 0 {
            callback(&input)
        } else {
            mess_with_chars(input, depth-1, callback);
        }

        // undo changes
        input.insert(i, initial)

    }
}

fn read_lines_uppercase<P: AsRef<Path>>(path:P) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    contents.lines().filter(|s| !s.contains("#!")).map(|str| str.to_uppercase()).collect()
}

fn substitute_words(dictionary: &Vec<String>, depth:u8, input: [&str; NUM_WORDS]) {
    for i in 0..NUM_WORDS {
        let mut temp = input.clone();
        for word in dictionary {
            temp[i] = word.as_str();
            if depth - 1 == 0 {
                check_words(&temp);
            } else {
                substitute_words(dictionary, depth - 1, temp);
            }
        }
    }
}

fn check_words(input: &[&str; NUM_WORDS]) {
    let pattern = input.join(" ");
    check_correct(&pattern);
}