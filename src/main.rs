use rltk::{Rltk, GameState}; // Require "Rltk" & "GameState" types from the namespace "rltk".

struct State{} // New Structure. (Class)
impl GameState for State { // Implement trait "GameState" to "State". (Interface)
    fn tick(&mut self, ctx : &mut Rltk) { // Void function // Pass in variable called 'ctx' // '&' Pass a reference (pointer to existing copy of the variable)
        ctx.cls(); // Call function provided by ctx-variable. // This clears the screen
        ctx.print(1, 1, "Hello Toucan World"); // Prints string at location (1,1)
    }
}
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50() // Builder
        .with_title("Roguelike Toucan World")
        .build()?;
    let gs = State {}; // "Variable assigning" by making new variable called 'gs' and copying the 'State' struct above.
    rltk::main_loop(context, gs)  // Game loop
}
