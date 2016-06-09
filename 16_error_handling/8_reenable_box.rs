// implementing Display and From allows us to us almostall of the std lib error handling tools, oxcept for the ability to box our data types
// std lib will automatically convert from any type which implements the Error trait into the trait object Box<Error> via From, which allows the following
// fn foo(...) -> Result<T, Box<Error>> { ... }
