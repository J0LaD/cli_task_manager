use std::io;

fn display_list_content(list: &Vec<String>)
{
    print!("{}", list.join(""));
}

fn add_to_list(list: &mut Vec<String>)
{
    loop {
        let mut data: String = String::new();
        match io::stdin().read_line(&mut data) {
            Ok(_) => {
                if data.trim() == "end" {
                    break;
                }
                list.push(data);
            }
            Err(error) => {
                eprintln!("func read_line fail! Exit: {}", error);
            }
        }
    }
}

fn delete_from_list(list: &mut Vec<String>, task_to_delete: &str)
{
    if let Some(index) = list.iter().position(|task| task == task_to_delete) {
        println!("{} removed.", list[index]);
        list.remove(index);
    } else {
        println!("This task doesn't exist!");
    }
}

fn delete(list: &mut Vec<String>)
{
    if list.is_empty() == true {
        println!("No more things to delete!");
        return;
    }

    loop {
        let mut task_to_delete: String = String::new();
        match io::stdin().read_line(&mut task_to_delete) {
            Ok(_) => {
                if task_to_delete.trim() == "end" {
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
        io::stdin().read_line(&mut command).expect("func_error");
        match command.trim() {
            "Quit" => break,
            "add" => add_to_list(&mut list),
            "disp" => display_list_content(&list),
            "del" => delete(&mut list),
            _ => println!("Key not known")
        }
    }
}

fn main()
{
    get_input();
}
