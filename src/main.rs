
fn main() {

    hello();
    mutibility_print();
    check_loop();
    if_statement();
    data_print();
}
// HELLO WORLD PRINT!
fn hello() {
    println!("Hello world");
}

fn mutibility_print(){
    let mut x = 50;
    println!("x is {}", x);

    x = 20;
    println!("x new is {}", x);
}
fn data_print(){
    let x = 50; // i32
    let y:i64 = 20; // i64
    let z:bool = false;
    println!("x is {}", x);
    println!("y is {}", y);
    println!("z is {}", z);
}

fn if_statement(){
    let x = 51; 
    if x < 50{
        println!("value is less then 50");
    }else if x > 50{
        println!("value is greater then 50");
    }
}
// LOOP TEST
fn check_loop() {
    let mut x = 1;
    loop {
        x+=1;
        if x == 3{
            break;
        }
        println!("x = {}", x);
    }
}
    