    use std::fs;
use std::error::Error;
use std::io::{self, BufRead};

fn main() {
    if std::env::args().skip(1).len() == 0
    {
        cat_no_arguments();
    }

    for arg in std::env::args().skip(1)
    {
        cat_read_file(&arg);
    }
}

fn cat_no_arguments()
{
    let stdin = io::stdin();
    loop
    {
        let mut input = String::new(); 
        stdin.lock().read_line(&mut input).unwrap();
        print!("{}", input);
    }
}

fn cat_read_file(file_name : &str)
{
    let status = fs::metadata(file_name);
    match status
    {
        Err(err) => {
            //Os { code: 2, kind: NotFound, message: "No such file or directory" }
            println!("Err: {}", err.description());
        }
        Ok(_v) => {
            let contents = fs::read_to_string(file_name).unwrap();
            print!("{}", contents);
        }
    }
}
