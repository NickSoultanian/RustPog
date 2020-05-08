use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const PADDLE_SPEED: f32 = 8.0;
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;

fn main() -> tetra::Result{
    ContextBuilder::new("Pog", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32) //holds framework of game
        .quit_on_escape(true) //esc key to close game
        .build()?
        .run(GameState::new)
}
struct Entity {
    texture: Texture,
    position: Vec2<f32>,
}
impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position }
    }
}
struct GameState {
    player1: Entity,
    player2: Entity,
    centerLine: Entity,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        let player1_position = Vec2::new(
            16.0,
            (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
        );
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32)/2.0,
        );
        let center_texture = Texture::new(ctx, "./resources/center.png")?;
        let center_position = Vec2::new(
            0 as f32, 0 as f32,
        );
        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            centerLine: Entity::new(center_texture, center_position),
        })
    }
}
//stores current state of the game
impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_down(ctx, Key::W) {
            self.player1.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::S) {
            self.player1.position.y += PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Up){
            self.player2.position.y -= PADDLE_SPEED;
        }

        if input::is_key_down(ctx, Key::Down){
            self.player2.position.y += PADDLE_SPEED;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.centerLine.texture, self.centerLine.position);
        Ok(())
    }
}