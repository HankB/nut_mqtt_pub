use std::io;

static COMPARE: &str = "battery.charge:";

// process each line
fn process_line(line: &String) {
    if line.contains(COMPARE) {
        println!("matched:{}", line);
    }
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
