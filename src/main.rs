use serde::{Deserialize, Serialize};
use serde_json::Result;
use chrono::prelude::*;
use clap::Clap;

use std::collections::HashMap;
use std::fmt;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Clap)]
#[clap(
    name = "todo-rs",
    about = "a simple todo app to set timers and switch tasks"
)]
struct Opt {
    #[clap(subcommand)]
    cmd: Args,
}

#[derive(Debug, Clap)]
enum Args {
    /// Add a new task to the task list!
    Add {
        /// Specify the time in hours that the task is estimated to take. Default is one hour.
        #[clap(default_value = "1")]
        duration: i32,

        /// Name of the task needed to complete.
        #[clap(short, long)]
        name: String,

        /// Description of the task being completed.
        #[clap(short, long)]
        description: String,
        //    /// A due-date if the task is an ongoing task, fmt "15-01-2021"
        //#[clap(name = "<Due Date>", default_value)]
        //due_date: String,
    },

    /// Get the status of a targetted or all tasks.
    Status {
        /// Returns the status of a targetted task.
        #[clap(name = "Task Name")]
        name: String,

        /// Returns the status of all ongoing tasks.
        #[clap(short, long)]
        all: Option<bool>,
    },

    /// Remove all or targetted todo tasks
    Remove {
        /// Remove a specific taks from the todo list.
        #[clap(name = "Task Name")]
        name: String,

        /// Remove all tasks from the todo list: do you really want to delete everything?
        #[clap(short, long)]
        all: Option<bool>,
    },
}

#[derive(Hash, Eq, PartialEq)]
#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    description: String,
    duration: i32,
    due_date: String,
}

impl Task {
    fn new(name: String, description: String, duration: i32) -> Result<Task> {
        // reformat new function to create a disk storage of todo tasks if there is not one there already/
        // f will open file if it is there or create a file if it is not

        let task = Task {
            name,
            description,
            duration,
            due_date: Local::today().to_string(),
        };
                Ok(task)
    }

    fn read<P: AsRef<Path>>(path: P) -> Result<Task> {

        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let t = serde_json::from_reader(reader)?;

        Ok(t)

    }
}

fn main() -> Result<()>{
 //   let mut tasks = HashMap::new();
    let opts = Opt::parse();

    let mut tasks= Vec::new();


    println!("{:?}", opts);
    match opts.cmd {
        Args::Add {
            duration,
            name,
            description,
        } => {
            println!("add was used");
            let new_task = Task::new(name, description, duration).unwrap();



            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("db.json").unwrap();
            let mut c = String::new();
            f.read_to_string(&mut c).unwrap();
            let old_task: Task = serde_json::from_str(&mut c).unwrap();
            tasks.push(old_task);
            tasks.push(new_task);

            let content = serde_json::to_string(&tasks)?;

            std::fs::write("db.json", content).unwrap();



            println!("{:?}", tasks)
        },
        Args::Status { name, all } => {
            println!("{:?}", tasks);

        },
        Args::Remove { name, all } => {
            println!("remove was used");
        }
        _ => eprintln!("footsies")
    }


    Ok(())
}
