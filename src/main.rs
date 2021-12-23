// enum Direction{
//     Up,
//     Down,
//     Left,
//     Right
// }

struct Color {
    red : u8,
    green : u8,
    blue : u8,
}

// struct Color (u8,u8,u8);


fn main() {

    // hello();
    
    // mutibility_print();
    
    // check_loop();
    
    // if_statement();
    
    // data_print();
    
    // check_loop();
    
    // while_loop();
    
    // test_enum();
    
    // tuple_test();
    // tuple_test_two();
    
    
    // func_test(10);
    // count(8);
    
    // block();

    // shadowing();

    // reference();

    // struct_test();

    // struct_tuple();

    // let blue = Color{red :0, blue : 0, green:0};

    // print_color(&blue);
    // print_color(blue);


    // let num : [i32; 5] = [1,2,3,4,5];
    let num = [2; 200];
    // for n in num.iter() {
    //     println!("{}",n);
    // }

    for i in 0..num.len() {
        println!("{}", num[i]);
    }
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
    
// fn test_enum(){
//     let mut player_direction:Direction = Direction::Up;

//     match player_direction{
//         Direction::Up => println!("player direction is up"),
//         Direction::Down => println!("player direction is down"),
//         Direction::Left => println!("player direction is left"),
//         Direction::Right => println!("player direction is right"),
//     }
//     // player_direction:Direction = Direction::Right;

//     // match player_direction{
//     //     Direction::Right => println!("player direction is right"),
//     // }
// }

// fn tuple_test(){
//     let tup = (12,"jhon",53,false, (1,2,3));
//     // println!("tuple  {}", tup.3);
//     println!("tuple {}", (tup.4).1);
// }

// fn tuple_test_two(){
//     let tup = (12,"jhon",53);
//     let (a,b,c) = tup;
//     println!("a {}",a);
//     println!("b {}",b);
//     println!("c {}",c);
// }

// fn count(n : u32){
//     for n in 1..n{
//         if func_test(n){
//             println!("count is even {}",n);
//         }else{
//             println!("count is odd {}",n);
//         }
//     }
// }

// fn func_test(n : u32) -> bool{
//     return n % 2 == 0;
// }

//          Code block

// fn block(){
//     let x =10;
//     {
//         let y = 20;
//         println!("x:{}, y:{}",x,y); // not error
//     }
//     // println!("x:{}, y:{}",x,y); //error

// }

// fn shadowing(){
//     let mut x = 10;
//     {
//         let x = 20; //wont work
//     }
//     println!("x : {}",x); // shows 10;

//     let x = "this is x string ";
//     println!("x : {}",x); // string 

//     let x = true;
//     println!("x : {}",x); // boolean 

// }

// fn reference(){

//     // { // will not work
//         let mut x = 10;
//         let xr = &mut x;
//     // }       
//     *xr +=1;
//     println!("xr : {}",xr);
// }

// fn struct_test(){
//     let bg = Color{red:26,green:44,blue:20};
//     println!("{},{},{}",bg.green, bg.blue, bg.red);
// }

// fn struct_tuple(){
//     let mut red = Color(1,2,3);
//     red.2 = 10;
//     println!("{},{},{}", red.0,red.1,red.2);
// }

// fn print_color(c: &Color){
//     println!("{}", c.blue);
// }