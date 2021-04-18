use std::io;
use std::str::FromStr;

fn main() {
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Can't read the line!");
    let f = f64::from_str(f.trim()).expect("Can't get f64 from read line");
    println!("{:.2}ºF = {:.2}ºC", f, f_to_c(f));
}

fn f_to_c(f: f64) -> f64 {
    5.0 / 9.0 * (f - 32.0)
}
