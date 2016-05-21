// match might be tedious to use all the time, especially with operations that are only valid with an input
// For situations where a simplistic mapping of Some -> Some and None -> None is needed, Option has a built in method called map()
// map can be chained together
#![allow(dead_code)]

#[derive(Debug)] enum Food { Apple, Carrot, Potato }

#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// peeling food, if there isn't any then return None
// otherwise return peeled food
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// like above, we need to know if the peeled food exists before chopping
fn chop(food: Option<Peeled>) -> Option<Chopped> {
    match food {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// uses map instead of match
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

// we can use map chaining to simplify process
fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

// check for food before eating it
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("mmm. I love {:?}.", food),
        None => println!("Oh no, it wasn't edible."),
    }
}

fn main() {
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));

    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}