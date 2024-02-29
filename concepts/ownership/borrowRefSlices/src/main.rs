fn main() {
    let mut value = String::from("Borrow");

    // Immutable borrowing (references)
    let reference1 = &value;
    let reference2 = &value;

    println!("Immutable references: {}, {}", reference1, reference2);

    // Mutable borrowing (mutable reference)
    let mutable_reference = &mut value;
    mutable_reference.push_str(", world!");

    println!("Mutable reference: {}", mutable_reference);

    // Slices (borrowing a part of a collection)
    let slice = &value[0..5];
    println!("Slice: {}", slice);
}