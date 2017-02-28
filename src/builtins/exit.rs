use std::process;

pub fn exit(args: &Vec<String>) -> bool {
    if args.len() > 0 {
        match args[0].parse::<i32>() {
            Ok(status) => process::exit(status),
            Err(_) => {
                println!("exit requires numberic value");
                process::exit(0);
            }
        }
    } else {
        process::exit(0);
    }
}