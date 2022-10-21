// Command Functions
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use ansi_term::{Style, Colour};

pub mod time_manager;
use time_manager::{
    get_total_time    
};

mod aux;
use aux::{
    read_json,
    write_json,
    date_fmt,
    append_json,
    completed_count
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tracker {
    date: String,
    hours: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    active: bool,
    start: String,
    tracker: Vec<Tracker>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    task: String,
    due: String,
    project: String,
    status: State
}

const PATHP: &str = "/usr/bin/db/pending.json";
const PATHC: &str = "/usr/bin/db/completed.json";

pub fn help() {
        println!("usage:
                 match_args <string>
                     Check whether given string is the answer.
                     match_args {{increase|decrease}} <integer>
                         Increase or decrease given integer by one.");
}

pub fn list_tasks() {
    let tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    
    let mut pending_c: u8 = 0;
    let style = Style::new().underline();
    println!("\n{}|{}|{}|{}|{}", 
    style.paint(" Id "),
    style.paint(" Project      "), 
    style.paint(" Due      "), 
    style.paint(" Time "),
    style.paint(" Description               "));

    //Print tasks
    for(i, task) in tasks.iter().enumerate() {
        println!("{:^5}{:<15}{:<12}{:<5}{}", i, task.project, task.due, get_total_time(&task.status.tracker), task.task);
        pending_c = pending_c + 1;
    }

    //Get Today's date
    let today: String = Local::now().format("%Y-%m-%d").to_string();
    println!("\n| Completed today: {} | Total pending: {} |\n", completed_count(today), pending_c);
}

pub fn remove_task(id: &str) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");

    //Remove 
    tasks.remove((id.parse::<u8>().unwrap()).into());

    // Write to file
    write_json(&tasks, PATHP).expect("Couldn't write to file");
}

pub fn add_task(todo: Vec<String>) {
    let mut tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let mut _tracker: Vec<Tracker> = Vec::with_capacity(5);

    // Init tracker Vector
    let tracker_info = Tracker {
            date: "".to_string(),
            hours: "00:00".to_string()
    };
    _tracker.push(tracker_info);
    
    // Init State struct
    let state = State {
        active: false,
        start: "0".to_string(),
        tracker: _tracker
    };

    //Create Task
    let mut task =  Task {
        task: (&todo[0]).to_string(),
        due: Local::now().format("%Y-%m-%d").to_string(),
        project: "Personal".to_string(),
        status: state
    };


    let mut incorrect_cmd: bool = false;
    //Check for correct commands
    if todo.len() == 3 {
       incorrect_cmd = match_cmd(todo[1..].to_vec(), &mut task);
    } else if todo.len() == 5 {
       if match_cmd(todo[1..3].to_vec(), &mut task) || 
          match_cmd(todo[3..].to_vec(), &mut task) 
          {
            incorrect_cmd = true;
          }
    } else if todo.len() > 1{
        incorrect_cmd = true;
        println!("Error: Incorrect Command or Incomplete");
    }
    
    if !incorrect_cmd {
        //Update and Write
        tasks.push(task);
        write_json(&tasks, PATHP).expect("Unable to write file");

        println!("Task Added: {}", &todo[0]);
    } 
}

pub fn completed_task(index: &str) {
    let tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let task_i: u8 = index.parse::<u8>().unwrap();


    let mut i: u8 = 0;
    let today: String = Local::now().format("%Y-%m-%d").to_string();
    //Move to completed.json
    for mut task in tasks {
        if i == task_i {
            task.due = today;
            append_json(task,PATHC);
            remove_task(index);
            println!("Task {} Added as completed!",&index);
            break;
        }
        i = i+1;
    }
}

pub fn update_task(index: &str, args: Vec<String>) {
    let tasks: Vec<Task> = read_json(PATHP).expect("error while reading");
    let task_i: u8 = index.parse::<u8>().unwrap();
    
    let mut i: u8 = 0;
    // Update fields
    for mut task in tasks {
        if i == task_i && !match_cmd(args.to_vec(), &mut task){
            remove_task(index);
            append_json(task,PATHP);
            println!("Task {} Updated!",&index);
            break;
        }
        i = i+1;
    }
}

// Aux Function
fn match_cmd(args: Vec<String>, task: &mut Task) -> bool {
    match args[0].as_str() {
        "-d" => {
            //Insert Date
            date_fmt(&args[1], task);
            false
        }
        "-p" => {
            //Insert Project
            task.project = (&args[1]).to_string();
            false
        } 
        _=> {
            println!("Incorrect command {} please check --help to see all commands", args[0]);
            println!("Date Format has to be: YYYY-MM-DD");
            true
        }
    }
}
