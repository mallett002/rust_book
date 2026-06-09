# Ownership

## heap vs stack
### stack
- has fixed amount of space
- push/pop stack (LIFO)
- faster than heap

### heap
- less organized than stack
- slower than stack
- goes and finds place in memory that has enough space to allocate data
- "allocating" is heap
- restaurant -> host finds place for your party with enough room -> seats you there
- pointer to allocation is stored in stack

### how works
- function called, args, local vars stored on stack (along with pointers to heap)
- when function execution ends, those values dropped (popped off stack, de-allocation)

## Ownership Rules
1. Every values has an owner
2. A value can only have 1 owner at a time
3. When owner goes out of scope, the value will be dropped

### ownership is a heap concept

### String type stores memory on the heap (unknown at compile time)

```rs
// Move: s1 moved into s2, and s1 will be dropped
let s1 = String::from("hello");
let s2 = s1; // "Move": makes s1 no longer valid (can't use it)
println!("{s1}"); // Error
```

```rs
// Variables no longer needed are dropped:
let mut s = String::from("hello"); // this will be dropped (de-allocated)
s = String::from("ahoy!"); // re-assigned "s"
println!("{s} world"):
```

```rs
// Can make deep copy if really need to:
let s1 = String::from("hello");
let s2 = s1.clone(); // deep copy (creates a new heap allocation for s2)
println!("s1: {s1}; s2: {s2}"); // works fine
// This is expensive, and a sign something not normal is occurring
```

```rs
// Copy trait - Doesn't allow moves
let x = 5; // doesn't move x to y (keeps them both)
let y = x;
println!("x: {x}; y: {y}"); // works fine

// many scalar types implement Copy trait: See `https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy`

// Cannot add Copy trait if type or any of its contents implement the Drop trait
```

### Left off here: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions





