use std::{fs, path::Path, process::exit};

const ALPHABET:&str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let target = "a1525fd30074da00e777a55e07107910";
    let guess = "FIND US ALIVE";
    
    let callback = |to_test: &str| check_correct(to_test, target);

    mess_with_chars(&mut guess.to_string(), 2, &callback);

    let dict = read_lines_uppercase("words.txt");
    let mut words = guess.split_ascii_whitespace().collect();
    substitute_words(&dict, 1, &mut words, &callback);
}

fn check_correct(to_test: &str, target: &str) {
    let hashed = format!("{:?}", md5::compute(to_test));
    if target == hashed.as_str() {
        println!("{to_test}");
        exit(0);
    }
}

fn mess_with_chars(input: &mut String, depth:u8, callback: &dyn Fn(&str)) {
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

fn substitute_words<'a>(dictionary: &'a Vec<String>, depth:u8, words: &mut Vec<&'a str>, callback: &dyn Fn(&str)) {
    for i in 0..words.len() {
        let initial = words[i];
        for word in dictionary {
            words[i] = word.as_str();
            if depth - 1 == 0 {
                callback(&words.join(" "));
            } else {
                substitute_words(dictionary, depth - 1, words, callback);
            }
        }
        // undo changes
        words[i] = initial;
    }
}