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

fn get_input()
{
    let mut list: Vec<String> = Vec::new();

    loop {
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).expect("func_error");
        if command.trim() == "Quit" {
            break;
        } else if command.trim() == "add" {
            add_to_list(&mut list);
        } else if command.trim() == "disp" {
            display_list_content(&list);
        } else {
            println!("Key not known");
        }
    }
}

fn main()
{
    get_input();
}
