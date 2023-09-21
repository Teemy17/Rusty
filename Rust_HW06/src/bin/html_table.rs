fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        panic!("Please provide three arguments: start_temperature, end_temperature, and interval");
    }

    let x: i32 = args[1].parse().unwrap();
    let y: i32 = args[2].parse().unwrap();
    let z: i32 = args[3].parse().unwrap();

    if z == 0{
        println!("The interval cannot be 0");
        return;
    }

    if x < y {
        println!("<html>");
        println!("<style>");
        println!("table, td {{");
        println!("   border: 1px solid #000000;\n   border-collapse: collapse;\n}}");
        println!("</style>\n");
        println!("   <body>");
        println!("       <table>");
        println!("           <tr>");
        println!("               <th>Fahrenheit</th>");
        println!("               <th>Celsius</th>");
        println!("           </tr>");

        let mut x1 = x;
        while x1 <= y {
            let cel = temptable::conv(x1);
            println!("           <tr>");
            println!("               <td>{}</td>", x1);
            println!("               <td>{:.1}</td>", cel);
            println!("           </tr>");
            x1 += z;
        }

        println!("       </table>");
        println!("   </body>");
        println!("</html>");
    } else {
        println!("   <html>");
        println!("   <head>");
        println!("       <title>Fahrenheit to Celsius Conversion</title>");
        println!("   </head>");
        println!("   <body>");
        println!("       <table>");
        println!("           <tr>");
        println!("               <th>Fahrenheit</th>");
        println!("               <th>Celsius</th>");
        println!("           </tr>");

        let mut x2 = x;
        while x2 >= y {
            let cel2 = temptable::conv(x2);
            println!("           <tr>");
            println!("               <td>{}</td>", x2);
            println!("               <td>{:.1}</td>", cel2);
            println!("           </tr>");
            x2 -= z;
        }

        println!("       </table>");
        println!("   </body>");
        println!("</html>");
    }
}

mod temptable {
    pub fn conv(fahrenheit: i32) -> f32 {
        (5. / 9.) * (fahrenheit as f32 - 32.)
    }
}
