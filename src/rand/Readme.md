#  This is an example about how to create a random number generator given a seed using 2 methods

------
## There are two simple ways to generate "random" numbers:
- Linear congruental generators
- Xor Bit Operator
----
### Linear congruental:
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
