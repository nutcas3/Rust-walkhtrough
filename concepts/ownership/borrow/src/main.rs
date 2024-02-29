fn main() {
    let mut value = 8;

    // this is immutable borrowing
    let reference1 = &value;
    let reference2 = &value;

    println!("Immutable references: {}, {}", reference1, reference2);

    // mutable borrowing
    let mutable_reference = &mut value;
    *mutable_reference += 1;

    println!("mutable reference: {}", mutable_reference);
}
