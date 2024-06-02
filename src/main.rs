use std::io;

// process each line
fn process_line(line: &String) {
    println!("{}", line);
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        // read input a line at a time
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        process_line(&buffer);
        if r == 0 {
            break;
        }
    }
    Ok(())
}
