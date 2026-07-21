use std::collections::HashMap; // bring the whole hashmap into scope (idiomatic for structs/enums

use std::fmt;
use std::io;

use std::io::Result as IoResult; // use the "as" keyword to rename

fn main() {
    let mut map = HashMap::new();

    map.insert(1, 2);
}

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

fn function3() -> IoResult<()> {
    // --snip--
    Ok(())
}

// TODO: left off here: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use
