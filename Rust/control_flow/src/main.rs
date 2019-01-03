

fn main() {
    let number = 3;

    if (number < 5) {
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
