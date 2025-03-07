use std::fs::File;
use std::io::Read;


fn main(){
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("!!ERROR: No file given!!");
    }

    println!("reading {:?}", args[1]);
    let f = File::open(args[1].clone());
    let mut content = vec![];
    let _ = f.expect("Error while reading file").read_to_end(&mut content);

    println!("{:?}", content);
}
