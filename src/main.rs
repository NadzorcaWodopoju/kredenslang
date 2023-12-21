use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut input = stdin().lock();
    let in_buf = BufReader::new(&mut input);

    let mut a: u8 = 0;

    for line in in_buf.lines() {
        let line = line.unwrap();

        match line.as_str() {
            "sus" => println!("{}", a as char),
            "specjal" => a=a.wrapping_add(1),
            "skibiditoilet" => println!("Skibidi toilet goni mnie\nŚpiewając tę piosenkę\nPomaga mi cameraman\nLecimy ratować ziemię\nJesteśmy niepokonani"),
            "zgon" => a=0,
            "rzyg" => a=a.wrapping_sub(1),
            "uboot" => a=a.wrapping_add(10),
            _ => {}
        }
    }
}
