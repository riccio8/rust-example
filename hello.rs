fn primis(n: i32) -> Option<i32>{
    let mut v: Option<i32> = None;
    if n <= 1 {
        return v; 
    }
    for a in 2..n {
        if n % a == 0 {
            return v;
        }
    }
    v = Some(n);
    v
}


fn main(){
    let mut v: Vec<i32> = Vec::new();

    for i in 1..100000 {
        match primis(i){
            Some(prime) => v.push(prime),
            None => {},
        }
    }
    println!("{:?}\n", v);
}   
