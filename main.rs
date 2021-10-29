// use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    // set file name
    let in_file = "tasks.org";
    let out_file = "test.org";

    // get the contents out of the file
    let tasks_to_add = fs::read_to_string(in_file).expect("Something went wrong reading the file");

    // get the contents out of the file
    let mut contents = fs::read_to_string(out_file).expect("Something went wrong reading the file");

    // find where to add the tasks
    if contents.contains("** Every Day") {
        println!("file has spot to put tasks");
    }

    // find wher to add the tasks
    let index = contents.find("** Every Day");
    println!("{:?}", index);

    for i in 0..13 {
        let b: u8 = contents.as_bytes()[1441 + i];
        let c: char = b as char; // if you need to get the character as a unicode code point
        println!("{:?}", c);
    }

    // insert the tarsks in to the string
    contents.insert_str(1441 + 13, &tasks_to_add);

    // write to file
    let mut out_fh = File::create(out_file)?;
    write!(out_fh, "{}\n", contents)?;

    Ok(())
}
