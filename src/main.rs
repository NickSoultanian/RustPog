use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder,State};
fn main() -> tetra::Result{
    ContextBuilder::new("Pog", 640, 480) //holds framework of game
        .quit_on_escape(true) //esc key to close game
        .build()?
        .run(|_ctx| Ok(GameState{}))
}

struct GameState{}
//stores current state of the game
impl State for GameState{}