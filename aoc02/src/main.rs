use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let text = read_file().unwrap();
    println!("{}", challenge_one(&text).unwrap());
    println!("{}", challenge_two(&text).unwrap());
}

fn challenge_one(text: &String) -> std::io::Result<usize> {
    let mut similar_boxes: HashMap<usize, usize> = HashMap::with_capacity(2);
    similar_boxes.insert(2, 0);
    similar_boxes.insert(3, 0);

    let mut chars_dict: HashMap<char, usize> = HashMap::with_capacity(26);
    for word in text.lines() {
        for c in word.chars() {
            if chars_dict.contains_key(&c) {
                chars_dict.insert(c, chars_dict.get(&c).unwrap() + 1);
            } else {
                chars_dict.insert(c, 1);
            }
        }
        let mut added_to_two = false;
        let mut added_to_three = false;
        for val in chars_dict.values() {
            if val == &2 && !added_to_two {
                similar_boxes.insert(2, similar_boxes.get(&2).unwrap() + &1);
                added_to_two = true;
            } else if val == &3 && !added_to_three {
                similar_boxes.insert(3, similar_boxes.get(&3).unwrap() + &1);
                added_to_three = true;
            }
            if added_to_two && added_to_three {
                break
            }
        }
        chars_dict.clear();
    }
    Ok(similar_boxes.get(&2).unwrap() * similar_boxes.get(&3).unwrap())
}

fn challenge_two(text: &String) -> std::io::Result<String>  {
    let mut seen_words: HashSet<&str> = HashSet::new();
    for word in text.lines() {
        let mut found_match = false;
        let mut error_index = 0;
        for seen_word in &seen_words {
            let mut seen_first_error = false;
            let mut seen_word_chars = seen_word.chars();
            let mut seen_word_c = seen_word_chars.next().unwrap();
            for (i, c) in word.char_indices() {
                if c != seen_word_c && !seen_first_error {
                    error_index = i;
                    seen_first_error = true
                } else if c != seen_word_c && seen_first_error {
                    break
                }
                seen_word_c = match seen_word_chars.next() {
                    Some(c) => c,
                    None => {
                        found_match = true;
                        break
                    }
                };
            }
            if found_match {
                let mut final_str = String::with_capacity(25);
                for (i, c) in word.char_indices() {
                    if i != error_index {
                        final_str.push(c);
                    }
                }
                return Ok(final_str);
            }
        }
        seen_words.insert(word);
    }
    Ok(String::new())
}

fn read_file() -> std::io::Result<String> {
    let mut file = File::open("input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}