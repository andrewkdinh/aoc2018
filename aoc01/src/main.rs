use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let contents = read_file("input");
    let mut sum = 0;
    let mut frequencies_seen = HashSet::<isize>::new();

    loop {
        for str_num in contents.lines() {
            sum += str_num.parse::<isize>().unwrap();
            if frequencies_seen.contains(&sum) {
                println!("{}", sum);
                return
            }
            frequencies_seen.insert(sum);
        }
        // break
    }
    // println!("{}", sum)
}

fn read_file(file_name: &str) -> String {
    let mut file: File = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}