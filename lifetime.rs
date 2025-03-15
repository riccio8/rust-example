fn main() {
    let s1 = String::from("Hey");
    let s2;

    {
        let s3 = String::from("Yo");
        s2 = &s3; // Uh-oh! s3's gonna drop out of scope before s2 gets used
    }

    // println!("{}", s2); // Rust says "nope" here, because s2 points to invalid memory now
}
