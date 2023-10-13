use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::cmp::Ordering;

fn main() {
    let files = vec!["fox.txt", "bustle.txt", "para3.txt"]; // Add your file names here
    let output_file = "2_2.html";

    let mut file_word_counts: Vec<(String, usize)> = Vec::new();

    for file_name in &files {
        if let Ok(file) = File::open(file_name) {
            let reader = BufReader::new(file);
            let word_count = count_words(reader);
            file_word_counts.push((file_name.to_string(), word_count));
        } else {
            eprintln!("Error opening file: {}", file_name);
        }
    }

    file_word_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut wtr = File::create(output_file).expect("Unable to create output file");

    writeln!(wtr, "<style>").expect("Error writing to output file");
    writeln!(wtr, "table, td {{").expect("Error writing to output file");
    writeln!(wtr, "    border: 1px solid #000000;").expect("Error writing to output file");
    writeln!(wtr, "    border-collapse: collapse;}}").expect("Error writing to output file");
    writeln!(wtr, "</style>").expect("Error writing to output file");
    writeln!(wtr, "<table>").expect("Error writing to output file");
    writeln!(wtr, "<tr><th>File</th><th>Words</th></tr>").expect("Error writing to output file");
    for (file_name, word_count) in file_word_counts {
        writeln!(wtr, "<tr><td>{}</td><td>{}</td></tr>", file_name, word_count).expect("Error writing to output file");
    }
    writeln!(wtr, "</table>").expect("Error writing to output file");
}

fn count_words<R: BufRead>(reader: R) -> usize {
    let mut word_count = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let words = line.split_whitespace().collect::<Vec<&str>>();
            word_count += words.len();
        }
    }

    word_count
}
