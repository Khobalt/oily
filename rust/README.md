# Rusty Notes

1. Immutability is a thing in Rust.
1. Bits to remember:
    1. cargo new
    1. cargo build
    1. cargo run
    1. Rust complains if you assign a variable and then immediately overwrite it. In my situation, just declaring a variable was better.
    1. Rust is pretty happy about it's immutable by default paradigm.
        1. It's kind of a Java/C hybrid in terms of its basic types. Even addresses of (big O) Objects are imutable, so when you pass by reference into a function you need to specify that the address& is mut, if you want in-indempoetent functions.
    1. Rust is fairly opinionated about snake case. In fact, they don't really enforce style but they sure do remind you to be fashionable, which I'm ok with.
    1. You type-casting works by calling out 'as'.
    1. for loops are going to ask for an iterator, which you can run through backwards by calling .rev() on them.
1. Things I did:
    1. Went to the Rust website and installed via a shell script known as 'rustup'
    1. Installed all the vs code extensions that looked handy, including the language server
    1. i64s are handy for bigger numbers
