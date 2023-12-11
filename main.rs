use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut sum: u16 = 0;

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let num_str = parse_line(line);

                match num_str.parse::<u8>() {
                    Ok(num) => {
                        sum += num as u16;
                    }
                    Err(_) => {
                        println!("Failed to parse '{}' as an integer", num_str);
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    println!("{}", sum);
}

fn parse_line(line: String) -> String {
    let mut vec: Vec<char> = Vec::new();

    for c in line.chars() {
        if c.is_numeric() {
            vec.push(c);
        }
    }

    let mut num: String = String::new();

    if !vec.is_empty() {
        let last_index = vec.len() - 1;
        let first_element = vec[0];
        let last_element = vec[last_index];

        num.push(first_element);
        num.push(last_element);
    }

    return num;
}
