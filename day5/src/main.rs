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

#[derive(Clone, Default)]
struct Entry {
    before: Vec<i64>,
    after: Vec<i64>
}

fn main() -> Result<()> {
    let mut lines = load_from_file("input.txt")?;
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut sum: i64 = 0;
    let mut entries = vec![Entry::default(); 100];
    loop {
        println!("{}", lines[0]);
        if let Some(captures) = re.captures(&lines[0]) {
            let first = captures.get(1).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            let second = captures.get(2).map_or("", |m| m.as_str()).parse::<i64>().unwrap();
            entries[first as usize].after.push(second);
            entries[second as usize].before.push(first);
            lines.remove(0);
        } else {
            break;
        }
    }
    for i in 0..entries.len() {
        println!("{}", i);
        println!("Before: ");
        for j in &entries[i].before {
            print!("{} ", j);
        }
        println!("");
        println!("After: ");
        for j in &entries[i].after {
            print!("{} ", j);
        }    
        println!("");    
    }
    lines.remove(0);
    println!("Remaining lines = {}", lines.len());
    /*for line in lines {
        let mut numbers: Vec<i64> = line
            .split(',')
            .filter_map(|s| s.parse::<i64>().ok()) // Try to parse each number and filter out invalid ones
            .collect();
        let mut valid: bool = true;
        for i in 0..numbers.len() {
            let x = numbers[i];
            println!("{}", x);
            for n in i+1..numbers.len() {
                if !entries[x as usize].after.contains(&numbers[n]) {
                    println!("   {} not in after", numbers[n]);
                    valid = false;
                }
            }
        }
        if valid {
            sum += numbers[numbers.len()/2];
        }
        println!("sum is {}", sum);
    }
    println!("Sum is {}", sum);*/

    for line in lines {
        let mut numbers: Vec<i64> = line
            .split(',')
            .filter_map(|s| s.parse::<i64>().ok()) // Try to parse each number and filter out invalid ones
            .collect();
        let mut valid: bool = true;
        loop {
            let mut changes: bool = false;
            for i in 0..numbers.len() {
                let x = numbers[i];
                println!("{}", x);
                for n in i+1..numbers.len() {
                    print!("{} ", n);
                    if !entries[x as usize].after.contains(&numbers[n]) {
                        println!("   {} not in after", numbers[n]);
                        println!("Switch numbers[{}] = {} with numbers[{}] = {}", i, numbers[i], n, numbers[n]);
                        numbers[i] = numbers[n];
                        numbers[n] = x;
                        valid = false;
                        changes = true;
                        break;
                    }
                }
            }
            if !changes {
                break;
            }
        }
        if valid {
            //sum += numbers[numbers.len()/2];
        } else {
            /*println!("Fixed:");
            for n in numbers {
                print!("{} ", n);
            }*/
            println!("");
            sum += numbers[numbers.len()/2];
        }

        println!("sum is {}", sum);
    }
    println!("Sum is {}", sum);
    Ok(())
}
