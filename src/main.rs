use include_fields_macro::include_fields;

fn main() {
    let person = Person {
        name: "test".into(),
        age: 43,
    };

    println!("{person:#?}");
}

// Macro limitation regarding paths: https://github.com/rust-lang/rust/issues/54725
// Attribute order matters!
#[include_fields(path = "src/common.fields")]
#[derive(Debug)]
struct Person {
    name: String,
}
