use std::io;

fn add_to_list(list: &mut Vec<String>)
{
    let mut data: String = String::new();

    io::stdin().read_line(&mut data).expect("func_error");
    list.push(data);
}

fn get_input()
{
    let mut list: Vec<String> = Vec::new();

    loop {
        let mut command: String = String::new();
        io::stdin().read_line(&mut command).expect("func_error");
        if command.trim() == "Quit" {
            break;
        }
        if command.trim() == "add" {
            add_to_list(&mut list);
        }
    }
    print!("{:?}", list);
}

fn main()
{
    get_input();
}
