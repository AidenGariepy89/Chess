macro_rules! clearscr {
    () => {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    };
}

pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input failed!");
    return input;
}

