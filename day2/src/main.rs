use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;

fn load_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() -> Result<()> {
    /*let lines = load_from_file("input.txt")?;
    let mut safe_entries: u64 = 0;
    for line in lines {
        println!("{}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        let ascending: bool;
        if parts[0].parse::<i64>().unwrap() < parts[1].parse::<i64>().unwrap() {
            ascending = true;
        } else {
            ascending = false;
        }
        let mut safe: bool = true;
        println!("parts.len() = {}", parts.len());
        for i in 0..parts.len()-1 {
            print!("{} {} ", i, parts[i].parse::<i64>().unwrap());
            if (parts[i].parse::<i64>().unwrap() < parts[i+1].parse::<i64>().unwrap()) && ascending == false {
                safe = false;
            }
            if (parts[i].parse::<i64>().unwrap() > parts[i+1].parse::<i64>().unwrap()) && ascending == true {
                safe = false;
            }      
            if parts[i].parse::<i64>().unwrap() == parts[i+1].parse::<i64>().unwrap() {
                safe = false;
            }
            if (parts[i].parse::<i64>().unwrap() - parts[i+1].parse::<i64>().unwrap()).abs() > 3 {
                safe = false;
            }
        }
        if safe {
            safe_entries += 1;
        }
        println!("{}",safe);
    }
    println!("safe_entries {}", safe_entries);*/
 
    let lines = load_from_file("input.txt")?;
    let mut safe_entries: u64 = 0;
    for line in lines {
        println!("{}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut entry_safe: bool = false;
        for j in 0..parts.len() {
            let mut copy = parts.clone();
            copy.remove(j);
            for i in &copy {
                print!(" {}", i);
            }
            println!("");
            let ascending: bool;
            if copy[0].parse::<i64>().unwrap() < copy[1].parse::<i64>().unwrap() {
                ascending = true;
            } else {
                ascending = false;
            }
            let mut safe: bool = true;
            for i in 0..copy.len()-1 {
                //print!("{} ", copy[i].parse::<i64>().unwrap());
                if (copy[i].parse::<i64>().unwrap() < copy[i+1].parse::<i64>().unwrap()) && ascending == false {
                    safe = false;
                }
                if (copy[i].parse::<i64>().unwrap() > copy[i+1].parse::<i64>().unwrap()) && ascending == true {
                    safe = false;
                }      
                if copy[i].parse::<i64>().unwrap() == copy[i+1].parse::<i64>().unwrap() {
                    safe = false;
                }
                if (copy[i].parse::<i64>().unwrap() - copy[i+1].parse::<i64>().unwrap()).abs() > 3 {
                    safe = false;
                }
            }
            if safe {
                entry_safe = true;
            }
            println!("{}",safe);
        }
        if entry_safe {
            safe_entries += 1;
        }
        println!("safe_entries {}", safe_entries);
    }
    Ok(())
}
