# stinpse
A basic command entry parser for personal use without any dependencies.

## Usage

```rust

    let parser = Parser::parse("ls -l");
    println!("{:?}", parser.get_args());
    // Output: ["ls", "-l"]

``` 