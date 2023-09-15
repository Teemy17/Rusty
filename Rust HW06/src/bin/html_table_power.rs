fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("Please provide three arguments: start_number, end_number, and interval");
    }

    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();
    let z: i32 = args[3].parse().unwrap();
    
    if z == 0{
        println!("The interval cannot be 0");
        return;
    }

    println!("<html>");
    println!("<style>");
    println!("table, td {{");
    println!("   border: 1px solid #000000;\n   border-collapse: collapse;\n}}");
    println!("</style>\n");
    println!("   <body>");
    println!("       <table>");
    println!("           <tr>");
    println!("               <th>x</th>");
    println!("               <th>x^2</th>");
    println!("               <th>x^3</th>");
    println!("           </tr>");
    
    if x <= y {
        let mut x = x;
        while x <= y {
            let square = x * x;
            let cube = x * x * x;

            println!("           <tr>");
            println!("               <td>{}</td>", x);
            println!("               <td>{}</td>", square);
            println!("               <td>{}</td>", cube);
            println!("           </tr>");

            x += z;
        }
    }

    println!("       </table>");
    println!("   </body>");
    println!("</html>");
}
