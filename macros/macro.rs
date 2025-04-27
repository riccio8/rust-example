macro_rules! hi {
    () => {
        println!("hi");
    };
}

macro_rules! do_math {
    (add $a:expr, $b:expr) => {
        println!("{} + {} = {}", $a, $b, $a + $b);
    };

    (mul $mul:expr, $bho:expr) => {
        println!("{} * {} = {}", $mul, $bho, $mul * $bho);
    };
}

macro_rules! add_numbers {
    ($($num:expr), *) => {
        {
            let mut total = 0;
            $(
                total += $num;
            )*
            println!("{}", total);
        }
    };
}


macro_rules! log {
    ($message:expr) => {
        println!("[LOG]: {}", $message);
    };
}


macro_rules! var_type{
    ($valore:ty) => {
        println!("Var type: {:?}", std::any::type_name::<$valore>());
    };
}

fn main() {
    hi!();
    log!("Program started");
    do_math!(add 34, 8);
    do_math!(mul 7976987, 8);
    add_numbers!(2, 4, 5, 5, 5, 3, 3);
    var_type!(i32);
    var_type!(String);
}
