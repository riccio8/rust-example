fn main() {  
    // Ownership is transferred when a variable is assigned to another.
    let s1 = String::from("Hello, world!");

    // Ownership of s1 is moved to s2
    let s2 = s1;
  
    // At this point, s1 is no longer valid. Trying to use s1 will give an error
    // println!("{}", s1); // This line would cause a compile-time error: use of moved value: `s1`

    // But we can still use s2 because it now owns the String
    println!("{}", s2); // Outputs: Hello, worldd!

    // Ownership can also be borrowed, either immutably or mutably
    let s3 = String::from("Goodbye!");

    let s4 = &s3; // Immutable borrow
    println!("{}", s4); // Outputs: Goodbye!

    // If we tried to mutably borrow s3 here, it would be a problem:
    // let s5 = &mut s3; // Error! Cannot borrow as mutable because s4 is an immutable borrow.

    // When s3 goes out of scope, its memory is deallocated automatically, no need to manually free memory

  
    // This is because types like String are allocated in the heap, not in the stack
}
