use std::io;

fn main() {
    loop {
        println!("Enter the number times to iterate");
        let mut numbr = String::new();

        io::stdin()
            .read_line(&mut numbr)
            .expect("Failed to read line");

        let numbr: u32 = match numbr.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("provide a number");
                continue;
            }
        };
        println!("");
        println!("Iteration output : ");
        println!("");
        number_iterate(numbr);
        break;
    }
}
fn number_iterate(count: u32)  {
    for counter in 1..count+1{
        for _counter2 in 1..counter+1{
            print!("{counter}");
        }
        println!("");
    }
}