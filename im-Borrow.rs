fn main() {
    let x = 5;

    // Two immutable borrows (this is chill)
    let y = &x;
    let z = &x;

    println!("{}", y); // 5
    println!("{}", z); // 5

    // Trying to borrow mutably while you got immutable borrows, compiler won't like that
    // let w = &mut x; // This will throw an error
}
