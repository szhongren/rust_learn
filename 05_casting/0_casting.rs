// rust provides no implicit type conversion between primitive types, but explicit type casting can be done with the as keyword
// supress all warnings from casts that overflow
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // error! no implicit conversion
    // let integer: u8 = decimal;

    // explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T, std::T::MAX + 1 is added or subtracted until the value fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // under the hood, the first 8 bits from the LSB are used, while the rest towards the MSB get truncated
    println!("1000 as a u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);

    // for positive numbers, this is the same as the modulus
    println!("1000 % 256 is: {}", 1000 % 256);

    // when casting to a signed type, result is the same as first casting to corresponding unsigned type then taking 2's complement
    // unless it already fits
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, whose two's complement in eight bits is:
    // 128 is 1000_0000 which is the min value of an i8
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a i8 is : {}", 1000 as i8);
    // and the two's complement of 232 is -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
