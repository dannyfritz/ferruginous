use graphics_2d::Shapes;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum GameError {}
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GameError")
    }
}
impl Error for GameError {}

type GameResult = Result<(), GameError>;

struct Transform {}
impl graphics_2d::Transform for Transform {}

struct Extra {}
impl graphics_2d::Extra for Extra {}

struct Graphics {}

impl Shapes<Transform, Extra> for Graphics {
    type Scalar = f32;
    type UserError = GameError;
    fn circle(&mut self, radius: f32, transform: &Transform, style: &Extra) -> GameResult {
        Ok(())
    }
    fn rectangle(
        &mut self,
        width: f32,
        height: f32,
        transform: &Transform,
        style: &Extra,
    ) -> GameResult {
        Ok(())
    }
    fn line(&mut self, width: f32, transform: &Transform, style: &Extra) -> GameResult {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    let _graphics = Graphics {};
}
