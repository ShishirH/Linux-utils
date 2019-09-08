use std::fs;
use colored::*;
use std::os::unix::fs::PermissionsExt;

fn main() -> std::io::Result<()>
{
    let mut additional : bool = false;
    let arguments_length = std::env::args().skip(1).len();
    if arguments_length == 0
    {
        read_dir(".", additional)?;
        std::process::exit(1);
    }
    for arg in std::env::args().skip(1)
    {
        if arg.eq_ignore_ascii_case("-l")
        {
            additional = true;
            if arguments_length == 1
            {
                read_dir(".", additional)?;
            }
            continue;
        }
        read_dir(&arg, additional)?;
    }
    Ok(())
}

fn read_dir(path : &str, additional : bool) -> std::io::Result<()>
{
    println!("\n{:?}\n", fs::canonicalize(path)?);
    let mut i = 0;
    for file in fs::read_dir(path)?
    {
        let dir = file?;
        let metadata = dir.metadata()?;
        let path = dir.path();

        // let sample = dir.path().to_string_lossy();
        // println!("{}", sample);

        let additional_info = match additional
        {
            true => metadata.permissions().mode().to_string(),
            _ => String::new()
        };

        let file_name = match path.file_name()
        {
            Option::Some(v) => v,
            _ => { std::process::exit(1);}
        };

        if file_name.to_string_lossy().starts_with(".") && !additional
        {
            continue;
        }

        if metadata.is_dir() 
        {
            print!("{}{}", format!("{: <50}", file_name.to_string_lossy().red()), format!("{: <15}", additional_info));
            i += 1;
        }
        else
        {
            print!("{}{}", format!("{: <50}", file_name.to_string_lossy()), format!("{: <15}", additional_info));
            i += 1;
        }

        if i % 2 == 0 || file_name.to_string_lossy().len() > 50
        {
            println!("");
        }        
    }

    Ok(())
}
