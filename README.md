# politeness-macro
[![crates.io](https://img.shields.io/crates/v/politeness-macro?style=flat)](https://crates.io/crates/politeness-macro)  

Aren't we all too rude to computers?
Isn't it time to bring a bit more politeness into our programming?
Shouldn't we be a bit nicer to rustc?  

Well, now we can be! Introducing `polite!`, the macro that makes you say please.  

Using `polite!` is simple, simply put valid Rust code into a polite block and watch it not compile because it's too rude.
To get your code to compile again, just say please! Inserting `please` into your code will get rustc to compile it again,
but don't do it too much or it's gonna think you're too polite!

## Why?
I'm the same person that did [this](https://twitter.com/lostkagamine/status/1501565876054679562). What do you mean why.

## Code example
```rs
use politeness_macro::polite;

fn main() {
    polite! {
        // println!("hello, world"); // DOES NOT COMPILE

        please println!("hello, world"); // does compile!
    }
}
```