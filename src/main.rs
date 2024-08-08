mod xorstr;

fn main() {
    let xorstr = xorstr!("hello world from earth");
    println!("{}", xorstr.as_str().unwrap());
}
