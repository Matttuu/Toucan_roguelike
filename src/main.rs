use rltk::{Rltk, GameState}; // Require "Rltk" & "GameState" types from the namespace "rltk".

struct State{} // New Structure. (Class)
impl GameState for State { // Implement trait "GameState" to "State". (Interface)
    fn tick(&mut self, ctx : &mut Rltk) { // Void function
        ctx.cls();
        ctx.print(1, 1, "Hello Toucan World");
    }
}
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Toucan World")
        .build()?;
    let gs = State {};
    rltk::main_loop(context, gs)
}
