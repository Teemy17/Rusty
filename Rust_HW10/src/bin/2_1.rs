use std::fs::File;
use std::io::{BufRead, BufReader, Write};


fn main() {
    let files = vec!["fox.txt", "bustle.txt", "para3.txt"]; 
    let output_file = "2_1.html";

    let mut file_paragraph_counts: Vec<(String, usize)> = Vec::new();

    for file_name in &files {
        if let Ok(file) = File::open(file_name) {
            let reader = BufReader::new(file);
            let paragraph_count = count_paragraphs(reader);
            file_paragraph_counts.push((file_name.to_string(), paragraph_count));
        } else {
            eprintln!("Error opening file: {}", file_name);
        }
    }

    file_paragraph_counts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut wtr = File::create(output_file).expect("Unable to create output file");

    writeln!(wtr, "<style>").expect("Error writing to output file");
    writeln!(wtr, "table, td {{").expect("Error writing to output file");
    writeln!(wtr, "    border: 1px solid #000000;").expect("Error writing to output file");
    writeln!(wtr, "    border-collapse: collapse;}}").expect("Error writing to output file");
    writeln!(wtr, "</style>").expect("Error writing to output file");
    writeln!(wtr, "<table>").expect("Error writing to output file");
    writeln!(wtr, "<tr><th>File</th><th>Paragraphs</th></tr>").expect("Error writing to output file");
    for (file_name, paragraph_count) in file_paragraph_counts {
        writeln!(wtr, "<tr><td>{}</td><td>{}</td></tr>", file_name, paragraph_count).expect("Error writing to output file");
    }
    writeln!(wtr, "</table>").expect("Error writing to output file");
}

fn count_paragraphs<R: BufRead>(reader: R) -> usize {
    let mut paragraph_count = 0;
    let mut in_paragraph = false;

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.trim().is_empty() {
                if in_paragraph {
                    paragraph_count += 1;
                }
                in_paragraph = false;
            } else {
                in_paragraph = true;
            }
        }
    }

    if in_paragraph {
        paragraph_count += 1;
    }

    paragraph_count
}
