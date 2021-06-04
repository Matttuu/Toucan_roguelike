//use rltk::{Rltk, GameState}; // Require "Rltk" & "GameState" types from the namespace "rltk".
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB
}

struct State{
    ecs: World
} // New Structure. (Class)
impl GameState for State { // Implement trait "GameState" to "State". (Interface)
    fn tick(&mut self, ctx : &mut Rltk) { // Void function // Pass in variable called 'ctx' // '&' Pass a reference (pointer to existing copy of the variable)
        ctx.cls(); // Call function provided by ctx-variable. // This clears the screen
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50() // Builder
        .with_title("Roguelike Toucan World")
        .build()?;
    let mut gs = State {
        ecs: World::new()
    }; // "Variable assigning" by making new variable called 'gs' and copying the 'State' struct above.
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y:25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    for i in 0..10{
        gs.ecs.create_entity()
        .with(Position { x: i * 7, y:20 })
        .with(Renderable {
            glyph: rltk::to_cp437('☺'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
    }
    rltk::main_loop(context, gs)  // Game loop
}
