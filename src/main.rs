use std::io;

fn main() {
    let context = 0u8;
    let to_continue = true;
    let mut state = State { context, to_continue };
    println!("############################");
    println!("A Fibonacci Number Generator");
    println!("############################");
    while state.to_continue {
        let mut input = String::new();
        print_options(state.context);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read");

        state = process_input(&mut input, state);
    }

    println!("Bye-Bye!");
}

struct State {
    context: u8,
    to_continue: bool,
}

fn print_options(current_ctx: u8) {
    match current_ctx {
        0u8 => println!("1. Get the n-th Fibonacci Number.\nq. Quit."),
        1u8 => println!("Enter n:"),
        _ => println!(""),
    }
}

fn process_input(input: &mut String, state: State) -> State {
    match input.trim() {
        "q" => {
            State { context: 0u8, to_continue: false }
        },
        _ => {
            if state.context == 0u8 {
                let new_context = match input.trim().parse::<u8>() {
                    Ok(num) => num,
                    Err(error) => {
                        println!("{}", error);
                        0
                    }
                };
                State { context: new_context, to_continue: true }
            } else {
                let n = match input.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(error) => {
                        println!("{}", error);
                        0u32
                    }
                };
                let nth_fibonacci_number = get_nth_fibonacci_number(n);
                print!("The {}-th fibonacci number is {}.\n\n", n, nth_fibonacci_number);
                State { context: 0u8, to_continue: true }
            }
        }
    }
}

fn get_nth_fibonacci_number(n: u32) -> u32 {
    match n {
        0u32 => 0u32,
        1u32 => 1u32,
        _ => get_nth_fibonacci_number(n - 1u32) + get_nth_fibonacci_number(n - 2u32),
    }
}