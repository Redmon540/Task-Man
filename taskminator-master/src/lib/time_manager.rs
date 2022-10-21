use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use ansi_term::{Style, Colour};
use crate::lib::{PATHP,Task, Tracker};
use crate::lib::aux::{
    read_json,
    write_json,
    date_fmt,
    append_json,
    completed_count,
};

// Start timer for task
pub fn start(_index: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let index: u8 = _index.parse::<u8>().unwrap();
    
    let mut i: u8 = 0;
    for mut task in &mut tasks {
        if i == index {
           task.status.active = true;   
           task.status.start = Local::now().format("%H:%M").to_string();
        }
        i=i+1;
    }
    write_json(&tasks, PATHP).expect("Unable to write file");
}

// Stop Timer for task
pub fn stop(_index: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let index: u8 = _index.parse::<u8>().unwrap();
    
    let mut i: u8 = 0;
    for mut task in &mut tasks {
        if i == index && task.status.active {
           task.status.active = false;   
           let now = Local::now().format("%H:%M").to_string();
           let _start = &task.status.start;
           let _tracker = Tracker {
                    date: Local::now().format("%Y-%m-%d").to_string(),
                    hours: time_difference(_start.to_string(), now) 
            };

           // Push tracker info to vector
           task.status.tracker.push(_tracker);
        }
        i=i+1;
    }
    write_json(&tasks, PATHP).expect("Unable to write file");
}

fn time_difference(start_time: String, end_time: String) -> String {
   // Get hours
   let  mut end_hour = end_time[..2].parse::<u8>().unwrap(); 
   let start_hour = start_time[..2].parse::<u8>().unwrap();

   // Get minutes
   let mut end_min = end_time[3..].parse::<u8>().unwrap();
   let start_min = start_time[3..].parse::<u8>().unwrap();

   if start_min > end_min {
       end_hour = end_hour - 1;
       end_min = end_min + 60;
   };
   
   // Calculate time difference
   let hours = end_hour - start_hour;
   let min = end_min - start_min;
   println!("{:02}:{:02}", hours, min);
   format!("{:02}:{:02}", hours, min).to_string()
}

pub fn get_total_time(tracks: &Vec<Tracker>) -> String {
    let length = tracks.len();
    let mut i = 0;
    let mut _hours = 0;
    let mut _mins = 0;

    while i < length {
        _hours += tracks[i].hours[..2].parse::<u8>().unwrap(); 
        _mins += tracks[i].hours[3..].parse::<u8>().unwrap(); 
        i += 1;
    }
   format!("{}:{}", _hours, _mins).to_string()
}
