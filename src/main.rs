use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::process;

fn append_string_to_file(file_name: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
    
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn show_todo() {
    let file = match File::open("ToDo.txt") {
        Ok(file) => file,
        Err(error) => {
            println!("Failed to open file: {}", error);
            return;
        }
    };

    let reader = BufReader::new(file);

    for (line_num, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                println!("Failed to read line: {}", error);
                return;
            }
        };

        println!("{}. {}", line_num + 1, line);
    }
}

fn main() {
    loop {
        println!("Select options:");
        println!("1. Show current ToDo List");
        println!("2. Add items into ToDo List");
        println!("3. Exit the program");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Error reading console. Please report this issue.");

        match choice.trim() {
            "1" => {
                println!("Displaying the todo list");

                show_todo();
                
                println!("Press enter to continue");

                let mut discard_input = String::new();
                io::stdin().read_line(&mut discard_input).expect("Failed to read input");
            },
            "2" => {
                println!("Selected option 2");

                let mut item = String::new();

                println!("Enter the item you want to add in the todo list");

                io::stdin().read_line(&mut item).expect("Error reading console. Please report this issue.");

                append_string_to_file("todo.txt", &item).expect("Failed to append item to the file");
            },
            "3" => {
                println!("Exiting the program");
                process::exit(0);
            },
            _ => {
                println!("Invalid choice");
            }
        };
        print!("\x1B[2J\x1B[1;1H");
    }
}
