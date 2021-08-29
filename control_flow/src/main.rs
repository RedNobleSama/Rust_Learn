fn main() {
    if_else_fn();

    loop_fn();

    while_fn();

    for_fn();
}


fn if_else_fn() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is disvisble by 4")
    } else if number %  3 == 0  {
        println!("number is disvisble by 3")
    } else if number %  2 == 0  {
        println!("number is disvisble by 2")
    } else {
        println!("number is disvisble by 1")
    }
}

// fn match_fn() {
//     let number = 6;
//     match number % { }
// }

fn loop_fn() {
    let mut con = 1;
    let result = loop {
        con += 1;
        if con == 10 {
            break 50
        };
       println!("hello");
    };
    println!("the result is {}", result)
}

fn while_fn () {
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    };
    println!("lifioff");

}

fn for_fn () {
    let a = [10, 20 ,30, 40, 50];
    let mut index = 0;
    for element in a.iter() {
        println!("the value is {}", element)
    }
}