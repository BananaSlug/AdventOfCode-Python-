use std::io::{BufReader, BufRead};
use std::fs::File;



fn main() {
    
    println!("Hello, world!");
    
    let array = read("temp.txt").unwrap();
    //println!("{:?}", array);
    let result = get_num_of_2_and_3(&array);
    //println!("{:?}", result);
    
    let checksum = result.0*result.1;
    println!("Checksum is: {}", checksum);

    /* PART 2 */
    let result = compare_strings(&array);
    
    println!("The matching characters are: {:?}", result);

}

fn read(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    println!("Entered Read Function");
    let file = File::open(path)?; // open file by given path
    
    let br = BufReader::new(file);

    let mut v = Vec::new();

    for line in br.lines() {
        let line = line?;
        v.push(line);
    }
    Ok(v) // everything is Ok, return vector
}

fn get_array_of_number_times_character_used(input_line: &String) -> Vec<i64>
{
    let mut count = vec![0;26];
    let mut i = 0;
    let alphabet = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    
 //   for line in &inputFile
 //   {
    for character in alphabet
    {
        for input_char in input_line.chars()
        {
            if input_char == character
            {
               count[i] += 1; 
            }
        }
        i += 1;
    }
//    }
    count
}

fn does_contain_2_or_3(input: &[i64]) -> (bool,bool)
{
    let mut is2 = false;
    let mut is3 = false;
    for &value in input
    {
        if value == 2
        {
            is2 = true;
        }
        else if value == 3
        {
            is3 = true;
        }
    }
    return(is2,is3)
}

fn get_num_of_2_and_3(input_lines: &[String]) -> (i64, i64)
{
    let mut num_of_2 = 0;
    let mut num_of_3 = 0;
    
    for input_line in input_lines.iter()
    {
        let result = get_array_of_number_times_character_used(input_line);
        //println!("{:?}", result);
        
        let result = does_contain_2_or_3(&result);
        //println!("{:?}", result);
        if result.0 == true
        {
            //println!("2: true");
            num_of_2 += 1;
        }
        if result.1 == true
        {
            //println!("3: true");
            num_of_3 += 1;
        }
    }
    (num_of_2,num_of_3)
}

fn compare_strings(input_lines: &[String]) -> Option<String>
{
    let mut start_point = 0;
    for a_line in input_lines
    {
        start_point += 1;
        for i in start_point..input_lines.len()
        {
            if are_these_strings_off_by_one(a_line, &input_lines[i]) == true
            {
                
                return Some(get_all_matching_characters(a_line, &input_lines[i]))
            }
        }
    }
    None
}

fn are_these_strings_off_by_one(string_a: &String, string_b: &String) -> bool
{
    let ascii_string_a = string_a.as_bytes();
    let ascii_string_b = string_b.as_bytes();
    
    let mut result = Vec::new();
    for i in 0..ascii_string_a.len()
    {
        let a = ascii_string_a[i] as i8;
        let b = ascii_string_b[i] as i8;
        result.push(a-b);
    }
    
    //Count the number of zeros, we want it to be one less than length of the string
    let mut num_of_zero = 0;
    for i in &result
    {
        if *i == 0
        {
            num_of_zero += 1;
        }
    }
    
    if (num_of_zero+1) == result.len()
    {
        return true
    }
    false
}

fn get_all_matching_characters(string_a: &String, string_b: &String) -> String
{
    let mut output = "".to_string();

    for i in 0..string_a.len()
    {
        let a = string_a.chars().nth(i);
        let b = string_b.chars().nth(i);
        if a == b
        {
            output.push(a.unwrap());
        }
    }
    
    output.to_string()
}

