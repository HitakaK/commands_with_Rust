use std::collections::HashMap;

pub enum Command {
    Cut(String, HashMap<String, String>),
}

pub fn run(args: Vec<String>) -> Result<Command, String>{
    let mut opts: HashMap<String, String> = HashMap::new();

    let num_args = args.len();

    for i in 1..num_args - 1 {
        if args[i].starts_with('-') {
            if args[i + 1].starts_with('-') || i == num_args - 2 {
                opts.insert(args[i].to_string(), "True".to_string());
                println!("Recognized option: {}", &args[i]);
            }
            else {
                opts.insert(args[i].to_string(), args[i + 1].to_string());
                println!("Recognized option: {} {}", &args[i], args[i + 1]);
            }
        }
    }

    if &args[0] == "cut" {
        Ok(Command::Cut(args[num_args - 1].clone(), opts))
    }
    else {
        Err(format!("Unknown command {}", &args[0]))
    }
}
