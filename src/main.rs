use todolib::{self, ListTypeOption, MenuOption};

fn main() -> todolib::Result<()> {
    dotenvy::dotenv()?;

    let mut db_conn = todolib::establish_connection()?;

    let menu_selection = inquire::Select::new(
        "What would you like to do?",
        vec![
            MenuOption::CreateTask,
            MenuOption::DeleteTasks,
            MenuOption::CompleteTasks,
            MenuOption::ListTasks,
        ],
    )
    .prompt()?;

    let all_tasks = todolib::get_tasks(&mut db_conn)?;

    match menu_selection {
        MenuOption::CreateTask => {
            todolib::create_task(&mut db_conn)?;
        }
        MenuOption::DeleteTasks => {
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

            for task_selection in selected_tasks {
                let task_id = todolib::task_id_from_selection(&task_selection)?;
                todolib::delete_task(&mut db_conn, task_id)?;
                println!("Deleted Task:\n{task_selection}");
            }
        }
        MenuOption::CompleteTasks => {
            if all_tasks.is_empty() {
                println!("No tasks to complete");
                return Ok(());
            }

            let completed_tasks = all_tasks
                .iter()
                .filter(|task| task.complete)
                .map(|task| format!("{}: {}", task.id, task.title))
                .collect::<Vec<_>>();

            let selected_tasks =
                inquire::MultiSelect::new("Select Tasks To Complete", completed_tasks).prompt()?;

            for selected_task in selected_tasks {
                let task_id = todolib::task_id_from_selection(&selected_task)?;
                todolib::complete_task(&mut db_conn, task_id)?;
            }
        }
        MenuOption::ListTasks => {
            let list_type_selection = inquire::Select::new(
                "What type of task would you like to see?",
                vec![
                    ListTypeOption::All,
                    ListTypeOption::Completed,
                    ListTypeOption::Incomplete,
                ],
            )
            .prompt()?;

            match list_type_selection {
                ListTypeOption::All => {
                    if all_tasks.is_empty() {
                        println!("No tasks to list");
                        return Ok(());
                    }

                    for task in all_tasks.iter() {
                        println!("{task}");
                    }
                }
                ListTypeOption::Completed => {
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
                ListTypeOption::Incomplete => {
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
            }
        }
    };

    Ok(())
}
