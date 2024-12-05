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
    let lines = load_from_file("input.txt")?;
    let dim: i64 = lines.len() as i64;
    println!("Dim is {}", dim);
    let mut field: Vec<Vec<char>> = vec![];
    for line in lines {
        println!("{}", line);
        field.push(line.chars().collect());
    }
    let mut sum: i64 = 0;
/*    for y in 0..dim { 
        for x in 0..dim-3 {
            if field[y as usize][x as usize] == 'X' && field[y as usize][(x+1) as usize] == 'M'  && field[y as usize][(x+2) as usize] == 'A'  && field[y as usize][(x+3) as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'S' && field[y as usize][(x+1) as usize] == 'A'  && field[y as usize][(x+2) as usize] == 'M'  && field[y as usize][(x+3) as usize] == 'X' {
                sum += 1;
            }
        }
    }
    for y in 0..dim-3 { 
        for x in 0..dim {
            if field[y as usize][x as usize] == 'X' && field[(y+1) as usize][x as usize] == 'M'  && field[(y+2) as usize][x as usize] == 'A'  && field[(y+3) as usize][x as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'S' && field[(y+1) as usize][x as usize] == 'A'  && field[(y+2) as usize][x as usize] == 'M'  && field[(y+3) as usize][x as usize] == 'X' {
                sum += 1;
            }
        }
    }
    for y in 0..dim-3 { 
        for x in 0..dim-3 {
            if field[y as usize][x as usize] == 'X' && field[(y+1) as usize][(x+1) as usize] == 'M'  && field[(y+2) as usize][(x+2) as usize] == 'A'  && field[(y+3) as usize][(x+3) as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'S' && field[(y+1) as usize][(x+1) as usize] == 'A'  && field[(y+2) as usize][(x+2) as usize] == 'M'  && field[(y+3) as usize][(x+3) as usize] == 'X' {
                sum += 1;
            }
        }
    }
    for y in 0..dim-3 { 
        for x in 0..dim-3 {
            if field[y as usize][(x+3) as usize] == 'X' && field[(y+1) as usize][(x+2) as usize] == 'M'  && field[(y+2) as usize][(x+1) as usize] == 'A'  && field[(y+3) as usize][x as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][(x+3) as usize] == 'S' && field[(y+1) as usize][(x+2) as usize] == 'A'  && field[(y+2) as usize][(x+1) as usize] == 'M'  && field[(y+3) as usize][x as usize] == 'X' {
                sum += 1;
            }
       }
    }
    println!("Sum is {}", sum);*/

    for y in 0..dim-2 { 
        for x in 0..dim-2 {
            if field[y as usize][x as usize] == 'M' && field[y as usize][(x+2) as usize] == 'M'  && field[(y+1) as usize][(x+1) as usize] == 'A'  && field[(y+2) as usize][x as usize] == 'S'  && field[(y+2) as usize][(x+2) as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'M' && field[y as usize][(x+2) as usize] == 'S'  && field[(y+1) as usize][(x+1) as usize] == 'A'  && field[(y+2) as usize][x as usize] == 'M'  && field[(y+2) as usize][(x+2) as usize] == 'S' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'S' && field[y as usize][(x+2) as usize] == 'M'  && field[(y+1) as usize][(x+1) as usize] == 'A'  && field[(y+2) as usize][x as usize] == 'S'  && field[(y+2) as usize][(x+2) as usize] == 'M' {
                sum += 1;
            }
            if field[y as usize][x as usize] == 'S' && field[y as usize][(x+2) as usize] == 'S'  && field[(y+1) as usize][(x+1) as usize] == 'A'  && field[(y+2) as usize][x as usize] == 'M'  && field[(y+2) as usize][(x+2) as usize] == 'M' {
                sum += 1;
            }
        }
    }
    println!("Sum is {}", sum);
    Ok(())
}
