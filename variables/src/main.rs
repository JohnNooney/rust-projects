const NUMBER_TWO: u32 = 1 + 1;

fn main() {
    println!("{NUMBER_TWO}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");


    let tup: (i32, f64, i8) = (500, 6.4, -8);
    let (_x, _y, z) = tup;
    println!("The value of z is: {z}");

    let six_point_four = tup.1;
    println!("The value of tup.1 is: {six_point_four}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("10th month: {0}", months[9]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("2nd number is: {0}", a[1]);

    another_function();
}

fn another_function() {
    println!("Another function!");
}