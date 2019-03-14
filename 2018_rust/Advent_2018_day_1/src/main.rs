use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    
    println!("Hello, world!");
    
    let array = read("temp.txt").unwrap();
    //println!("{:?}", array);
    
    let result = get_total(&array);
    println!("The Resulting Frequnecy is {}",result);
    
    let result = get_first_repeate(&array);
    println!("The first repeate was: {}", result);

}

fn read(path: &str) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    println!("Entered Read Function");
    let file = File::open(path)?; // open file by given path
    
    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
      
        let line = line?;
        let line = line.replace(","," ");
        for s in line.split_whitespace()
        {
            let s = s.parse()?;
            v.push(s);
        }
    }
    Ok(v) // everything is Ok, return vector
}

fn get_total(array: &[i64]) -> i64
{
    let mut position = 0;
    for x in array 
    {
        position += x;
    }
    position
}

fn get_first_repeate(array: &[i64]) -> i64
{
    let mut values = Vec::new();
    let mut position = 0;
    let mut i = 0;
    values.push(0);
    loop
    {
        for x in array
        {
            position += x;
            for y in &values
            {
                if *y==position
                {
                    return position
                }
            }
            values.push(position);
        }
        println!("Loop Number: {} \t\t Final Value: {}", i, position);
        i += 1;
    }
}