use serde::{Serialize, Deserialize};
use std::fs::File;
// use csv::ReaderBuilder;
use bincode;
use std::io::{self, Write, BufRead, BufReader};
use regex::Regex;

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    key: String,
    value: u32,
}

// fn read_csv_and_create_entries(file_path: &str) -> io::Result<Vec<Entry>> {
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(false)
//         .from_path(file_path)?;

//     let mut entries = Vec::with_capacity(200_000);  // Pre-allocate memory for 200,000 entries
//     for (i, result) in rdr.records().enumerate() {
//         let record = result?;
//         if let Some(field) = record.get(0) {
//             entries.push(Entry {
//                 key: field.to_string(),
//                 value: i as i32,
//             });
//         }
//     }
//     Ok(entries)
// }

fn read_txt_and_create_entries(file_path: &str) -> io::Result<Vec<Entry>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::with_capacity(200_000);  // Pre-allocate memory for 200,000 entries

    let re = Regex::new(r"[0-9]+").unwrap();  // Regular expression to match numbers

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        let cleaned_line = re.replace_all(&line, "");  // Remove numbers
        entries.push(Entry {
            key: cleaned_line.trim().to_string(),  // Trim any surrounding whitespace
            value: i as u32,  // Use u32 to store the line index
        });
    }
    Ok(entries)
}


fn write_entries_to_file(entries: &Vec<Entry>, file_path: &str) -> io::Result<()> {
    let encoded: Vec<u8> = bincode::serialize(entries).expect("Failed to serialize data");
    let mut file = File::create(file_path)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let csv_file_path = "blcu_list.txt";
    let bin_file_path = "data.profile";

    // Read entries from TXT file
    let entries = read_txt_and_create_entries(csv_file_path)?;

    // Serialize entries to binary file
    write_entries_to_file(&entries, bin_file_path)?;

    println!("Data successfully serialized to {}", bin_file_path);
    Ok(())
}
