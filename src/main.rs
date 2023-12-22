use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        list(&[]);
        return;
    }
    std::fs::create_dir_all(get_pls_path()).unwrap();
    let command = &args[1];
    let extra_args = &args[2..];
    match command.as_str() {
        "help" | "-h" | "--help" | "h" => help(),
        "add" | "new" | "-a" | "-n" | "a" => add(extra_args),
        "delete" | "remove" | "-d" | "-r" | "d" | "r" | "del" => delete(extra_args),
        "version" | "-v" | "--version" | "v" => version(extra_args),
        _ => {
            if command.parse::<usize>().is_ok() {
                run(&[command.to_string()]);
            } else {
                print_unknown_command("", command);
            }
        }
    }
}

fn get_pls_path() -> PathBuf {
    dirs_next::data_dir().unwrap().join("pls")
}

fn version(args: &[String]) {
    if !args.is_empty() {
        print_unknown_command("version", &args.join(" "));
        return;
    }
    println!("pls v1.0.0");
}

fn help() {
    println!("Commands & Usage:");
    println!("> pls              List all pls");
    println!("> pls [X]          execute pls");
    println!("> pls add <C>      Add pls");
    println!("> pls delete <X>   Delete pls");
}

fn print_unknown_command(command: &str, args: &str) {
    println!("Unknown command: 'pls {} {}'. More help: 'pls help'", command, args);
}

fn get_pls_file_names() -> Vec<String> {
    let pls_path = get_pls_path();
    let mut file_names = Vec::new();
    for entry in std::fs::read_dir(pls_path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        file_names.push(file_name.to_str().unwrap().split('.').collect::<Vec<&str>>()[0].to_string());
    }
    file_names
}

fn add(args: &[String]) {
    if args.is_empty() {
        println!("Example: 'pls add cd /home/user/projects'");
        return;
    }
    let mut next_index = 1;
    let file_names = get_pls_file_names();
    if !file_names.is_empty() {
        let last_file_name = &file_names[file_names.len() - 1];
        if last_file_name.parse::<usize>().is_err() {
            std::fs::remove_file(get_pls_path().join(last_file_name)).unwrap();
            println!("Removed invalid pls: '{}'. pls not modify files by yourself.", last_file_name);
            return;
        }
        next_index = last_file_name.parse::<usize>().unwrap() + 1;
    }
    let pls_path = get_pls_path().join(format!("{}.txt", next_index));
    let new_pls = args.join(" ");

    let mut file = File::create(pls_path).unwrap();
    file.write_all(new_pls.as_bytes()).unwrap();

    println!("Added: \"{}\"", new_pls);
}

fn delete(args: &[String]) {
    if args.len() > 1 || args.is_empty() {
        println!("Example: 'pls delete 2'");
        return;
    }
    let index = match args[0].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            print_unknown_command("delete", &args[0]);
            return;
        }
    };
    let file_names = get_pls_file_names();    

    if index > file_names.len() || index < 1 {
        println!("Not found: [{}]", index);
        return;
    }

    let pls_path = get_pls_path().join(format!("{}.txt", file_names[index - 1]));
    std::fs::remove_file(pls_path).unwrap();

    println!("Deleted: [{}]", index);
}

fn run(args: &[String]) {
    if args.len() > 1 || args.is_empty() {
        print_unknown_command("run", &args.join(" "));
        return;
    }
   
    let index = match args[0].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            print_unknown_command("run", &args[0]);
            return;
        }
    };

    let file_names = get_pls_file_names();
    if index > file_names.len() || index < 1 {
        println!("Not found: [{}]", index);
        return;
    }

    let file_content = std::fs::read_to_string(get_pls_path().join(format!("{}.txt", file_names[index - 1]))).unwrap();
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &file_content])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&file_content)
            .output()
            .expect("failed to execute process")
    };
    println!("{}", String::from_utf8_lossy(&output.stplsut));
}

fn list(args: &[String]) {
    if !args.is_empty() {
        print_unknown_command("list", &args.join(" "));
        return;
    }

    let file_names = get_pls_file_names();
    if file_names.is_empty() {
        println!("Empty");
        return;
    }
    for (index, file_name) in file_names.iter().enumerate() {
        let file_content = std::fs::read_to_string(get_pls_path().join(format!("{}.txt", file_name))).unwrap();
        println!(" [{}] {}", index + 1, file_content);
    }
}