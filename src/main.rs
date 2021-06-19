/*use serde_json::Result;*/
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use chrono::prelude::*;
use clap::Clap;
use postgres::{Client, Error, NoTls};

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
 #[derive(Debug)]
struct Task {
    _id: i32,
    name: String,
    description: String,
    duration: i32,
    due_date: String,
    start_date: String,
}

impl Task {
    fn new(client: &Client, name: String, description: String, duration: i32) -> Result<Task, Error> {
        let task = Task {
            _id: 0,
            name: name.to_string(),
            description: description.to_string(),
            duration,
            start_date: Local::today().to_string(),
            due_date: Local::today().to_string(),
        };
        client.execute(
            "INSERT INTO task (name, description, duration, start_date, due_date) values ($1, $2, $3, $4, $5)",
            &[&task.name, &task.description, &task.duration, &task.start_date, &task.due_date],
        )?;

        Ok(task)
    }

    /* fn read<P: AsRef<Path>>(path: P) -> Result<Task, Error> {

         let file = File::open(path).unwrap();
         let reader = BufReader::new(file);

         let t = serde_json::from_reader(reader)?;

         Ok(t)

     }*/
}

fn main() -> Result<(), Error> {

    //   let mut tasks = HashMap::new();
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS task (
            id      SERIAL PRIMARY KEY,
            name  VARCHAR NOT NULL,
            duration   INTEGER NOT NULL,
            description  VARCHAR NOT NULL,
            due_date  VARCHAR NOT NULL,
            start_date  VARCHAR NOT NULL,
        )
        ");
    let opts = Opt::parse();

    let mut tasks: Vec<Task> = Vec::new();


    println!("{:?}", opts);
    match opts.cmd {
        Args::Add {
            duration,
            name,
            description,
        } => {
            println!("add was used");
            let new_task = Task::new(&client, name, description, duration).unwrap();

            for row in client.query("SELECT id, name, duration, description, due_date, start_date\
             FROM task", &[])? {
                let task = Task {
                    _id: row.get(0),
                    name: row.get(1),
                    duration: row.get(2),
                    description: row.get(3),
                    due_date: row.get(4),
                    start_date: row.get(5),
                };
                println!("task {} is {}", task.name, task.description);
            }
            /*            let b = std::path::Path::new("db.json").exists();
            if b {
                let f = File::open("db.json").expect("file error");
                let buf = BufReader::new(f);
                let mut c: Vec<Task> = serde_json::from_reader(buf).expect("error reading json into a task type");
                tasks.append(&mut c);
            }
            tasks.push(new_task);

            let content = serde_json::to_string(&tasks)?;

            std::fs::write("db.json", content).unwrap();

            println!("{:?}", tasks)
            */
        },
        Args::Status { name, all } => {
            println!("{:?}", tasks);
        }
        Args::Remove { name, all } => {
            println!("remove was used");
        }
        _ => eprintln!("footsies")
    }
    Ok(())
}
