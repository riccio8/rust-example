#  This is an example about how to create a random number generator given a seed using two methods

------
## There are two simple ways to generate "random" numbers:
- Linear congruental generators
- Xor Bit Operator
----
### Linear congruental:
- The expression is: Xₙ₊₁ = (a * Xₙ + c) % m where `a` is the seed
- Using ```n``` as any possible number
- first we need a simple math expression like ```(n + 5 + 8) % 9```
- we will use a recursive function that return that value and iterate over the returned value
```rust
fn main() {
	rand_rec(56, 6);
}

fn rand_rec(n: i32, times: i32){
	if times == 0{
		return;
	}
	
	let rec = (n + 5 + times as i32 *8) % 97;
	println!("{}", rec);
	rand_rec(rec, times - 1);
}
```
- But the period length (the time the program needs to repeat some numbers) is short, because of that we can replace 5 and 8 with any bigger number (same as 97)

### XorShift
- Instead of dealing with numbers, we'll have to deal with bits
```rust
fn main() {
   let a:i32 = 2;     
   let b:i32 = 4;     

   let mut result:i32;
   result = a ^ b;
   println!("(a ^ b) (xor shift) => {} ", result);
}
```
- Now, we need to shift bites before doing the xor bitwise operation:
```rust
fn main() {
    let x: i32 = 76587657;

    let shifted_left = x << 7;  
    let shifted_right = x >> 7; 
	let ored = x ^ shifted_left;
    println!("Original:      {:08b}", x);
    println!("Shift left 7:  {:08b}", shifted_left);
    println!("Shift right 7: {:08b}", shifted_right);
    println!("Xor over the shifted lef and x is: {:08b}", ored);
}

```
- So, what we're going to do is taking the original one and making the xor operation over using the left shifted bytes over and over again, so:
	- x = 100100100001010001010001001 ^ 1001000010100010100010010000000
- After that, we'll take the result and repeat again
```rust
fn main() {
    let x: i32 = 76587657;

    let shifted_left = x << 7;  
    let shifted_right = x >> 7; 
	let ored = x ^ shifted_left;
    println!("Original:      {:08b}", x);
    println!("Shift left 7:  {:08b}", shifted_left);
    println!("Shift right 7: {:08b}", shifted_right);
    println!("Xor over the shifted lef and x is: {:08b}", ored);
}
```
- So, all together:

```rust
fn main(){
	rand_xor(657);
}

fn rand_xor(seed: i64){
	let mut num_shift = 9;
	let result = seed ^ (seed << num_shift);
	println!("result: {result}");
	
	num_shift += 1;
	if num_shift >= 11{
		num_shift = 3;
	}
}
```

- Otherwise u can do something more realistic using the `Marsiglia` algorithm, something like that:
```rust
fn xorshift(mut x: u32) -> u32 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}

fn main(){
	println!("{}", xorshift(1));
}
```
