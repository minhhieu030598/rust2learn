
fn main() {
    println!("My name is {}", "Hieu");
    println!("My friends are {0} {1}", "Hoang", "Van");
    println!("{subject} {verb} {object}", subject = "I", verb = "like", object = "meat");

    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    println!("{number:0>width$}", number=1, width=5);

}

