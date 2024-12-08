use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Result;

#[derive(PartialEq)]
struct Coord {
    y: usize,
    x: usize
}

impl Coord {
    // Constructor for Coord
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

fn load_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn print_map(m: &Vec<Vec<char>>, d: i64) {
    for x in 0..d {
        for y in 0..d {
            print!("{}", m[x as usize][y as usize]);
        }
        println!("");
    }
    println!("");
    println!("");
}

fn main() -> Result<()> {
    let lines = load_from_file("input.txt")?;
    let dim: i64 = lines.len() as i64;
    println!("Dim is {}", dim);
    let mut field: Vec<Vec<char>> = vec![];
    for line in lines {
        //println!("{}", line);
        field.push(line.chars().collect());
    }
    let mut x: usize = 0;
    let mut y: usize = 0;
    for yi in 0..dim {
        for xi in 0..dim {
            if field[yi as usize][xi as usize] == '^' {
                y = yi as usize;
                x = xi as usize;
            }
        }
    } 


  /*let mut dir: i64 = 0;   // 0 ^, 1 >, 2 v, 3 <
    loop {
        //println!("Coords are {},{}", y, x);
        //print_map(&field, dim );
        if dir == 0 {
            if y == 0 {
                field[y][x] = 'X';
                break;
            }
            if field[y-1][x] == '.'  || field[y-1][x] == 'X'{
                field[y][x] = 'X';
                y -= 1;
            } else {
                dir = 1;
            }
        } else if dir == 1 {
            if x == (dim-1).try_into().unwrap() {
                field[y][x] = 'X';
                break;
            }
            if field[y][x+1] == '.'  || field[y][x+1] == 'X'{
                field[y][x] = 'X';
                x += 1;
            } else {
                dir = 2;
            }
            continue;
        } else if dir == 2 {
            if y == (dim-1).try_into().unwrap() {
                field[y][x] = 'X';
                break;
            }
            if field[y+1][x] == '.' || field[y+1][x] == 'X'  {
                field[y][x] = 'X';
                y += 1;
            } else {
                dir = 3;
            }
            continue;
        } else if dir == 3 {
            if x == 0 {
                field[y][x] = 'X';
                break;
            }
            if field[y][x-1] == '.'  || field[y][x-1] == 'X'{
                field[y][x] = 'X';
                x -= 1;
            } else {
                dir = 0;
            }
            continue;
        }
    }
    print_map(&field, dim );
    let mut sum: i64 = 0;
    for yi in 0..dim {
        for xi in 0..dim {
            if field[yi as usize][xi as usize] == 'X' {
                sum += 1;
            }
        }
    }
    println!("sum is {}", sum);*/

  let mut obst: Vec<Coord> = vec![];
    let mut dir: i64 = 0;   // 0 ^, 1 >, 2 v, 3 <
    let mut field_dirs: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; 4]; dim as usize]; dim as usize];
    for yi in 0..dim {
        for xi in 0..dim {
            for diri in 0..4 {
                field_dirs[yi as usize][xi as usize][diri] = 0;
            }
        }
    }
    loop {
        //println!("Coords are {},{}", y, x);
        //print_map(&field, dim );
        field_dirs[y][x][dir as usize] = 1;
        if dir == 0 {
            if y == 0 {
                field[y][x] = 'X';
                break;
            }
            let mut x_a: usize = x;
            let mut y_a: usize = y;
            let mut o_r_found: bool = false;
            loop {
                if field[y_a][x_a] == '#' {
                    break;
                }
                if field_dirs[y_a][x_a][((dir+1)%4) as usize] == 1 {
                    o_r_found = true;
                    break;
                }
                if x_a == (dim-1) as usize {
                    break;
                }
                x_a += 1;
            }
            if o_r_found {
                let o = Coord::new(y-1, x);
                if !obst.contains(&o) {
                    println!("New obstacle in dir {}: {},{}", dir, &o.y, &o.x);
                    obst.push(o);
                }
            }
            if field[y-1][x] == '.'  || field[y-1][x] == 'X'{
                field[y][x] = 'X';
                y -= 1;
            } else {
                dir = 1;
            }
        } else if dir == 1 {
            if x == (dim-1).try_into().unwrap() {
                field[y][x] = 'X';
                break;
            }
            let mut x_a: usize = x;
            let mut y_a: usize = y;
            let mut o_r_found: bool = false;
            loop {
                if field[y_a][x_a] == '#' {
                    break;
                }
                if field_dirs[y_a][x_a][((dir+1)%4) as usize] == 1 {
                    o_r_found = true;
                    break;
                }
                if y_a == (dim-1) as usize {
                    break;
                }
                y_a += 1;
            }
            if o_r_found {
                let o = Coord::new(y, x+1);
                if !obst.contains(&o) {
                    println!("New obstacle in dir {}: {},{}", dir, &o.y, &o.x);
                    obst.push(o);
                }
            }
            if field[y][x+1] == '.'  || field[y][x+1] == 'X'{
                field[y][x] = 'X';
                x += 1;
            } else {
                dir = 2;
            }
            continue;
        } else if dir == 2 {
            if y == (dim-1).try_into().unwrap() {
                field[y][x] = 'X';
                break;
            }
            let mut x_a: usize = x;
            let mut y_a: usize = y;
            let mut o_r_found: bool = false;
            loop {
                if field[y_a][x_a] == '#' {
                    break;
                }
                if field_dirs[y_a][x_a][((dir+1)%4) as usize] == 1 {
                    o_r_found = true;
                    break;
                }
                if x_a == 0 as usize {
                    break;
                }
                x_a -= 1;
            }
            if o_r_found {
                let o = Coord::new(y+1, x);
                if !obst.contains(&o) {
                    println!("New obstacle in dir {}: {},{}", dir, &o.y, &o.x);
                    obst.push(o);
                }
            }
            if field[y+1][x] == '.' || field[y+1][x] == 'X'  {
                field[y][x] = 'X';
                y += 1;
            } else {
                dir = 3;
            }
            continue;
        } else if dir == 3 {
            if x == 0 {
                field[y][x] = 'X';
                break;
            }
            let mut x_a: usize = x;
            let mut y_a: usize = y;
            let mut o_r_found: bool = false;
            loop {
                if field[y_a][x_a] == '#' {
                    break;
                }
                if field_dirs[y_a][x_a][((dir+1)%4) as usize] == 1 {
                    o_r_found = true;
                    break;
                }
                if y_a == 0 as usize {
                    break;
                }
                y_a -= 1;
            }
            if o_r_found {
                let o = Coord::new(y, x-1);
                if !obst.contains(&o) {
                    println!("New obstacle in dir {}: {},{}", dir, &o.y, &o.x);
                    obst.push(o);
                }
            }
            if field[y][x-1] == '.'  || field[y][x-1] == 'X'{
                field[y][x] = 'X';
                x -= 1;
            } else {
                dir = 0;
            }
            continue;
        }
    }
    let mut sum: i64 = 0;
    for yi in 0..dim {
        for xi in 0..dim {
            if field[yi as usize][xi as usize] == 'X' {
                sum += 1;
            }
        }
    }
    println!("sum is {}", sum);
    println!("Obstacles are {}", obst.len());
    for o in obst {
        //println!("{},{}", o.y, o.x);
        field[o.y][o.x] = '+';
    }
    print_map(&field, dim );
    Ok(())
}
