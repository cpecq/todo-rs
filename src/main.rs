use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "todo-rs",
    about = "a simple todo app to set timers and switch tasks"
)]
enum todo_rs {
    /// Add a new task to the task list!
    Add {
        /// Specify the time in hours that the task is estimated to take. Default is one hour.
        #[structopt(short, long, default_value = "1")]
        time: f32,

        /// Name of the task needed to complete.
        #[structopt(short, long)]
        name: String,

        /// Description of the task being completed.
        #[structopt(short, long)]
        description: String,

        /// A due-date if the task is an ongoing task, fmt "15-01-2021"
        #[structopt(name = "<Due Date>")]
        due: Option<String>,
    },

    /// Get the status of a targetted or all tasks.
    Status {
        /// Returns the status of a targetted task.
        #[structopt(name = "Task Name")]
        name: String,

        /// Returns the status of all ongoing tasks.
        #[structopt(short, long)]
        all: bool,
    },

    Remove {
        #[structopt(name = "Task Name")]
        name: String,

        #[structopt(short, long)]
        all: bool,
    },
}

fn main() {
    println!("Hello, world!");
}
