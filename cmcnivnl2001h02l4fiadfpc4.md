---
title: "Speed Up Rust Code Easily with Rayon"
datePublished: Thu Jul 03 2025 15:10:30 GMT+0000 (Coordinated Universal Time)
cuid: cmcnivnl2001h02l4fiadfpc4
slug: speed-up-rust-code-easily-with-rayon
cover: https://cdn.hashnode.com/res/hashnode/image/upload/v1751555344934/cbb70a4a-7a8b-42fd-bfd8-2497e1156608.jpeg

---

---

## Why Performance Is Hard

I wanted to speed things up in Rust and let's be honest, *threads are one of the best tools to improve performance in Rust*..

But using the Tokio crate can be quite ***unintuitive*** and **difficult** to use, all those await and features...

Here's where [rayon](https://docs.rs/rayon/latest/rayon/index.html) helps us, allows us to parallelize tasks without having to think about threads. It's simple to use, fast, lightweight, and just works.

---

### A simple definition

Rayon is a library that helps you run code in parallel, making it easy to turn slow, step-by-step computations into faster ones that use multiple CPU cores.

It's a small and easy-to-use tool that lets you add parallelism. It makes sure your code runs safely without data races, and it only uses parallelism when it makes sense, depending on the amount of work at runtime.

For example, we can simply turn this line:

```rust
let results: Vec<_> = data.iter().map(|x| x.do_something()).collect();
```

into:

```rust
use rayon::prelude::*;

let results: Vec<_> = data.par_iter().map(|x| x.do_something()).collect();
```

Using the [prelude](https://docs.rs/rayon/latest/rayon/prelude/index.html) is the easiest way to do parallelism using rayon in rust.

---

## Let's break down performance

### Without rayon

I ran the following code which iterates from 1 to 1,000,000, computes the cube (`x.pow(3)`) and the square (`x.pow(2)`) of each number, takes the remainder of both results using modulo `97,531`, then sums those two remainders.

I ran it using `cargo run` without any optimization:

* `Finished dev profile [unoptimized + debuginfo] target(s) in 0.86s`
    
* `Running target\debug\ry.exe`
    
* `2, 12, 36, 80, 150, 252, 392, 576, 810, 1100`
    

These are the CPU specs:

* CPU Name: Intel(R) microarchitecture code named Alderlake-S
    
* Frequency: 2.5 GHz
    
* Logical CPU Count: 12
    

```rust
fn main() {
    let data: Vec<u64> = (1..1_000_000).collect();
    let results: Vec<u64> = data.iter()
        .map(|x| x.pow(3) % 97_531 + x.pow(2) % 97_531)
        .collect();
    println!("{:?}", &results[..10]);
}
```

I measured performance using *Intel Vtune profiler* and we can see that without using rayon it needs 0.041s using 1 single thread

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1751554621303/a63bf8c8-dea1-4e6b-bc4f-fe829b14ec83.png align="left")

and the function which needs more time is the main, because we iterate, calculate and collect the result

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1751554626742/902f9d41-7766-4d93-8ef9-b5d4f2547d39.png align="left")

---

### With rayon

The computation level is the same as before, but this time we use rayon:

```rust
use rayon::prelude::*;

fn main() {
    let data: Vec<u64> = (1..1_000_000).collect();
    let results: Vec<u64> = data.par_iter()
        .map(|x| x.pow(3) % 97_531 + x.pow(2) % 97_531)
        .collect();
    println!("{:?}", &results[..10]);
}
```

adding `rayon = "1.10.0"` to your Cargo.toml dependencies

I compiled without optimizations:

* `Finished dev profile [unoptimized + debuginfo] target(s) in 0.02s`
    
* `Running target\debug\ry.exe`
    
* `[2, 12, 36, 80, 150, 252, 392, 576, 810, 1100]`
    

Already now we can see that the program ran in 0.02 seconds, compared to 0.86 seconds without Rayon, but let's see in detail:

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1751554640610/e7a8827b-275c-4f3d-bdbb-8ae50d041b68.png align="left")

* First, we can see it uses 8 threads instead of just one
    
* We see that it took 0.029 seconds instead of 0.041s
    
* CPU status is constantly in idle mode instead of poor as before
    

As before all the effective time is used by one single function which is the last called

![](https://cdn.hashnode.com/res/hashnode/image/upload/v1751554746676/05d593a4-0be2-4b63-8515-89f6274be6e5.png align="left")

---

## When (and When Not) to Use Rayon

The ideal use cases are *CPU-bound work, large datasets, pure functions, sorting, etc.*

Instead for *small workloads, shared mutable state or I/O-heavy tasks* it's better to use the [Tokio](https://docs.rs/tokio/latest/tokio/) runtime if you really need it. The [Tokio module](https://docs.rs/tokio/latest/tokio/#modules) supports `fs, time, command execution, net` and a lot more using multithreading, but that's another topic I'll write about...

---

### Other stuff Rayon does

Beyond `.map` and `.par_iter` Rayon also includes `.filter()`, `.reduce()`, `.for_each()`, `join()` for parallel sorting

---

## To sum up

`Rayon` isn't always the best choice. Still, it's a **smart and safe way** to add parallelism. It helps you **scale workloads with minimal code changes**, making it a solid choice for performance-critical applications.

`💡 Got another crate in mind?`

## ☕ Was this helpful?

Treat me to a coffee on Ko-fi [https://ko-fi.com/riccardoadami](https://ko-fi.com/riccardoadami)