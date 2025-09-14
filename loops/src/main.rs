fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 8 {
            break counter * 2;
        }
    };  

    println!("Returned value is {result}"); 

    counting_up(); 
    iterate_array_while();
    iterate_array_for();
    countdown();
}

fn counting_up()
{
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("Ending count = {count}");
}

fn iterate_array_while() {
    let list = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < list.len() {
        println!("the value is: {}", list[index]);

        index += 1;
    } 
}

fn iterate_array_for() {
    let a = [10, 20, 30, 40, 50];

    for number in a {
        println!("The number is: {number}");
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!");
}