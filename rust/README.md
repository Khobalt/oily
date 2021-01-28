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
    1. The last expression in a bloc is implicitly returned, you'll be tipped off by a lack of semi-colon.
        1. In fact, if-else blocs are expressions so they can be used as assignments if the last line is an expression.
    1. You can return tuples and destructure them, which is nice.
1. Things I did:
    1. Went to the Rust website and installed via a shell script known as 'rustup'
    1. Installed all the vs code extensions that looked handy, including the language server
    1. i64s are handy for bigger numbers

## Ownership

Since there's no garbage collector, and dereferencing seems a bit less explicit, Rust has another paradigm altogether. The materials that I'm reading more or less claim that "Ownership" is the center-piece of Rust. Once variables go out of scope theres an implicit `drop` called on them. There's no such thing as a shallow copy on variables that are on the heap. So if you do what looks like one in Java or C++ you're actually doing a move, ie. the old variable is no longer valid.

Transfering ownership also extends into functions, where again there are implicit drops at the end. I'm not quite sure what shenanigans are going to propagate from this design.

References are the loophole here...

``` Rust
//Here's an example from the official book
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Referencing and dereferencing seem similar to C/C++, ie & and *

To navigate the whole "ownership" construct, passing work regarding collections such as strings into functions relies on slices. A string slice looks like `&str`.
