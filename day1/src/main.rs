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
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    let lines = load_from_file("input.txt")?;
    for line in lines {
        //println!("{}", line);
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse().unwrap());
        right.push(parts[1].parse().unwrap());
    }
    let mut sum: i64 = 0;
    /*while !left.is_empty() {
        let a: i64 = match left.iter().min() {
            Some(y) => *y,
            None => panic!()
        };
        let b: i64 = match right.iter().min() {
            Some(y) => *y,
            None => panic!()
        };
        sum = sum + (a-b).abs();
        println!("{} {} {}", a, b, sum);
        if let Some(index) = left.iter().position(|&x| x == a) {
            left.remove(index);
        }
        if let Some(index) = right.iter().position(|&x| x == b) {
            right.remove(index);
        }
    }*/
    while !left.is_empty() {
        let x: i64 = match left.pop() {
            Some(y) => y,
            None => panic!()
        };
        let count = right.iter().filter(|&&a| a == x).count() as i64;
        sum = sum + (count*x);
        println!("{}", sum);
    }

    Ok(())
}
