fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("No Fahr provided")
    }
    
    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();
    let z: i32 = args[3].parse().unwrap();
   
    if x < y { 
        println!("Fahr\tCelcius");
        let mut x1 = x;
        while x1 <= y {
            let cel = temptable::conv(x1);
            println!{"{:>4}\t{:>4.1}",x1,cel}
            x1 += z;
        }
    }
    else {
        println!("Fahr\tCelcius"); 
        let mut x2 = x;
        while x2 >= y {
            let cel2 = temptable::conv(x2);
            println!{"{:>4}\t{:>4.1}",x2,cel2}
            x2 -= z;
        }
    }    


}   
          