use std::io;

fn get_input()
{
    let mut user_input = String::new();

    loop {
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("func_error");
        print!("{}", user_input);
    }
}

fn main()
{
    get_input();
}
