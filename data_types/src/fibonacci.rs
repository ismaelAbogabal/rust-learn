fn fibonacci(index: u128) {
    
    if index == 0 {return 0;}
    if index == 1 { return 1;}

    return fibonacci(index - 1) + fibonacci(index - 2);
}