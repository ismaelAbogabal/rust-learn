use std::{collections::HashMap, io::{self, BufWriter}, process::exit, time::SystemTime};

 
fn main() {
   let time = SystemTime::now();

   let index = 46;
   let mut cache = HashMap::new();
   let fib = fibonacci(index,  &mut cache);

    let elapsed = time.elapsed().unwrap();

    println!("Results of {index} is {fib} in {} ms", elapsed.as_millis());
}

fn fibonacci(index: u128, cache: &mut HashMap<u128, u128>) -> u128{
    
    if index == 0 {return 0;}
    if index == 1 { return 1;}

    if cache.contains_key(&index) {return cache[&index];} 

    let val = fibonacci(index - 1, cache) + fibonacci(index - 2, cache);

    // cache.insert(index, val);

    return val;
}

fn loop_for(){
    for i in (1..10).rev() {
        println!("The value of i is: {i}");
         
    }
}

fn sum() {

    println!("To end up the program enter 0");
    println!("Enter the first number");

    let mut total = 0;
    loop {
        let num = enterANumber();


        if num == 0 {
            println!("The total sum is: {total}");
            return;
        }

        total += num;
        println!("The total sum is: {total}. Add another number:")
        
    }
}

fn enterANumber() -> i32{
    println!("Enter a number");

    loop {
        let mut buffer = String::new();

         match io::stdin().read_line(&mut buffer) {
            Err(_) => {println!("Cant read your number try again"); continue;},
            Ok(_) => {}
        }
    
        match buffer.trim().parse::<i32>() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Invalid number entered. try again");
            }
            
        }
      }
}

fn numbers(){
    println!("Numbers in Rust");

    let a : u8 = 10;
    let b : i16 = a.into();


    let c = 10_000;
    let d = 0xff;
    let e = 0o44;
    let f = b'a'; // ASCII value of a is 97
    let g = b'0'; // ASCII value of 0 is 48

 
    let x = if a > 3 {10} else {11};


    let x = {
         4;
    };

    println!("a: {a} \nB: {b} \nC: {c} \nD: {d} \nE: {e} \nF: {f}\nG:{g}");


    // number_over_flow();


   tuple_types(); 
}


fn tuple_types() {
    let tup = (1, 2, 3);
     
     let (x, y , z) = tup;

    let (a, b , c) = (1, 2, 3);


    println!("x: {x} \nY: {y} \nZ: {z} \nA: {a} \nB: {b} \nC: {c}");
    }

fn number_over_flow(){
    let a : u8 = 255;
    let b = a + 1;
    println!("a: {a} \nB: {b} \n");
}
 

fn arrays(){
    let a = [1, 2, 3, 4, 5];
    
    
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Cant read your string");
 
   let index= match  buffer.trim().parse::<usize>() {
        Ok(num) => num,
        Err(e) => {
             println!( "Invalid number entered.");
            exit(1);
        }
    };

    println!("The value of the index is: {}", a[index])
}