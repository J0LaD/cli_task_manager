use std::io;

fn display_list_content(list: &Vec<String>)
{
    if list.is_empty() == true {
        println!("The list is currently empty.\n");
    } else {
        println!("--- Task list ---");
        println!("{}", list.join("\n"));
        println!("--- End of list ---\n");
    }
}

fn add_to_list(list: &mut Vec<String>)
{
    loop {
        let mut data: String = String::new();

        match io::stdin().read_line(&mut data) {
            Ok(_) => {
                let trimmed_data = data.trim();

                if trimmed_data == "end" {
                    println!("");
                    break;
                }
                list.push(trimmed_data.to_string());
            }
            Err(error) => {
                eprintln!("func read_line failed! Exit: {}", error);
            }
        }
    }
}

fn delete_from_list(list: &mut Vec<String>, task_to_delete: &str)
{
    let mut iterator = list.iter();
    let result = iterator.position(|task| task == task_to_delete);

    match result {
        Some(index) => {
             println!("{} removed.\n", list[index]);
            list.remove(index);
        }
        None => {
            println!("This task doesn't exist!\n");
        }
    }
}

fn delete(list: &mut Vec<String>)
{
    if list.is_empty() == true {
        println!("Nothing to delete!\n");
        return;
    }

    loop {
        let mut task_to_delete: String = String::new();

        match io::stdin().read_line(&mut task_to_delete) {
            Ok(_) => {
                if task_to_delete.trim() == "end" {
                    println!("");
                    break;
                }
                delete_from_list(list, &task_to_delete.trim());
            }
            Err(error) => {
                eprintln!("func read_line failed! Exit: {}", error);
            }
        }
    }
}

fn get_input()
{
    let mut list: Vec<String> = Vec::new();

    loop {
        let mut command: String = String::new();

        match io::stdin().read_line(&mut command) {
            Ok(_) => {
                match command.trim() {
                "Quit" => break,
                "add" => add_to_list(&mut list),
                "disp" => display_list_content(&list),
                "del" => delete(&mut list),
                _ => println!("Key not known.\n")
                }
            }
            Err(error) => {
                eprintln!("func read_line failed! Exit: {}", error);
            }
        }
    }
}

fn main()
{
    get_input();
}
