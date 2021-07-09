use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::exit;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    if args.len() == 0 {
        println!("Help::{} <filename>", args[0]);
        exit(1);
    }
    println!("Do work {}", args[1]);

    if let Ok(lines) = read_lines(args[1].to_string()) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }

    exit(0);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
