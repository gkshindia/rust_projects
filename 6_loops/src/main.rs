fn main() {
    //loop {
    //    println!("again!");
    //}
    return_from_loop();
    while_loop();
    loop_through_collection();
    for_loop();
    for_with_range();
}

fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_through_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_loop(){
    let a = [100, 220, 330, 450, 540];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_with_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}