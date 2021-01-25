use chrono::prelude::*;
use clap::Clap;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

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

#[derive(Debug, Hash, Eq, PartialEq, Display)]
struct Task {
    name: String,
    description: String,
    duration: i32,
    due_date: String,
}

impl Task {
    fn new() -> Result<Task, std::io::Error> {
        // reformat new function to create a disk storage of todo tasks if there is not one there already/
        // f will open file if it is there or create a file if it is not
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;
        // Need to add in reading the hashmap into a new task -> maybe serialise and deserialise as json
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap(Task, bool)
        let task = Task {
            name,
            description,
            duration,
            due_date: Local::today().to_string(),
        };

        Ok(task)
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}, {} ", self.name, self.description, self.duration, self.due_date)
    }
}

fn save(map: &mut HashMap<Task, bool>) -> Result<(), std::io::Error> {
        let mut content = String::new();
        for(k, v) in map {
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        std::fs::write("db.txt", content)
    }


fn main() {
    let mut tasks = HashMap::new();
    let opts = Opt::parse();

    println!("{:?}", opts);
    match opts.cmd {
        Args::Add {
            duration,
            name,
            description,
        } => {
            println!("add was used");
            let new_task = Task::new(duration, name, description).unwrap();
            tasks.insert(new_task, true);
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
    save(&mut tasks).unwrap();
}
