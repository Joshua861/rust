use clap::{App, Arg, Subcommand};
use crate::handlers::{add_task, list_tasks, complete_task};

#[tokio::main]
async fn main() {
    let matches = App::new("Todo App")
        .version("1.0")
        .author("Josh")
        .about("A super simple todo app in rust.")
        .arg(Arg::with_name("INPUT")
        .help("Sets the input file to use.")
        .required(true)
        .index(1))
        .subcommand(Subcommand::with_name("add")
            .about("Add a new task.")
            .arg(Arg::with_name("task")
                .help("The task to add.")
                .required(true)))
                .subcommand(Subcommand::with_name("list"))
        .subcommand(Subcommand::with_name("complete")
            .about("Marks a task as complete")
            .arg(Arg::with_name("task")
                .help("The task to mark as complete")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let task = sub_m.value_of("task").unwrap();
            add_task(task).await;
        },
        ("list", _) => {
            list_tasks.await;
        },
        ("complete", Some(sub_m)) => {
            let task = sub_m.value_of("task").unwrap();
            complete_task(task).await;
        },
        _ => unreachable!(),
    }
}
