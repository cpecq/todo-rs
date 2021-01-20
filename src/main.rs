use chrono::prelude::*;
use clap::Clap;
use std::error::Error;

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
        duration: f32,

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

#[derive(Debug)]
struct Task {
    name: String,
    description: String,
    duration: f32,
    due_date: String,
}

impl Task {
    fn new(duration: f32, name: String, description: String) -> Result<Task, Box<dyn Error>> {
        let task = Task {
            name,
            description,
            duration,
            due_date: Local::today().to_string(),
        };

        Ok(task)
    }
}

fn main() {
    let opts = Opt::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Args::Add {
            duration,
            name,
            description,
        } => {
            println!("add was used");
            let task = Task::new(duration, name, description).unwrap();
            println!("{:?}", task)
        }
        Args::Status { name, all } => println!("status was used"),
        Args::Remove { name, all } => println!("remove was used"),
        _ => eprintln!("footsies"),
    }
}
