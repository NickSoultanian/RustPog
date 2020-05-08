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
impl State for GameState{
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));//background color

        Ok(())
    }
}