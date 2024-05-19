use std::net::Ipv4Addr;

enum IpAddress {
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    let home = IpAddress::V4(127,0,0,1);
    let loopback = IpAddress::V6(String::from("::1"));
    

    // for i in [home, loopback] {

    //     match  i { 
    //         IpAddress::V4(a,b,c,d) => println!("V4 {} {} {} {}",a,b,c,d),
    //         IpAddress::V6(s) => println!("V6 {}",s),
            
    //     }
    // }


    if  let IpAddress::V4(_ , _ ,_ ,_) = home {
        println!("Home is V4");
    }
     
}
