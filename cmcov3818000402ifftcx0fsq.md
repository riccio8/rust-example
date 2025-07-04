---
title: "Rust's regex under the hood..."
seoTitle: "Deep Dive into Rust's Regex Engine"
seoDescription: "Explore how Rust's regex engine ensures safety and speed, tips for writing efficient patterns, and using RegexSet for multiple matches"
datePublished: Fri Jul 04 2025 13:40:04 GMT+0000 (Coordinated Universal Time)
cuid: cmcov3818000402ifftcx0fsq
slug: rusts-regex-under-the-hood
cover: https://cdn.hashnode.com/res/hashnode/image/upload/v1751636085915/3f8f6e61-c494-4572-afb7-0a46cfc8f798.jpeg

---

## Why should you use regex

A regular expression (regex) is a sequence of symbols that identifies a set of strings, numbers, or other characters. They are used for pattern matching and for verifying data

---

Regular expressions are powerful — and dangerous. In most languages, they can lead to unpredictable performance, surprising behavior, or catastrophic backtracking

Rust's `regex` crate is different. It's designed with safety and speed in mind. But how? What makes it so fast? And how should you write your regex patterns to stay in the fast lane?

In this post, we'll dig into how Rust handles regular expressions under the hood, how to write efficient patterns, and when things can go wrong (yes, even in Rust).

---

### Key design choices:

* No backtracking engine
    
* Compiled to finite automata (NFA/DFA)
    
* Safe by default: patterns that could cause catastrophic backtracking are **impossible**
    

