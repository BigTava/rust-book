use std::fs::File;
use std::io::{Error, ErrorKind, Read};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // panic!("crash and burn");

    let greeting_file_result: Result<File, Error> = File::open("hello.txt");

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // or

    let greeting_file: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // or

    let greeting_file: File = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result: Result<File, Error> = File::open("hello.txt");

    let mut username_file: File = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username: String = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    };

    // or

    let mut username_file: File = File::open("hello.txt")?;
    let mut username: String = String::new();
    username_file.read_to_string(&mut username)?;
    let _result: Result<String, Error> = Ok(username);

    // or 

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    let _result: Result<String, Error> = Ok(username);

    // or

    fs::read_to_string("hello.txt")
}