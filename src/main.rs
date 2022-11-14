

///@title This program solves for the modular multiplicative inverse of a number A under Mod B using Euclidean Algorithm
///@author Jelo
/// The multiplicative inverse of an integer A is the number x such that Ax ≅ 1 (mod B).
/// Put in another way, the multiplicative inverse of an integer A is the number x, 
/// such that the remainder gotten after dividing Ax by B is 1.
/// Example: If A = 3, and B = 5. Substituting into the equation above, we can see that
/// the value of X should be 2. Since (3 * 2) / 5 = 1 remainder 1, which satisfies the definition of a multiplicative inverse.  
/// 
/// The modular mult. inverse can be calculated quickly for small numbers, but when it gets larger, there poses a problem.
/// It becomes difficult to solve, so we use the Extended Euclidean Algorithm to calculate the modular mult. inverse when 
/// the numbers are large. Note: The algorithm works for both small numbers as well.
/// 
/// The algorithm(Bézout’s theorem) states that if A and B are two positive integers, 
/// then there exist integers x and y such that Ax + By = GCD(A, B).
/// GCD stands for Greatest Common Divisor. It is the largest integer that divides two numbers without a remainder.
/// The algorithm also works for only relatively prime numbers. These are numbers whose GCD equals 1.
/// Since we know that the GCD must be equal to 1, we can simplify the formula Ax + By = GCD(A, B) by substituting GCD(A, B) for 1.
/// We now have Ax + By = 1.
/// This can still be simplifed further by multiplying the Modulo of B on both sides.
/// We would then have AxModB + ByMoB = 1ModB
/// ByModB will always be 0 for any integer y.
/// So we will be left with AxModB = 1ModB
/// 1ModB will always equal 1 for any integer B
/// The formula is now simplified to just AxModB = 1, which is equivalent to Ax ≅ 1ModB
/// The x value now becomes the modular multiplicative inverse of A. 
/// The algorithm helps us find x.
/// 
/// ======================================Tabular representation of Extended Euclidean Algorithm======================================
/// In the table below, Q represents the quotient. R = remainder. A and B are given by the user. 
/// The first values of x and y are 0 and 1 respectively.
/// To calucluate T, we use the formula T = x - y * Q
/// To calculate Q, we divide A by B.
/// To calculate R, we take the mod of A with respect to B. (i.e. A % B)
/// To get the value of the next row, we do some shifting.
/// A takes the previous value of B.  B takes the previous value of R.  x takes the previous value of y.  y takes the previous value of T. 
/// The remaining values, Q, R and T are then computed.
/// This process is repeated until B becomes 0. When this happen, we take the last value of x(in this case, 2) as the multiplicative inverse.
///         | Q | A | B | R | x | y | T |
///         |---|---|---|---|---|---|---|
///         | 1 | 5 | 3 | 2 | 0 | 1 | -1|
///         | 1 | 3 | 2 | 1 | 1 | -1| 2 |
///         | 2 | 2 | 1 | 0 | -1| 2 | -5|
///         | - | 1 | 0 | - | 2 | -5| - |
/// NB: We assume that the GCD of 3 and 5 is 1
/// The algorithm makes repeated use of integer divisions until the divisor (B) becomes 0 

//@notice:  The first entry point to any program written in rust
fn main() {

    //declaration of variables in rust
    //here we bind the values 3 and 5 to variables a and b respectively
    //Rust is a strongly typed language, so whenever you declare a variable, you must specify the type.
    //In this case we specified a type of unsigned integer with a size of 64 bits.
    //To specify a type in rust, you use a colon followed by the type
    let a = 3;
    let b = 5;

    //The println!() is a macro that displays the result of the modular multiplicative inverse on the screen
    //A macro is a concept unique to rust and is different from a function.
    //It differs in the sense that it can accept an unknown number of parameters. For instance you can call
    //println!("Yooo") with one argument and println!("It's me {}", name) with two arguments.
    //The downside is that macros are more difficult to write compared to functions.
    //The curly braces"{}" in the println macro is a placeholder that tells the compiler that a space in memory
    //should be reserved for a variable or value. 
    println!("The modular multiplicative inverse of {} Mod {} is {}", a, b, modular_multiplicative_inverse(a, b));
}

//dev:          This function calculates the modular multiplicative inverse of a number a with respect to Mod b
//              Rust is a statically typed language, so you must always specify the type for all the parameters in your function
//returns:      Returns the modular multiplicative inverse
//              When returning a value in rust, you must specify the return type. 
fn modular_multiplicative_inverse(a: u64, b: u64) -> u64 {

    //The mod multiplicative inverse of A with respect to Mod B is always zero whenever the value of B is 1
    if b == 1 {
        return 0; 
    }

    //Checks to see if the two given numbers are relatively prime
    //Throws an error if they aren't
    if !is_relatively_prime(a, b) {
        //There are two types of errors in rust. Recoverable and unrecoverable error.
        //An unrecoverable error is an error that halts the program whenever an error occurs.
        //A panic macro is type of unrecoverable error, and that's what we used here.
        //To end the program whenever the numbers given aren't coprime.
        panic!("{} and {} aren't relatively prime", a, b);
    }

    //variables in rust are immutable by default.
    //Because of the fact that the values of our variables will change during the course of these operations,
    //we have to make them mutable by adding the "mut" keyword
    //x, y and t are of type signed integer because of occassions where they become negative
    let mut x: i64 = 0;
    let mut y: i64 = 1;
    let mut t: i64 = 0;

    //A, B, q and r can't become negative throughout the lifecycle of the operation. 
    //This is the reason why they are of type unsigned integer.
    let mut A = b;
    let mut B: u64 = a;
    let mut q = 0;
    let mut r: u64 = 0;

    //A loop to calculate the multiplicative inverse as long as B(the divisor) isn't zero
    while B > 0 {
        q = A/B; //here we calculate the quotient q
        r = A % B; //calculating the remainder r

        //we had to cast the quotient q to a signed integer. the compiler will throw an error if an operation is carried out on different types.
        //to cast/convert a type to another type in rust, you use the "as" keyword
        t = x - y * q as i64; 


        //this is where the shifting occurs.
        //A takes the previous value of B.  B takes the previous value of r.  x takes the previous value of y.  y takes the previous value of t. 
        A = B;
        B = r;
        x = y;
        y = t;
    }

    //if the value of x is below zero, we add it up to 'b' to get a positive value for the multipicative inverse
    if x < 0 {
        x = b as i64 + x; //b has to be converted to a signed integer for the compiler not to throw an error.
    }

    //we expect a positive result (unsigned integer) as our return type. For this reason, x had to be converted to a u64 to be returned correctly.
    return x as u64;
}

//notice:   There exists a modular multiplicative inverse for a number A under Mod B iff both numbers are relatively prime
//          or the GCD(Greatest Common Denominator) is 1
//dev:      This function checks if two numbers are relatively prime.
//returns:  Returns true if two numbers are relatively prime, false otherwise.
fn is_relatively_prime(a: u64, b: u64) -> bool {
    //Rust is an expression based language. An expression is a statement that returns a value.
    //You can return an expression without using the return keyword.
    //But note, you mustn't make use of semicolon at the end. Adding a semicolon converts the expression to a statement.
    gcd(a, b) == 1
} 

//dev:          This function calculates the gcd of two numbers
//Assumption:   Assumes a, b >= 0 
//returns:      Returns the GCD of two integers   
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a; //GCD(a, 0) = a
    } else {
        return  gcd(b, a % b); //GCD(a, b) = GCD(b, a mod b)
    }
} 