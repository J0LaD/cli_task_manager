use std::io;

fn get_input()
{
    let mut list: Vec<String> = Vec::new();
    
    loop {
        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).expect("func_error");
        if user_input.trim() == "Quit" {
            break;
        }
        list.push(user_input);
    }
    print!("{:?}", list);
}

fn main()
{
    get_input();
}
