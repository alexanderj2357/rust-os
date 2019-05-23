// FIXME: Make me compile! Diff budget: 2 lines.

// Do not change this definition.
enum MyEnum {
    A(String),
    B(String)
}

fn matcher(val: &MyEnum) -> &str {
    match *val {
        MyEnum::A(string1) => string1.as_str(),
        MyEnum::B(string2) => string2.as_str()
    }
}

fn main() { }
