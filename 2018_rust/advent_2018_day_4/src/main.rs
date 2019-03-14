extern crate chrono;

use std::io::{BufReader, BufRead};
use std::fs::File;
use chrono::prelude::*;


struct Input
{
    time: DateTime<Utc>,
    task: Task,
    guard: u32,
}

#[derive(PartialEq, Eq)]
enum Task
{
    Start,
    Sleep,
    Awake,
}

struct Day
{
    guard: u32,
    hour: [i32;60],
}

fn main() {
    
    println!("Hello, world!");
    
    let lines = read("temp.txt").unwrap();
    let mut input = process_line(lines);
    input.sort_by(|a, b| a.time.cmp(&b.time));
    let mut days = get_guard_data(input);
    days.sort_by(|a, b| a.guard.cmp(&b.guard));
    //Right here the struct Day lost its meaning,
    //because now it contains multiple days!
    let stats = get_guard_stats(days);
    solve_part_one(&stats);
    solve_part_two(&stats);
    
    
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

fn process_line(lines: Vec<String>) -> Vec<Input>
{
    let mut inputs = Vec::new();

    for line in lines
    {
        let task: Task;
        if line.find("begin").is_some()
        {
            task = Task::Start;
        }
        else if line.find("falls").is_some()
        {
            task = Task::Sleep;
        }
        else
        {
            task = Task::Awake;
        }
        
        let line = line.replace("["," ");
        let line = line.replace("]"," ");
        let line = line.replace("-"," ");
        let line = line.replace(":"," ");
        let line = line.replace("#"," ");
        let line = line.replace("Guard"," ");
        let line = line.replace("begins shift"," ");
        let line = line.replace("falls asleep"," ");
        let line = line.replace("wakes up"," ");
        
        let mut line = line.split_whitespace();

        let year: i32  = line.next().unwrap().parse().unwrap();
        let month : u32      = line.next().unwrap().parse().unwrap();
        let day : u32      = line.next().unwrap().parse().unwrap();
        let hour : u32 = line.next().unwrap().parse().unwrap();
        let minute : u32 = line.next().unwrap().parse().unwrap();
        let mut guard : u32 = 0;
        if task == Task::Start
        {
            guard = line.next().unwrap().parse().unwrap();
        }
        
        let dt = Utc.ymd(year, month, day).and_hms(hour,minute,0);
        
        
        let input: Input = Input {  time:dt,
                                    task:task,
                                    guard:guard};
        inputs.push(input);
    }
    inputs
}

fn get_guard_data(input: Vec<Input>) -> Vec<Day>
{
    let mut days : Vec<Day> = Vec::new();

    let mut cur_time = 0;
    let mut cur_guard = 0;
    let mut cur_state;
    let mut cur_hour : [i32;60] = [0;60];
    for cur in input
    {
        if cur.task == Task::Start
        {
            if cur_guard != 0
            {
                days.push(Day {guard:cur_guard, hour:cur_hour});
            }
            
            cur_guard = cur.guard;
            cur_hour = [0;60];
        }
        else
        {
      
            if cur.task == Task::Sleep
            {
                cur_state = 0;
            }
            else
            {
                cur_state = 1;
            }

      
//            for min in cur_hour.iter()
//            {
//                print!("{}", min);
//            }
//            print!("\n");
        
            for i in cur_time..cur.time.minute()
            {
                cur_hour[i as usize] = cur_state;
            }

        }
            
        
        cur_time = cur.time.minute();
//        println!("{}", cur_time);
    }
    
    
    
    days.push(Day {guard:cur_guard, hour:cur_hour});
    
    days
//    for day in days
//    {
//        for min in day.hour.iter()
//        {
//            print!("{}", min);
//        }
//        print!("\n");
//    }
}

fn get_guard_stats(days : Vec<Day>) -> Vec<Day>
{
    let mut stat : Vec<Day> = Vec::new();
    let mut prev_guard = 0;
    let mut cur_index = 0;
    for day in days
    {
        if prev_guard != day.guard
        {
            cur_index = stat.len();
            stat.push(Day {guard:day.guard,hour:[0;60]});
            prev_guard = day.guard;
        }
        
        for i in 0..60
        {
            stat[cur_index].hour[i] += day.hour[i];
        }
    }

    stat

//   for day in stat
//   {
//       for min in day.hour.iter()
//       {
//           print!("{}", min);
//       }
//       print!("\n");
//   }
}

fn solve_part_one(stats: &Vec<Day>)
{
    let mut most_asleep = 0;
    let mut most_asleep_min = 0;
    let mut most_asleep_guard = 0;
    for stat in stats
    {
        if stat.hour.iter().sum::<i32>() > most_asleep
        {
            most_asleep = stat.hour.iter().sum::<i32>();
            
            let temp = stat.hour.iter().max().unwrap();
            most_asleep_min = stat.hour.iter().position(|&x| x == *temp).unwrap();
            most_asleep_guard = stat.guard;
        }
    }
    
    println!("Part 1: most asleep Guard: {}, most asleep minute: {}, checksum: {}", most_asleep_guard, most_asleep_min, most_asleep_guard*most_asleep_min as u32);
}


fn solve_part_two(stats: &Vec<Day>)
{
    let mut most_asleep : i32 = 0;
    let mut most_asleep_min = 0;
    let mut most_asleep_guard = 0;
    for stat in stats
    {
        if stat.hour.iter().max().unwrap() > &most_asleep
        {
            most_asleep = *stat.hour.iter().max().unwrap();
            most_asleep_min = stat.hour.iter().position(|&x| x == most_asleep).unwrap();
            most_asleep_guard = stat.guard;
        }
    }
    
    println!("Part 2: most asleep Guard: {}, most asleep minute: {}, checksum: {}", most_asleep_guard, most_asleep_min, most_asleep_guard*most_asleep_min as u32);


}
