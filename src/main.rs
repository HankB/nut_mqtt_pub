use std::collections::HashMap;
use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

// fields from the output of 'upsc' that we want.
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
fn process_line(line: &String) -> (&str, &str) {
    for token in TOKENS {
        if line.contains(token) {
            let token_value = &line[token.len() + 1..line.len() - 1]; // extract value
            return (*token, token_value);
        }
    }

    ("", "") // return zero length strings for no match
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut matched_vals: HashMap<String, String> = HashMap::new();
    let time_t;

    // add a timestamp to the HashMap
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => time_t = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
    matched_vals.insert("t".to_string(), time_t.to_string());

    // loop over input lines
    loop {
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        let (tag, val) = process_line(&buffer);
        if tag.len() > 0 {
            let tag = tag.replace(".", "_"); //substitute '.'
            let tag = tag.replace(":", ""); // remove trailing ':'
            matched_vals.insert(tag.to_string(), val.to_string());
        }
        if r == 0 {
            break;
        }
    }

    println!("{:?}", matched_vals); // print results, conveniently as JSON
    Ok(())
}
