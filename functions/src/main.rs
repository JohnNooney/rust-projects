fn main() {
    another_function(5);
    print_labeled_measurements(5,'h');

    let y = five();
    let six = six_fail(y);

    println!("The value of y is: {y}");
    println!("The valeu of six is: {six}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

// throws compile error because 
// statement instead of expression ie:( ';' is used here)
fn six_fail(x: i32) -> i32 {
    x + 1;
}