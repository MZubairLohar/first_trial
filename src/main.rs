enum Direction{
    Up,
    Down,
    Left,
    Right
}

fn main() {

    // hello();
    // mutibility_print();
    // check_loop();
    // if_statement();
    // data_print();
    // check_loop();
    // while_loop();
    test_enum();
}
// HELLO WORLD PRINT!
// fn hello() {
//     println!("Hello world");
// }

// fn mutibility_print(){
//     let mut x = 50;
//     println!("x is {}", x);

//     x = 20;
//     println!("x new is {}", x);
// }
// fn data_print(){
//     let x = 50; // i32
//     let y:i64 = 20; // i64
//     let z:bool = false;
//     println!("x is {}", x);
//     println!("y is {}", y);
//     println!("z is {}", z);
// }

// fn if_statement(){
//     let x = 51; 
//     if x < 50{
//         println!("value is less then 50");
//     }else if x > 50{
//         println!("value is greater then 50");
//     }
// }
// // LOOP TEST
// fn check_loop() {
//     let mut x = 1;
//     loop {
//         x+=1;
//         if x == 3{
//             break;
//         }
//         println!("x = {}", x);
//     }
// }

// fn while_loop (){
//     let mut x = 1;
//     while x <= 20{
//         if x % 5 ==0 {
//             println!("x = {}", x);
//         }
//         x+=1;
//     }
// }
    
fn test_enum(){
    let mut player_direction:Direction = Direction::Up;

    match player_direction{
        Direction::Up => println!("player direction is up"),
        Direction::Down => println!("player direction is down"),
        Direction::Left => println!("player direction is left"),
        Direction::Right => println!("player direction is right"),
    }
    // player_direction:Direction = Direction::Right;

    // match player_direction{
    //     Direction::Right => println!("player direction is right"),
    // }
}