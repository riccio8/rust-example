use std::ops::Add; // for operator overloading

#[derive(Copy, Clone)]
#[derive(Debug)] // for pretty print
//struct
struct Point{
    x: i32,
    y: i32,
}

// enum
enum Expr{
    Null,
    Add(i32, i32),
    Sub(i32, i32),
    Mul(i32, i32),
    Div{dividend: i32, divisor: i32}, // using {} not () 
    Val(i32),
}



// function
fn print_point(point: Point){
    println!("x: {}, y: {}", point.x, point.y);  
}

//pattern matching
fn match_expr(expr: Expr){
    match expr {
        Expr::Null => println!("Null"),
        Expr::Add(x, y) => println!("Add: {} + {}", x, y),
        Expr::Sub(x, y) => println!("Sub: {} - {}", x, y),
        Expr::Mul(x, y) => println!("Mul: {} * {}", x, y),
        Expr::Div{ dividend: x, divisor: y} => println!("Div: {} / {}", x, y),
        Expr::Val(x) => println!("Val: {}", x),
        _ => println!("Unhandle expression / case"),
    }
}

fn uppercase(c: u8) -> char {
    match c {
        b'a'..b'z' => (c - 32) as char,
        _ => c as char, // as is used to cast between types
    }
}

fn is_alphanumeric(c: char) -> bool{
    match c{
        'a'..'z' | 'A'..'Z' | '0'..'9' => true,
        _ => false,
    }
}

fn uppercases(c :u8) -> u8{
    if let b'a'..b'z' = c{
        c - 32
    } else {
        c
    }
}

// macros
macro_rules! int_bitset{
    ($ty:ty) => {
        impl BitSet for $ty{
            fn clear(&mut self, index: usize){
                *self &= !(1 << index);
            }
            fn set(&mut self, index: usize){
                *self |= 1 << index;
            }
            fn is_set(&self, index: usize) -> bool{
                (*self >> index) & 1 == 1
            }
            fn toogle(&mut self, index: usize){
                *self ^=1 << index;
            }
        }  
    };
}

macro_rules! op{
    (+ $_self:ident : $self_type:ty, $other:ident $expr:expr) => {
        impl ::std::ops::Add for $self_type{
            type Output = $self_type;

            fn add($_self, $other: $self_type) -> {
                $expr
            }
        }
    };
}

#[warn(unused_mut)]
fn main() {
    let mut p1 = Point { x: 10, y: 20 };
    println!("{:#?}", p1); // pretty print
    let mut p2 = p1; 
    print_point(p1);
    println!("{}", p2.x); 
    inc_x(&mut p2);
    print_point(p2);

    /*
    without #[derive(Copy, Clone)]:

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; 

    ERROR, point doesn't implement copy trait and clone either
    */

    // tuple
    let tuple = (24, 42);
    let (hello, world) = "helloworld".split_at(5);

    println!("{} {}", tuple.0, tuple.1); 
    println!("{} {}", hello, world);

    //enum
    let quotient = Expr::Div{ dividend: 10, divisor: 2 };
    let sum = Expr::Add(40, 2);

    //traits
    let mut num = 0;
    num.set(5);
    println!("{}", num.is_set(5)); // true
    num.clear(5);


    //associated types
    let pp1 = Point{ x: 10, y: 20 };
    let pp2 = Point{ x: 30, y: 40 };
    let pp3 = pp1 + pp2; // operator overloading
    println!("{:#?}", pp3); // pretty print
    println!("{:#?}", pp1.dist_from_origin()); // pretty print

    //macros usage
    int_bitset!(i32);
    int_bitset!(u8);
    // int_bitset!(u64); ERROR: 67  |         impl BitSet for $ty{
    // |         ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u64`
    // that's because i declared that twice
}

//fn not implementes in type
fn inc_x(point: &mut Point){
    point.x += 1;
}

// methods / implementation
impl Point{
    fn dist_from_origin(&self) -> f64{
        let sum_of_squares = self.x.pow(2) + self.y.pow(2); // https://doc.rust-lang.org/stable/std/primitive.i32.html#method.pow
        (sum_of_squares as f64).sqrt()
    }

    fn translate(&mut self, dx: i32, dy: i32){
        self.x += dx;
        self.y += dy;
    }
}

impl Point{
    fn new(x: i32, y: i32) -> Self{ // used like: let point = Point::new(i32, i32);
        Self { x, y }
    }
}

impl Point{
    fn origin() -> Self{
        Point { x: 0, y: 0 }
    }
}

//traits
trait BitSet{
    fn clear(&mut self, index: usize);
    fn set(&mut self, index: usize);
    fn is_set(&self, index: usize) -> bool;

    fn toogle(&mut self, index: usize){
        if self.is_set(index){
            self.clear(index);
        } else {
            self.set(index);
        }
    }
}

impl BitSet for u64{
    fn clear(&mut self, index: usize){
        *self &= !(1 << index);
    }
    fn set(&mut self, index: usize){
        *self |= 1 << index;
    }
    fn is_set(&self, index: usize) -> bool{
        (*self >> index) & 1 == 1
    }
    fn toogle(&mut self, index: usize){
        *self ^=1 << index;
    }
} 

impl Add<Point> for Point{
    type Output = Point;

    fn add(self, point: Point) -> Self::Output{
        Point{
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

//Generics
fn max<T: PartialOrd>(a: T, b: T) -> T{
    if a > b{
        a
    }else{
        b
    }
}

//slice
fn first<T>(slice: &[T]) -> &T{
    if slice.is_empty(){
        panic!("slice is empty")
    }else{
        &slice[0]
    }
}

//for loops over a slice
fn index<T: PartialEq>(slice: &[T], target: &T) -> Option<usize>{
    for (index, element) in slice.iter().enumerate(){
        if element == target {
            return Some(index);
        }
    }
    None
}

fn min_max(slice: &[i32]) -> Option<(i32, i32)>{
    if slice.is_empty(){
        return None;
    }
    let mut min = slice[0];
    let mut max = slice[0];
    for &element in slice{
        if element < min{
            min = element;
        }
        if element > max{
            max = element;
        }
    }
    Some((min, max))
}

