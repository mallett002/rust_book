# Smart Pointers

## Box<T>
- Allows you to store data on heap instead of stack
- stack points to heap data
- type who's size can't be known at compile time (ex: recursive struct like Cons list)
- large amt of data & you want to transfer ownership but ensure data isn't copied

## Rc<T> (reference counting)
- Allows you to have multiple owners of a value
- multiple pointers pointing to same piece of memory
- Will stay in scope until all owners have finished with it (out of scope)
- "Poeple watching TV" analogy

## RefCell<T>
- Allow you to mutate data in an immutable reference (interior mutability pattern)
- ex: src code has immutable reference, but for testing you need a mutable to track something
