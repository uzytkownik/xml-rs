# libxml2 DOM bindings for Rust

Wrapper for libxml2 DOM bindings.

## Installation

To install the bindings please run `rustpkg install github.com/uzytkownik/xml-rs`.

## Simple example

~~~rust
let xml = "<?xml version=\"1.0\"?> <test />".as_bytes();
match xml::read_memory(xml) {
    Some(doc) => {
        println(doc.get_root_element().unwrap().name());
    }
    None => {
        println("Parse failed!");
    }
}
~~~

## TODO

Write the bindings - the bindings are at early stage so there is little that can be currently done.

## License

The project is licensed on MIT License.
