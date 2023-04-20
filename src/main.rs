use std::{fs::{File, self}, path::Path, io::{BufReader, BufRead}, prelude::*};

fn day1() {
    let p = Path::new("resources").join("d1.txt");
    
    let f = File::open(p).expect("Could not open file");
    let reader = BufReader::new(f);

    let mut sums: Vec<(u32, u32)> = vec![];
    
    let mut total = 0;
    let mut current = 1;
    for line in reader.lines() {
        let value = line.unwrap();
        match &value.as_str() {
            &"" => {
                sums.push((current, total));
                current += 1;
                total = 0;
            }
            _ => {
                // convert to string and add to total
                let v: u32 = str::parse(&value.as_str()).expect("Cant parse");
                total += v;
            }
        }
    }

    sums.sort_by(|a , b| b.1.cmp(&a.1));

    for sum in &sums {
        println!("{}: {}", sum.0, sum.1);
    }

    println!("{}", sums.get(0).unwrap().1 + sums.get(1).unwrap().1 + sums.get(2).unwrap().1)
}

fn main() {
    println!("Running");
    day1();
}
