// in the std lib, an enum called Option<T> is used when absence is a possibility
// can be either Some(T) or None
// handled explicitly with match or implicitly with unwrap, which returns either the inner element or a panic
// you can manually customize panic with expect, but unwrap usually gives less meaningful output than explicit handling
// all gifts handled with match
fn give_commoner(gift: Option<&str>) {
    // match diff things
    match gift {
        Some("snake") => println!("Yuck! I'm throwing that snake in a fire!"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No gift? Oh well."),
    }
}

// our sheltered princess will panic at the sight of snakes
fn give_princess(gift: Option<&str>) {
    // using unwrap returns a panic when it receives a None
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAAAHHHH!!"); }

    println!("I love {}s!", inside);
}

fn main() {
    let food = Some("chicken");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    let nothing = None;

    give_princess(bird);
    give_princess(nothing);
}