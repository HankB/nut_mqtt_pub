use std::collections::HashMap;
use std::io;

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

//static mut MATCHED_VALS: HashMap<String, String> = HashMap::new();

// process each line
fn process_line(line: &String) -> (&str, &str) {
    for token in TOKENS {
        if line.contains(token) {
            let token_value = &line[token.len() + 1..line.len() - 1]; // extract value
            //println!("matched:{}, got \"{}\"", line, token_value);
            return (*token, token_value)
        }
    }
    ("","")
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut matched_vals: HashMap<String, String> = HashMap::new();
    loop {
        // read input a line at a time
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        let (tag, val) = process_line(&buffer);
        if tag.len() > 0 {
            matched_vals.insert(tag.to_string(), val.to_string());
        }
        if r == 0 {
            break;
        }
    }
    println!("{:?}", matched_vals);
    Ok(())
}
