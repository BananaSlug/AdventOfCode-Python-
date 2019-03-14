use std::io::{BufReader, BufRead};
use std::fs::File;

struct Cordinate
{
    x: u16,
    y: u16,
    finite : bool,
}

struct In_between
{
    closest_point: u16,
    distance_to_point : u16,
}


fn main() {
    
    println!("Hello, world!");
    
    let mut lines = read("temp.txt").unwrap();
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
            v.push(line);
        }
    }
    Ok(v) // everything is Ok, return vector
}

fn get_cordinates(lines: Vec<String>) -> Vec<Cordinate>
{
    let mut cordinates = Vec::new();
    
    for line in lines
    {
        let line = line.replace(","," ");
        
        let mut line = line.split_whitespace();
        
        let x : u16      = line.next().unwrap().parse().unwrap();
        let y : u16      = line.next().unwrap().parse().unwrap();
        
        let cordinate: Cordinate = Cordinate {x:x,y:y,finite:false};

        cordinates.push(cordinate);
    }
    cordinates
}

//helper function
fn get_distance(x1:int,y1:int,x2:int,y2:int)-> int
{

}

