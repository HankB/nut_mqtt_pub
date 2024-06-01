use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        let r = io::stdin().read_line(&mut buffer)?;
        println!("{}", buffer);
        if r == 0 {
            break;
        }
    }
    Ok(())
}
