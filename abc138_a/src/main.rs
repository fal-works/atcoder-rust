use std::io::*;
use std::str::FromStr;

fn parse_line<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn println<T: ToString>(s: T) {
    println!("{}", s.to_string());
}

// ------------------------------------------------------------------------

fn main() {
    let a = parse_line::<usize>();
  
    if a < 3200 {
        println("red");
    } else {
        let s = parse_line::<String>();
        println(s);
    }
}
