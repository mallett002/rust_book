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

### Left off: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-string-type
- test
