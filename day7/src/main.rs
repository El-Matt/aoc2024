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
/*  let mut lines = load_from_file("input.txt")?;
    let mut sum: i64 = 0;
    for line in lines {
        let substrings: Vec<&str> = line.split(|c: char| c.is_whitespace()).collect();
        let ops_len = substrings.len() - 1;
        for i in 0..substrings.len() {
            //print!("{} ", substrings[i]);
        }
        println!("");
        println!("ops_len {}", ops_len);
        for i in 0..(1<<(ops_len-1)) {
            //println!("i {}", i);
            let mut res_local: i64 = substrings[1].parse::<i64>().unwrap();
            for j in 2..ops_len+1 {
                //println!("{} {} {}", i, j, substrings[j]);
                if (i & (1<<(j-2))) == 0 {
                    //println!("Add {}", substrings[j].parse::<i64>().unwrap());
                    res_local += substrings[j].parse::<i64>().unwrap();
                } else {
                    //println!("Mul {}", substrings[j].parse::<i64>().unwrap());
                    res_local *= substrings[j].parse::<i64>().unwrap();                   
                }
            }
            //println!("res_local {}", res_local);
            if res_local == substrings[0][..substrings[0].len()-1].parse::<i64>().unwrap() {
                //println!("SUCCESS");
                sum += res_local;
                break;
            }
        }
    }
    println!("sum {}", sum);*/
    //1430271835320


    /*
    3 5 7
    /3
    %3
    0 -> 00
    1 -> 01
    2 -> 02
    3 -> 10
    ...

    3 5 7 9
    %27/9
    %9/3        %3^2 / 3^1
    %3/1        %3^1 /3^0
    0 -> 000
    1 -> 001
    2 -> 002
    3 -> 010
    4 -> 011
    5 -> 012
    6 -> 020
    7 -> 021
    8 -> 022
    9 -> 100
    10-> 101
    
     */

    let mut lines = load_from_file("input.txt")?;
    let mut sum: i64 = 0;
    for line in lines {
        let substrings: Vec<&str> = line.split(|c: char| c.is_whitespace()).collect();
        let ops_len = substrings.len() - 1;
        for i in 0..substrings.len() {
            //print!("{} ", substrings[i]);
        }
        //println!("");
        //println!("ops_len {}", ops_len);
        for i in 0..((3 as i64).pow((ops_len as u32)-1)) {
            //println!("i {}", i);
            let mut res_local: i64 = substrings[1].parse::<i64>().unwrap();
            for j in 2..ops_len+1 {
                //println!("{} {} {}", i, j, substrings[j]);
                //if (i & (1<<(j-2))) == 0 {    
                let op = i % (3 as i64).pow(j as u32 -2 +1) / (3 as i64).pow(j as u32 -2);       
                if op == 0 {
                    //println!("Add {}", substrings[j].parse::<i64>().unwrap());
                    res_local += substrings[j].parse::<i64>().unwrap();
                } else if op == 1 {
                    //println!("Mul {}", substrings[j].parse::<i64>().unwrap());
                    res_local *= substrings[j].parse::<i64>().unwrap();                   
                } else {
                    //println!("Con {}", substrings[j].parse::<i64>().unwrap());
                    res_local = (res_local.to_string() + substrings[j]).parse::<i64>().unwrap();
                }
            }
            //println!("res_local {}", res_local);
            if res_local == substrings[0][..substrings[0].len()-1].parse::<i64>().unwrap() {
                //println!("SUCCESS");
                sum += res_local;
                break;
            }
        }
    }
    println!("sum {}", sum);
    // 456565678667482




    Ok(())
}