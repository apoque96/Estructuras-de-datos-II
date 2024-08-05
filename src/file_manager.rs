use crate::structs::data::Data;
use encoding_rs::UTF_8;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    fs::File,
    io::{self, prelude::*, stdin, BufReader},
    path::Path,
};

// Reads the file line by line
fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(UTF_8))
        .build(file);
    let buf = BufReader::new(transcoded);
    buf.lines().collect()
}

pub fn read_file(message: &str) -> Result<Vec<Data>, io::Error> {
    println!("{message}");
    let mut path = String::new();

    if let Err(e) = stdin().read_line(&mut path) {
        return Err(e);
    }

    match lines_from_file(path.trim()) {
        Ok(lines) => {
            return file_to_data(lines);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

fn format_lines(lines: Vec<String>) -> Vec<String> {
    if lines.len() > 1 {
        return lines;
    }
    let ans: Vec<String> = lines[0].split('\r').map(|l| String::from(l)).collect();
    ans
}

fn file_to_data(lines: Vec<String>) -> Result<Vec<Data>, io::Error> {
    let mut datas = Vec::new();

    let lines = format_lines(lines);

    for line in lines {
        let parts: Vec<&str> = line.split(';').collect();

        if parts.len() == 2 {
            let method = parts[0].to_string();
            let data = parts[1].to_string();
            datas.push(Data::new(method, data));
        }
    }

    Ok(datas)
}
