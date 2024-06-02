use std::io;

//static COMPARE: &str = "battery.charge:";
const TOKENS: &'static [&'static str] = &[
    "battery.charge:",
    "battery.runtime:",
    "battery.voltage:",
    "device.model:",
    "driver.version:",
    "driver.version.data:",
    "driver.version.internal:",
    "input.voltage:",
    "output.voltage:",
    "ups.test.result:",
    "ups.timer.shutdown:",
    "ups.timer.start:",
];

// process each line
fn process_line(line: &String) {
    for token in TOKENS {
        if line.contains(token) {
            let token_value = &line[token.len()+1..line.len()-1]; // extract value
            println!("matched:{}, got \"{}\"", line, token_value);
        }
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
