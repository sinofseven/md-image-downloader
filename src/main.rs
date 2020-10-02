use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

fn main() {
    if let Err(msg) = process() {
        eprintln!("{}", msg);
        std::process::exit(1);
    }
}

fn process() -> Result<(), String> {
    let path = get_file_path()?;
    let urls = get_urls(&path)?;

    download(urls)?;

    Ok(())
}

fn get_file_path() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();
    
    args.get(1).ok_or(format!("not exists file name")).map(|s| s.clone())
}

fn get_urls(path: &String) -> Result<Vec<String>, String> {
    let mut result = Vec::new();
    let re = Regex::new(r#"^!\[.*\]\(.*\)"#).unwrap();

    let file = File::open(path).map_err(|e| format!("failed to open file: {}", e))?;
    
    for line in BufReader::new(file).lines() {
        if let Ok(s) = line {
            if !re.is_match(&s) {
                continue;
            }
            let part: Vec<&str> = s.split("](").collect();
            let s = part[1];
            let part: Vec<&str> = s.split(")").collect();
            let s = part[0];
            result.push(s.to_string());
        }
    }
    Ok(result)
}

fn download(v: Vec<String>) -> Result<(), String> {
    for (i, url) in v.iter().enumerate() {
        let part: Vec<&str> = url.split("/").collect();
        let name = part.last().unwrap().to_string();
        let resp = reqwest::blocking::get(url).map_err(|e| format!("failed to get image: {}", e))?;

        let a = resp.bytes().map_err(|e| format!("failed to parse byte: {}", e))?;
        std::fs::write(format!("[{:03}] {}", i, name), a).map_err(|e| format!("failed to write image: {}", e))?;
    }
    Ok(())
}