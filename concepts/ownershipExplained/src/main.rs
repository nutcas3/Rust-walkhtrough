// main function that owns s
fn main() {
    let mut s = String::from("Ownership");
    chenge(&mut s);
    println!("{}", s);
}


// function that borrows  variable s
fn chenge(some_string: &mut String) {
    some_string.push_str(", Example");
}