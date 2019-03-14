use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    
    println!("Hello, world!");
    
    let mut symbols = read("temp.txt").unwrap();
    //println!("{:?}", symbols);
    react(& mut symbols);
    println!("Length: {}",symbols.len());
    part_two(&symbols)
}

fn read(path: &str) -> Result<Vec<char>, Box<dyn std::error::Error>> {
    println!("Entered Read Function");
    let file = File::open(path)?; // open file by given path
    
    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        if !line.is_empty()
        {
            for ch in line.chars()
            {
                v.push(ch);
            }
        }
    }
    Ok(v) // everything is Ok, return vector
}

fn react(symbols: & mut Vec<char>)
{
    let mut find_reaction = true;
    let mut remove_these = Vec::new();

    while find_reaction
    {
        find_reaction = false;
        
        while remove_these.len() > 0
        {
            symbols.remove(remove_these.pop().unwrap());
        }
        
        //println!("{:?}",symbols);
        
        let mut i = 0;
        while i < symbols.len()-1
        {
            if symbols[i] as u8 +32 == symbols[i+1] as u8 ||
                symbols[i] as u8-32 == symbols[i+1] as u8
            {
            //println!("{}: {} and {}", i, symbols[i], symbols[i+1]);
                remove_these.push(i);
                remove_these.push(i+1);
                i += 1; //Skip the next value because we are removing it
                find_reaction=true;
            }
            i += 1
        }
    }
}

fn part_two(symbols_origin: &Vec<char>)
{
    let mut symbols = Vec::new();
    let mut min = 9999999;
    for i in 0..26
    {
        for j in 0..symbols_origin.len()
        {
            if  symbols_origin[j] as usize != i + 'a' as usize &&
                symbols_origin[j] as usize != i + 'A' as usize
            {
                symbols.push(symbols_origin[j]);
            }
        }
        react(&mut symbols);
        println!("{}/{} length: {}", (i as u8 + 'a' as u8) as char, (i as u8 + 'A' as u8) as char, symbols.len());
        //println!("{:?}", symbols);
        
        if symbols.len() < min
        {
            min = symbols.len();
        }
        
        symbols.clear();
    }
    println!("Minimum symbols: {}", min);
}