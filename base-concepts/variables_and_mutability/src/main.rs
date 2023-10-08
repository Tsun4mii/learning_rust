fn main() {

    //VARIABLES

    let mut x = 23;         //to change value add "mut" keyword after "let"
    println!("{x}");

    // x = 3;  //error if x is immutable
    x = 3;     // if x declared with "mut" this will now work
    println!("{x}");

    //CONSTANTS 

    const BASE_NUMBER: u32 = 2 * 3 * 6;
    const BASE_STRING: &str = "Base"; //String type is wrong. String is growable buffer and it`s purpose to be modifieble

    println!("Number const: {BASE_NUMBER}");
    println!("String const: {BASE_STRING}");

    let y = 32;

    let y = y + 2;

    {
        let y = y * 5;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
