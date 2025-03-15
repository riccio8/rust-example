fn main() {
    let mut x = 5;

    // Mutable borrow (this is cool)
    let y = &mut x;

    // Trying to borrow mutably again (this is a no-go)
    // let z = &mut x; // This will throw an error

    *y += 1; // We tweak x through y
    println!("{}", x); // Outp: 6
}
