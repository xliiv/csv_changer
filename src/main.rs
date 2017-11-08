// http://words.steveklabnik.com/the-expressive-c-17-coding-challenge-in-rust



use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::io;


const SEP: char = ',';


enum Error {
    CSV(String),
    Input(String),
    OS(String),
    Io(io::Error),
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}


fn _main() -> Result<(), Error> {
    // get prog args
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() != 4 {
        let msg =
            "Program requires 4 args, example:\n$ csv_changer input.csv city london output.csv";
        return Err(Error::Input(msg.into()));
    }

    // open file
    let mut csv_content = String::new();
    File::open(&args[0])
        .map_err(|e| {
            Error::OS(format!("Can't open file: {:?} ({})", args[0], e))
        })?
        .read_to_string(&mut csv_content)?;


    // replace data
    let mut new_content = String::with_capacity(csv_content.capacity());
    // process header
    let mut lines = csv_content.lines();
    let heading = lines
        .next()
        .ok_or_else(|| Error::CSV("Can't get header of CSV".into()))?;
    new_content.push_str(heading);
    new_content.push('\n');
    // process body
    let col_idx = heading
        .split(SEP)
        .position(|v| v == args[1])
        .ok_or_else(|| {
            Error::CSV(format!(
                "Missing column {:?} in file {:?}",
                args[1],
                args[0]
            ))
        })?;
    for line in lines {
        let mut values: Vec<&str> = line.split(SEP).collect();
        values[col_idx] = &args[2];
        new_content.push_str(&values.join(","));
        new_content.push('\n');
    }


    // save file
    let mut output_file = File::create(&args[3]).map_err(|e| {
        Error::OS(format!("Can't open file: {:?} ({})", args[3], e))
    })?;
    output_file.write_all(new_content.as_bytes())?;
    Ok(())
}



fn main() {
    match _main() {
        // TODO: return codes
        Ok(_) => println!("ok"),
        Err(Error::CSV(s)) => println!("CSV: {}", s),
        Err(Error::Input(s)) => println!("Input: {}", s),
        Err(Error::OS(s)) => println!("OS: {}", s),
        Err(Error::Io(e)) => println!("IO: {}", e),
    }
}
