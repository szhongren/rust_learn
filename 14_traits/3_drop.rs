// Drop trait only has one method: drop, called automatically when an object goes out of scope
// main use of Drop trait is to free the resources that the implementor instance owns
// Box, Vec, String, File, and Process implement Drop trait, and it can be manually implemented for any custom data type
// adding a print to the drop function to announce when it is called
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    // blk A
    {
        let _b = Droppable { name: "b" };

        // blk B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting blk B");
        }
        println!("Just exited blk B");

        println!("Exiting blk A");
    }
    println!("Just exited blk A");

    // vars can be manually dropped
    drop(_a);

    println!("end of main fn");
    // a won't be dropped here as it has already been dropped
}