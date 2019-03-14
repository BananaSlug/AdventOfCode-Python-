use std::io::{BufReader, BufRead};
use std::fs::File;
//use ndarray;

struct Claim
{
    number: i32,
    x: u16,
    y: u16,
    size_x: u16,
    size_y: u16,
    unoverlaped: u16,
}

const SIZE_OF_FABRIC: usize = 1000;

fn main() {
    
    println!("Hello, world!");
    
    let lines = read("temp.txt").unwrap();
    let mut claims = fill_claims(lines);
 
    //declare our big array
    let fabric = layout_fabric(&claims);
    
    let count = count_overlap(&fabric);
    
    println!("There are {} squares of overlap", count);  

    //Part 2:
    count_unoverlap(fabric, &mut claims);
    
    find_non_overlap_pattern(claims);
    
}

fn read(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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

fn fill_claims(lines: Vec<String>) -> Vec<Claim>
{
    let mut claims = Vec::new();
    
    for line in lines
    {
    //Remove all other non-useful character:
    // #1 @ 393,863: 11x29
    // to:
    //  1   393 863  11 29
        let line = line.replace("#"," ");
        let line = line.replace("@"," ");
        let line = line.replace(","," ");
        let line = line.replace(":"," ");
        let line = line.replace("x"," ");
        
        let mut line = line.split_whitespace();
        
        let number: i32  = line.next().unwrap().parse().unwrap();
        let x : u16      = line.next().unwrap().parse().unwrap();
        let y : u16      = line.next().unwrap().parse().unwrap();
        let size_x : u16 = line.next().unwrap().parse().unwrap();
        let size_y : u16 = line.next().unwrap().parse().unwrap();
        
        
        let claim: Claim = Claim {number:number, x:x,y:y,size_x:size_x,size_y:size_y,unoverlaped:0};
        //println!("number:{} x:{} y:{} size_x:{} size_y:{}", number.to_string(), x.to_string(),y.to_string(),size_x.to_string(),size_y.to_string());
        //claim.number = line.next().unwrap().parse().unwrap();
        
        claims.push(claim);
    }
    claims
}

fn layout_fabric(claims: &Vec<Claim>) -> Vec<Vec<i32>>
{
    let mut fabric = vec![vec![0i32;SIZE_OF_FABRIC]; SIZE_OF_FABRIC];

    for claim in claims
    {
        for i in claim.x..(claim.x+claim.size_x)
        {
            for j in claim.y..(claim.y+claim.size_y)
            {
                if fabric[i as usize][j as usize] == 0
                {
                    fabric[i as usize][j as usize] = claim.number;
                }
                else
                {
                    fabric[i as usize][j as usize] = -1;
                }
            }
        }
    }
    fabric
}

fn count_overlap(fabric: &Vec<Vec<i32>>) -> i32
{
    let mut count = 0;
    for i in 0..SIZE_OF_FABRIC
    {
        for j in 0..SIZE_OF_FABRIC
        {
            if fabric[i as usize][j as usize] == -1
            {
                count = count+1;
            }
        }
    }
    count
}

fn count_unoverlap(fabric: Vec<Vec<i32>>, claims: &mut Vec<Claim>)
{
    for i in 0..SIZE_OF_FABRIC
    {
        for j in 0..SIZE_OF_FABRIC
        {
            if fabric[i as usize][j as usize] > 0
            {
                let index = fabric[i as usize][j as usize] - 1;
                claims[index as usize].unoverlaped = claims[index as usize].unoverlaped + 1;
            }
        }
    }
}

fn find_non_overlap_pattern(claims: Vec<Claim>)
{
    for claim in claims
    {
        if claim.unoverlaped == (claim.size_x*claim.size_y)
        {
            println!("Unoverlapped claim: {}", claim.number);
        }
    }
}