use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};

const PADDLE_SPEED: f32 = 8.0;
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const BALL_SPEED: f32 = 5.0;
const BALL_STOP: f32 = 0.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_ACC: f32 = 0.05;
static mut PADDLE_CENTER: f32 = 0.0;
static mut PlAYER1_SCORE: i32 = 0;
static mut PlAYER2_SCORE: i32 = 0;

fn main() -> tetra::Result{
    ContextBuilder::new("Pog", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32) //holds framework of game
        .quit_on_escape(true) //esc key to close game
        .build()?
        .run(GameState::new)
}
struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}
impl Entity {
    fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity {
        Entity {
            texture,
            position,
            velocity,
        }
    }

    fn width(&self) -> f32 {
        self.texture.width() as f32
    }

    fn height(&self) -> f32 {
        self.texture.height() as f32
    }

    fn bound(&self) -> Rectangle {
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    fn centre(&self) -> Vec2<f32> {
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }

}
struct GameState {
    player1: Entity,
    player2: Entity,
    centerLine: Entity,
    ball: Entity,
    player1_score1: Entity,
    player1_score2: Entity,
    player1_score3: Entity,
    player1_score4: Entity,
    player1_score5: Entity,
    player2_score1: Entity,
    player2_score2: Entity,
    player2_score3: Entity,
    player2_score4: Entity,
    player2_score5: Entity,
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

        unsafe { PADDLE_CENTER = (WINDOW_HEIGHT - player1_texture.height() as f32)/2.0; }

        let center_texture = Texture::new(ctx, "./resources/center.png")?;
        let center_position = Vec2::new(
            0 as f32, 0 as f32,
        );

        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2:: new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );

        let ball_velocity = Vec2::new(-BALL_SPEED, 0.0);

        let score1_texture = Texture::new(ctx, "./resources/1.png")?;
        let score2_texture = Texture::new(ctx, "./resources/2.png")?;
        let score3_texture = Texture::new(ctx, "./resources/3.png")?;
        let score4_texture = Texture::new(ctx, "./resources/4.png")?;
        let score5_texture = Texture::new(ctx, "./resources/5.png")?;

        let p2score1_texture = Texture::new(ctx, "./resources/1.png")?;
        let p2score2_texture = Texture::new(ctx, "./resources/2.png")?;
        let p2score3_texture = Texture::new(ctx, "./resources/3.png")?;
        let p2score4_texture = Texture::new(ctx, "./resources/4.png")?;
        let p2score5_texture = Texture::new(ctx, "./resources/5.png")?;

        let p1Win_Screen = Texture::new(ctx,"./resources/p1_Wins.png")?;
        let p2Win_Screen = Texture::new(ctx,"./resources/p2_Wins.png")?;


        let score1_position = Vec2::new(
            5.0,
            5.0,
        );

        let score2_position = Vec2::new(
            500.0,
            5.0,
        );

        Ok(GameState {
            player1: Entity::new(player1_texture, player1_position),
            player2: Entity::new(player2_texture, player2_position),
            centerLine: Entity::new(center_texture, center_position),
            ball: Entity::with_velocity(ball_texture, ball_position, ball_velocity),
            player1_score1: Entity::new(score1_texture, score1_position),
            player1_score2: Entity::new(score2_texture, score1_position),
            player1_score3: Entity::new(score3_texture, score1_position),
            player1_score4: Entity::new(score4_texture, score1_position),
            player1_score5: Entity::new(score5_texture, score1_position),
            player2_score1: Entity::new(p2score1_texture, score2_position),
            player2_score2: Entity::new(p2score2_texture, score2_position),
            player2_score3: Entity::new(p2score3_texture, score2_position),
            player2_score4: Entity::new(p2score4_texture, score2_position),
            player2_score5: Entity::new(p2score5_texture, score2_position),
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

        self.ball.position += self.ball.velocity;

        let player1_bounds = self.player1.bound();
        let player2_bounds = self.player2.bound();
        let ball_bounds = self.ball.bound();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
            Some(&self.player1)
        }
        else if ball_bounds.intersects(&player2_bounds) {
            Some(&self.player2)
        } else {
            None
        };

        if let Some(paddle) = paddle_hit {
            self.ball.velocity.x = -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));

            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT
        {
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        if self.ball.position.x < 0.0 && unsafe { PlAYER1_SCORE } != 5{
            self.ball.position.x = WINDOW_WIDTH / 2.0;
            self.ball.position.y = WINDOW_HEIGHT / 2.0;
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
            self.player1.position.y = unsafe { PADDLE_CENTER };
            self.player2.position.y = unsafe { PADDLE_CENTER };
            unsafe { PlAYER2_SCORE += 1 }
        }

        if self.ball.position.x > WINDOW_WIDTH && unsafe { PlAYER1_SCORE } != 5{
            self.ball.position.x = WINDOW_WIDTH / 2.0;
            self.ball.position.y = WINDOW_HEIGHT / 2.0;
            self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
            self.player1.position.y = unsafe { PADDLE_CENTER };
            self.player2.position.y = unsafe { PADDLE_CENTER };
            unsafe { PlAYER1_SCORE += 1 }
        }

        if unsafe { PlAYER1_SCORE } == 5 {
            self.ball.velocity = Vec2::new(BALL_STOP,0.0); //pauses game once win condition is reached
        }

        if unsafe { PlAYER2_SCORE } == 5 {
            self.ball.velocity = Vec2::new(BALL_STOP,0.0);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        if unsafe { PlAYER1_SCORE } == 1 {
            graphics::draw(ctx, &self.player1_score1.texture, self.player1_score1.position);
        }
        if unsafe { PlAYER1_SCORE } == 2 {
            graphics::draw(ctx, &self.player1_score2.texture, self.player1_score2.position);
        }
        if unsafe { PlAYER1_SCORE } == 3 {
            graphics::draw(ctx, &self.player1_score3.texture, self.player1_score3.position);
        }
        if unsafe { PlAYER1_SCORE } == 4 {
            graphics::draw(ctx, &self.player1_score4.texture, self.player1_score4.position);
        }
        if unsafe { PlAYER1_SCORE } == 5 {
            graphics::draw(ctx, &self.player1_score5.texture, self.player1_score5.position);
        }
        if unsafe { PlAYER2_SCORE } == 1 {
            graphics::draw(ctx, &self.player2_score1.texture, self.player2_score1.position);
        }
        if unsafe { PlAYER2_SCORE } == 2 {
            graphics::draw(ctx, &self.player2_score2.texture, self.player2_score2.position);
        }
        if unsafe { PlAYER2_SCORE } == 3 {
            graphics::draw(ctx, &self.player2_score3.texture, self.player2_score3.position);
        }
        if unsafe { PlAYER2_SCORE } == 4 {
            graphics::draw(ctx, &self.player2_score4.texture, self.player2_score4.position);
        }
        if unsafe { PlAYER2_SCORE } == 5 {
            graphics::draw(ctx, &self.player2_score5.texture, self.player2_score5.position);
        }

        graphics::draw(ctx, &self.player1.texture, self.player1.position);
        graphics::draw(ctx, &self.player2.texture, self.player2.position);
        graphics::draw(ctx, &self.centerLine.texture, self.centerLine.position);
        graphics::draw(ctx, &self.ball.texture, self.ball.position);
        Ok(())
    }
}