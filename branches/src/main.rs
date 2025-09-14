fn main() {
    let number = 5;

    if number > 5 {
        println!("The number is greater than five");
    }
    else {
        println!("The number is less then 5");
    }

    let condition = true;
    let number_expression = if condition {5} else {6};
    println!("expression is {number_expression}.");
}
