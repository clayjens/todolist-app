use todolib;

fn main() -> todolib::Result<()> {
    dotenvy::dotenv()?;
    let mut db_conn = todolib::establish_connection()?;

    let menu_selection = inquire::Select::new(
        "What would you like to do?",
        vec![
            "Create A Task",
            "Delete Tasks",
            "Complete Tasks",
            "List Tasks",
        ],
    )
    .prompt()?;

    let all_tasks = todolib::get_tasks(&mut db_conn)?;

    match menu_selection {
        "Create A Task" => {
            todolib::create_task(&mut db_conn)?;
        }
        "Delete Tasks" => {
            if all_tasks.is_empty() {
                println!("No tasks to delete");
                return Ok(());
            }

            let selected_tasks = inquire::MultiSelect::new(
                "Select Tasks To Delete",
                all_tasks
                    .iter()
                    .map(|task| format!("{}: {}", task.id, task.title))
                    .collect(),
            )
            .prompt()?;

            for task in selected_tasks {
                let task_id = task
                    .split(": ")
                    .next()
                    .expect("Failed to get task id")
                    .parse::<i32>()?;

                todolib::delete_task(&mut db_conn, task_id)?;
                println!("Deleted Task:\n{task}");
            }
        }
        "Complete Tasks" => {
            if all_tasks.is_empty() {
                println!("No tasks to complete");
                return Ok(());
            }

            let selected_tasks = inquire::MultiSelect::new(
                "Select Tasks To Complete",
                all_tasks
                    .iter()
                    .filter(|task| !task.complete)
                    .map(|task| format!("{}: {}", task.id, task.title))
                    .collect(),
            )
            .prompt()?;

            for task in selected_tasks {
                let task_id = task
                    .split(": ")
                    .next()
                    .expect("Failed to get task id")
                    .parse::<i32>()?;

                todolib::complete_task(&mut db_conn, task_id)?;
            }
        }
        "List Tasks" => {
            let list_type = inquire::Select::new(
                "What type of list would you like to see?",
                vec!["All", "Complete", "Incomplete"],
            )
            .prompt()?;

            match list_type {
                "All" => {
                    if all_tasks.is_empty() {
                        println!("No tasks to list");
                        return Ok(());
                    }

                    for task in all_tasks.iter() {
                        println!("{task}");
                    }
                }
                "Complete" => {
                    let complete_tasks = all_tasks
                        .iter()
                        .filter(|task| task.complete)
                        .collect::<Vec<_>>();

                    if complete_tasks.is_empty() {
                        println!("No tasks are complete");
                        return Ok(());
                    }

                    for task in complete_tasks {
                        println!("{task}");
                    }
                }
                "Incomplete" => {
                    let incomplete_tasks = all_tasks
                        .iter()
                        .filter(|task| !task.complete)
                        .collect::<Vec<_>>();

                    if incomplete_tasks.is_empty() {
                        println!("No tasks are incomplete");
                        return Ok(());
                    }

                    for task in incomplete_tasks {
                        println!("{task}");
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };

    Ok(())
}
