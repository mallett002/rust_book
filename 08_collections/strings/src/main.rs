fn main() {
    // Create Strings -------------------------------------------------------------------------------
    let mut s = String::new();

    // with initial data
    let data = "initial contents";
    let s = data.to_string();
    let s = "hello".to_string();
    let s = String::from(data);

    // UTF-8 encoded (any language/symbols) -------------------------------------------------------------------------------
    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שלום");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // Grow a string -------------------------------------------------------------------------------
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    // can still use s2 (didn't move). push_str takes reference (&str)
    println!("{s2}");

    // add a single char
    let mut s = String::from("lo");
    s.push('l');

    // combining strings -------------------------------------------------------------------------------
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 moved here so can no longer be used
    // the "+" uses: fn add(self, s: &str) -> String
    // self means takes ownership (no ref). Rust coerces &String into &str for us
    // println!("{s1}"); // so, since moved, can't use here
    println!("{s3}");

    // concatenating multiple strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3; // pretty ugly
    let s = format!("{s1}-{s2}-{s3}"); // better. Also doesn't take any ownership

    // Indexing strings -------------------------------------------------------------------------------
    // String is wrapper over Vec<u8>
    let hello = String::from("Hola"); // len 4 (number of bytes it takes to store)
    let hello = String::from("Здравствуйте"); // len 24 (number of bytes takes to store). (each unicode scalar value takes 2 bytes)
    // let answer = &hello[0]; // can't do this in rust - would return 208 instead of "H"

    // Bytes, Scalar values, and Grapheme Clusters --------------------------------------------
    // “नमस्ते” (hindi word):
    //     “नमस्ते” stored like this:        [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    //     “नमस्ते” unicode scalar (char):   ['न', 'म', 'स', '्', 'त', 'े']
    //     “नमस्ते” grapheme cluster:        ["न", "म", "स्", "ते"]

    // Slicing strings ----------------------------------------------------------------
    println!("\nslicing strings:");
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");
    // let s = &hello[0..1]; // will panic (each character takes 2 bytes long) dangerous

    // Iterating over strings ---------------------------------------------------------
    println!("\nIterating over strings:");
    // use chars method
    for c in "Зд".chars() {
        println!("{c}");
    }

    // use bytes method to get the underlying bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
