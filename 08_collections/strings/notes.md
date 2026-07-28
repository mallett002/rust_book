- Only string type in language is `str` slice. It's a ref to UTF-8 string data stored elsewhere.
    - `&str` is like a sticky note to a locker number (memory address), plus a length (how many characters you can read) - AKA, a fat pointer
- `String` is implemented in Rust's standard library. Growable, mutatable, UTF-8 encoded string type (heap)
    - Like full access to a locker. You can put things in, take them out (adjustable size - heap)
    - wrapper around Vector of bytes (with some extra stuff)

