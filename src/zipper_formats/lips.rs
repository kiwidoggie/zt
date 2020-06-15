use std::fs::File;
use std::io::{self, BufRead};

/// .lips is the facial animation data
pub struct Lips {
    pub string_count: u64,
    pub strings: Vec<String>,

    pub entries: Vec<LipsEntry>
}

pub struct LipsEntry {
    pub entry_index: u64,
    pub entries: Vec<String> // len = string_count
}

impl Lips {
    pub fn from_file(file_path: &String) -> Lips {
        // TODO: Load .lips file
        let file  = match File::open(&file_path) {
            Err(_) => panic!("err: could not open path ({}) for reading.", file_path),
            Ok(f) => f
        };

        // Create a new list of strings
        let mut strings = Vec::new();

        // Get all lines of the input text file
        let lines = match io::BufReader::new(file).lines().collect::<io::Result<Vec<String>>>() {
            Ok(l) => l,
            Err(_) => panic!("")
        };
        
        // Keep track of our current line count
        let mut current_line_count = 0;

        // First we read the count
        let string_count = lines[0].parse::<u64>().unwrap();
        current_line_count += 1;

        // Iterate through each of the strings
        for _string_index in 0..string_count {
            let string = lines.get(current_line_count).unwrap();
            // Add the line to our list
            strings.push(string);

            // Increment the line count
            current_line_count += 1;
        }

        // Skip the ending count, don't know what this is for now
        let entry_count = lines.get(current_line_count).unwrap().parse::<u64>().unwrap();
        current_line_count += 1;

        // Hold all of our entries
        let mut entries = Vec::new();

        // Read out all of the entries
        for _ in 0..entry_count {
            let entry_index = lines.get(current_line_count).unwrap().parse::<u64>().unwrap();
            current_line_count += 1;

            let mut values = Vec::new();
            for _ in 0..string_count {
                values.push(&lines[current_line_count]);
                current_line_count += 1;
            }

            entries.push(LipsEntry {
                entry_index: entry_index,
                entries: Vec::new()
            });
        }

        return Lips {
            string_count: string_count,
            strings: Vec::new(),
            entries: entries
        };
    }
}