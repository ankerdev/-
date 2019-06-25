use super::functions::create_heading;

pub fn run() {
    let string = "Jonas";
    let number = 22;
    let boolean = true;

    println!("{}", create_heading("Print"));

    println!("Indexed arguments: {0}, {1}, {2}", string, number, boolean);

    println!(
        "Named arguments: {zero}, {one}, {two}",
        zero = string,
        one = number,
        two = boolean
    );

    println!("Debug: {:?}", (string, number, boolean));
}