📚 Docs: [regex](https://docs.rs/regex/latest/regex/)

---

## Regex Compilation and Execution Model in Rust

Rust transforms your regex into a sequence of optimized search structures.

### Compilation pipeline:

1. **Parse** the pattern into an AST
    
2. Convert it into a **Thompson NFA** (efficient, small-memory execution model)
    
3. Optionally compile into a **lazy DFA** at runtime (fast, but memory-hungry)
    

The engine dynamically decides which to use depending on the input and pattern.

### Lazy DFA:

* Built incrementally as needed
    
* Extremely fast matching (no backtracking)
    
* May allocate more memory if input is complex
    

Theory reference: [Regex crate internals – GitHub](https://github.com/rust-lang/regex/blob/master/README.md#implementation)

---

## Writing Efficient Regex Patterns

Even with a smart engine, **writing good regex matters**.

### Best practices:

* Prefer **explicit character classes** over overly broad wildcards:
    
    * Good: `[a-zA-Z0-9_]`
        
    * Avoid: `.*` or `.+`
        
* Use **non-greedy quantifiers** when needed (`*?`, `+?`)
    
* Avoid ambiguous alternations: `foo|foobar` is slower than `foobar|foo`
    

### Things to avoid:

* Nested quantifiers: `.*.*`
    
* Redundant groups: `((abc))`
    
* Expecting look-around/backreferences (unsupported)
    

---

## RegexSet: When You Have Many Patterns

[`RegexSet`](https://docs.rs/regex/latest/regex/struct.RegexSet.html) allows you to **match against many regexes simultaneously**.

Instead of testing one-by-one, `RegexSet` compiles all regexes into a **single DFA**.

### Example use cases:

* Firewall filters
    
* Log file scanners
    
* Search engines
    

### Advantages:

* Fast matching on large pattern sets
    
* All matches evaluated in a single pass
    
* Avoids allocation and recompilation
    

### Sample

```rust
use regex::RegexSet;

fn main(){
	let set = RegexSet::new([
	    r"foo",
	    r"bar",
	    r"baz",
	]).unwrap();

	let matches = set.matches("bar");

	assert!(matches.matched(1)); // bar is at index 1
}
```

---

## Regex Overhead: Compile-Time vs Runtime

Calling `Regex::new()` compiles the regex pattern at **runtime** — this is fast, but not free.

### Costs:

* Compilation allocates memory and builds the NFA/DFA
    
* Doing this inside a loop or hot path is a **performance killer**
    

### Best practices:

* Use [`lazy_static`](https://docs.rs/lazy_static/) or [`once_cell`](https://docs.rs/once_cell/) to compile once and reuse:
    

```rust
use regex::Regex;
use once_cell::sync::Lazy;

static RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
});
```

---

## Grouping and Captures in Rust Regex

Rust's regex engine supports **capturing groups** and **non-capturing groups**, but **not backreferences**.

---

### Basic Capturing Groups

Capturing groups let you extract parts of a match.

```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
let caps = re.captures("2025-07-04").unwrap();

assert_eq!(&caps[0], "2025-07-04"); // full match
assert_eq!(&caps[1], "2025");       // first group
assert_eq!(&caps[2], "07");         // second group
assert_eq!(&caps[3], "04");         // third group
```

You can also name your groups:

```rust
let re = Regex::new(r"(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
let caps = re.captures("2025-07-04").unwrap();

assert_eq!(&caps["year"], "2025");
assert_eq!(&caps["month"], "07");
assert_eq!(&caps["day"], "04");
```

Docs: [Regex::captures](https://docs.rs/regex/latest/regex/struct.Regex.html#method.captures)

### Non-Capturing Groups

Non-capturing groups don't save the match — they’re for grouping logic only:

```rust
let re = Regex::new(r"(?:cat|dog)s?").unwrap();
assert!(re.is_match("cats"));
```

Use `(?:...)` instead of `(...)` when you don't need to capture — this can **improve performance** slightly.

---

### No Backreferences, No Recursion

Rust’s regex engine **does not support**:

* Backreferences (like `\1`, `\k<name>`)
    
* Recursion (`(?R)`)
    
* Lookahead/lookbehind assertions
    

Why?

> Because the engine avoids features that **require backtracking**, to guarantee **linear-time execution**.

Example (NOT supported):

```rust
(.*?)\1     # invalid: backreference not allowed
```

---

## Using case

In one of my latest projects, I needed to send a request to an apache http server , which returned the HTTP response headers using [curl -I](https://curl.se/docs/manpage.html#-I).

An example of the returned information:

HTTP/1.1 200 OK

Date: Mon, 21 Apr 2025 09:47:53 GMT

Server: Apache/2.4.37 (AlmaLinux) OpenSSL/1.1.1k mod\_auth\_gssapi/1.6.1

Last-Modified: Fri, 07 Jan 2022 03:05:20 GMT

---

My goal was to find the version, such as `Apache/2.4.37`, so I wrote this regex:

```rust
r”(?P<server_str>Server:\s*)(?P<version>[A-Z]?[a-z]*/\d+\.\d+(?:\d+)?)\s(?P<infos>.*)"gm
```

Let's break it down:

It matches the Server: HTTP header and extracts:

* server\_str - the literal prefix "Server: "
    
* version - the first server name/version, like Apache/2.4.37
    
* infos - everything else after that (OS, modules, etc.)
    

---

### Quick part-by-part:

| Pattern | Captures |
| --- | --- |
| `(?P<server_str>Server:\s*)` | `"Server: "` |
| `(?P<version>[A-Z]?[a-z]*/\d+\.\d+(?:\.\d+)?)` | `Apache/2.4.37`, `nginx/1.23`, etc. |
| `\s` | A space separating version/info |
| `(?P<infos>.*)` | The rest: `(AlmaLinux) OpenSSL...` |

So it will return 3 named groups:

* `server_str`: `"Server: "`
    
* `version`: `"Apache/2.4.37"`
    
* `infos`: `"(AlmaLinux) OpenSSL/1.1.1k mod_auth_gssapi/1.6.1"`
    

---

### Practicing

If you want to practice [regex101](https://regex101.com/) is really useful, you can see time, performance, test unit make it for various languages and a lot more...

---

### Definitions

Catastrophic backtracking happens when certain regex patterns cause exponential time complexity due to nested quantifiers — Rust avoids this by design.

---

`💡 any particular request?`

---

## ☕ Was this helpful?

Treat me to a coffee on [Ko-fi](https://ko-fi.com/riccardoadami) [https://ko-fi.com/riccardoadami](https://ko-fi.com/riccardoadami)