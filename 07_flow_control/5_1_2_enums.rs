// enums are destructured like tuples
// allow to silence warnings because only one variant is used
#[allow(dead_code)]
#[derive(Debug)]
enum Color {
    // 3 specified solely by name
    Red,
    Blue,
    Green,
    // tie u32 tuples to different names of color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    // destructure enum with match
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Green => println!("The color is Green!"),
        Color::Blue => println!("The color is Blue!"),
        Color::RGB(r, g, b) =>
            println!("red: {}, green: {}, blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("cyan: {}, magenta: {}, yellow: {}, key: {}!", c, m, y, k),
    }
}
