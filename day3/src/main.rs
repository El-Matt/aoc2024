use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;
use regex::Regex;

fn load_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    reader.lines().collect()
}
fn main() -> Result<()> {
    let lines = load_from_file("input.txt")?;
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("Invalid regex");
    let mut sum: i64 = 0;
    //for line in lines {
    /*let line: String = lines.concat();
    for found in re.find_iter(&line) {
        println!("Found match: {}", found.as_str());
        let parts: Vec<&str> = found.as_str().split(|c| ['(',',',')'].contains(&c)).collect();
        sum += parts[1].parse::<i64>().unwrap() * parts[2].parse::<i64>().unwrap();
        println!("sum: {}", sum);
    }*/
    //}

    let mut line: String = lines.concat();
    println!("{}", line);
    let mut line_filtered: String = Default::default();
    
    loop {
        if let Some(index) = line.find("don't()") {
            println!("Found at index {}", index);
            let foo: String = line.chars().take(index).collect();
            line_filtered.push_str(&foo);
            line = line.chars().skip(index).collect();
        } else {
            println!("not found");
            line_filtered.push_str(&line);
            break;
        }
        if let Some(index) = line.find("do()") {
            println!("Found at index {}", index);
            let foo: String = line.chars().take(index).collect();
            line = line.chars().skip(index).collect();
        } else {
            println!("not found");
            break;
        }
    }

    for found in re.find_iter(&line_filtered) {
        println!("Found match: {}", found.as_str());
        let parts: Vec<&str> = found.as_str().split(|c| ['(',',',')'].contains(&c)).collect();
        sum += parts[1].parse::<i64>().unwrap() * parts[2].parse::<i64>().unwrap();
        println!("sum: {}", sum);
    }

    Ok(())
}
