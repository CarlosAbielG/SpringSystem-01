use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),
    
    Display(String),
    Create(String, String), 
    Remove(String), 
    Pwd,
}

fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn print_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
}

fn perform_operation(operation: FileOperation) 
{
    match operation 
    {
        FileOperation::List(directory_path) => 
        {
            let output = Command::new("ls").arg(&directory_path).output();

            match output 
            {
                Ok(result) => 
                {
                    if result.status.success()
                    {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } 
                    else 
                    {
                        eprintln!("Failed to list files.");
                        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(_) => eprintln!("Failed to execute ls"),
            }
        }

        FileOperation::Display(file_path) => 
        {
            let output = Command::new("cat").arg(&file_path).output();

            match output 
            {
                Ok(result) => 
                {
                    if result.status.success() 
                    {
                        println!("{}", String::from_utf8_lossy(&result.stdout));
                    } 
                    else 
                    {
                        eprintln!("Failed to display file contents.");
                        
                        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                    }
                    
                }
                Err(_) => eprintln!("Failed to execute cat"),
            }
        }

        FileOperation::Create(file_path, content) => 
        {
            let safe_content = content.replace('\'', r"'\''");
            
            
            let safe_path = file_path.replace('\'', r"'\''");



            let command = format!("echo '{}' > '{}'", safe_content, safe_path);

            let output = Command::new("sh").arg("-c").arg(&command).output();

            match output 
            {
                Ok(result) => 
                {
                    if result.status.success() 
                    {
                        println!("File '{}' created successfully.", file_path);
                    } 
                    else 
                    {
                        eprintln!("Failed to create file.");
                        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                
                
                Err(_) => eprintln!("Failed to execute file creation command"),
            }
        }

        FileOperation::Remove(file_path) => 
        {
            let output = Command::new("rm").arg(&file_path).output();

            match output 
            {
                Ok(result) => 
                {
                    if result.status.success()
                    {
                        println!("File '{}' removed successfully.", file_path);
                    } 
                    else 
                    {
                        eprintln!("Failed to remove file.");
                        
                        
                        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(_) => eprintln!("Failed to execute rm"),
            }
        }

        FileOperation::Pwd => 
        {
            let output = Command::new("pwd").output();

            match output 
            {
                Ok(result) => 
                {
                    if result.status.success() 
                    {
                        print!("Current working directory: ");
                        
                        println!("{}", String::from_utf8_lossy(&result.stdout).trim());
                    } 
                    else 
                    {
                        eprintln!("Failed to get working directory.");
                        
                        
                        eprintln!("{}", String::from_utf8_lossy(&result.stderr));
                    }
                }
                Err(_) => eprintln!("Failed to execute pwd"),
            }
        }
    }
}

fn main() 
{
    println!("Welcome to the File Operations Program!");

    loop 
    {
        print_menu();
        let choice = read_input("\nEnter your choice (0-5): ");

        let operation = match choice.as_str() 
        {
            "1" =>
            {
                let dir = read_input("Enter directory path: ");
                Some(FileOperation::List(dir))
            }
            "2" => 
            {
                let file = read_input("Enter file path: ");
                Some(FileOperation::Display(file))
            }
            "3" => 
            {
                let file = read_input("Enter file path: ");
                
                let content = read_input("Enter content: ");
                Some(FileOperation::Create(file, content))
            }
            "4" => 
            {
                let file = read_input("Enter file path: ");
                Some(FileOperation::Remove(file))
            }
            "5" => Some(FileOperation::Pwd),
            "0" => 
            {
                println!("Goodbye!");
                break;
            }
            _ => 
            {
                println!("Invalid choice. Please enter a number from 0 to 5.");
                None
            }
        };

        if let Some(op) = operation 
        {
            perform_operation(op);
        }
    }
}