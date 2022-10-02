/*
  ██      ██    ██ ███    ██  █████  ██████      ██       █████  ███    ██ ██████  ███████ ██████      
  ██      ██    ██ ████   ██ ██   ██ ██   ██     ██      ██   ██ ████   ██ ██   ██ ██      ██   ██     
  ██      ██    ██ ██ ██  ██ ███████ ██████      ██      ███████ ██ ██  ██ ██   ██ █████   ██████      
  ██      ██    ██ ██  ██ ██ ██   ██ ██   ██     ██      ██   ██ ██  ██ ██ ██   ██ ██      ██   ██     
  ███████  ██████  ██   ████ ██   ██ ██   ██     ███████ ██   ██ ██   ████ ██████  ███████ ██   ██   
    
    @Author : GCast31 / From formation GAMECODEUR.COM
*/
use game2d::game::common::{Dimension, Angle, Velocity2d, Position, Point2d, GAME_FONT_DEFAULT_, GAME_FONT_DEFAULT_SIZE, Scale2d, Velocity, Force, Force2d, angle_add};
use game2d::game::game::*;
use game2d::graphics::color::Color;
use game2d::graphics::fonts::FontsManager;
use game2d::graphics::graphics::Graphics;

use game2d::graphics::images::Image;
use game2d::inputs::keyboard::Keyboard;

use game2d::inputs::keyboard::Keys;


// ################################################################################################################
// #                                      C O N S T R A N T E S  FOR  G A M E                                     #
// ################################################################################################################
pub const GAME_WINDOW_HEIGHT: Dimension = 600;
pub const GAME_WINDOW_WIDTH: Dimension = 800;

pub const GRAVITY: Velocity = 0.6;

// ################################################################################################################
// #                                        S T R U C T U R E    G A M E                                          #
// ################################################################################################################
pub struct Lunarlander {
    lander: Option<Lander>,
}

pub enum EngineState {
    Activate,
    Disable,
}


pub struct Lander {
    position: Point2d,
    angle: Angle,
    velocity: Velocity2d,
    image: Option<Image>,
    scale: Scale2d,
    speed: Velocity2d,

    engine: Option<Engine>,
}

impl Default for Lander {
    fn default() -> Self {
        Lander { 
            position: Point2d {x: 0., y: 0.}, 
            angle: 270., 
            velocity: Velocity2d{x: 0., y: 0.},  
            scale: Scale2d { sx: 1., sy: 1. },
            speed: Velocity2d { x: 3., y: 3. },
            image: Option::None,

            engine: Some(Engine{..Default::default()}),
        }
    }
}

pub struct Engine {
    position: Point2d,
    angle: Angle,
    velocity: Velocity2d,
    image: Option<Image>,

    state: EngineState,
}

impl Default for Engine {
    fn default() -> Self {
        Engine { 
            position: Point2d {x: 0., y: 0.}, 
            angle: 0., 
            velocity: Velocity2d{x: 0., y: 0.},  
            image: Option::None,
            state: EngineState::Disable,
        }
    }
}


#[allow(dead_code)]
impl Default for Lunarlander {
    fn default() -> Self {
        Lunarlander {
            lander: Option::None,
        }
    }
}

// ################################################################################################################
// #                                                   M A I N                                                    #
// ################################################################################################################
fn main() {

    let mut graphics = Graphics::new(
            "Lunarlander", 
            GAME_WINDOW_WIDTH, 
            GAME_WINDOW_HEIGHT, 
            false
        )
        .unwrap();

    // Fonts
    let mut font_context = Graphics::create_fonts_context();
    let mut fonts_manager: FontsManager = FontsManager::new(graphics.get_fonts_creator());
    let font_detail = fonts_manager.load_font(&mut font_context, GAME_FONT_DEFAULT_.to_string(), GAME_FONT_DEFAULT_SIZE).unwrap();
    graphics.set_font(font_detail);
    
    // Game
    Game::new(graphics)
            
        .set_params(Lunarlander::default())
        .set_max_fps(Some(144.))
        .set_callback_draw(draw)
        .set_callback_load(load)
        .set_callback_key_pressed(keypressed)
        .set_callback_update(update)
        .set_callback_quit(quit)
        .run(&mut Some(fonts_manager));

}

// ################################################################################################################
// #                                                    L O A D                                                   #
// ################################################################################################################
#[allow(unused_variables)]
pub fn load(graphics: &mut Graphics, game: &mut Option<Lunarlander>) {

    // Set background color
    graphics.set_background_color(Color::BLUE);

    // Set font color
    graphics.set_font_color(Color {r: 255, g: 255, b: 255, a: 120});

    // Create Lander + Engine
    if let Some(game) = game {
        let lander = Lander {
            position: Point2d {
                x: (GAME_WINDOW_WIDTH as f32 / 2.) as Position,
                y: (GAME_WINDOW_HEIGHT as f32 / 2.) as Position,
            },
            image: Some(graphics.new_image("images/ship.png").unwrap()),
            engine: Some(Engine {
                            image: Some(graphics.new_image("images/engine.png").unwrap()),
                            ..Default::default()
                        }
                    ),   
            ..Default::default()
        };
        game.lander = Some(lander);    
    }

}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<Lunarlander>, keyboard: &mut Keyboard, dt: f32) {
    if let Some(game) = game {
        // Draw lander
        if let Some(lander) = &mut game.lander {
            lander.velocity.y = lander.velocity.y + (GRAVITY * dt);

            // ****** SHIP
            if keyboard.is_down(&Keys::Right) {
                lander.angle = lander.angle + 90. as Angle * dt as Angle;
            }

            if keyboard.is_down(&Keys::Left) {
                lander.angle = lander.angle - 90. as Angle * dt as Angle;
            }

            lander.angle = angle_add(lander.angle, 0., true);

            // ****** ENGINE
            if let Some(engine) = &mut lander.engine {

                engine.angle = lander.angle;
                engine.position.x = lander.position.x - (5. * lander.scale.sx) ;
                engine.position.y = lander.position.y - (5. * lander.scale.sy) ;

                // Activate engine
                if keyboard.is_down(&Keys::Up) {
                    engine.state = EngineState::Activate;

                    let force = Force2d::new(lander.angle, lander.speed);
                    lander.velocity.x += force.fx * dt;
                    lander.velocity.y += force.fy * dt;
                } 
                // Disable engine
                else {
                    engine.state = EngineState::Disable;
                }

                engine.velocity   = lander.velocity;
                engine.position = Point2d::add_velocity2d(&mut engine.position, &engine.velocity);   
            }

            lander.position = Point2d::add_velocity2d(&mut lander.position, &lander.velocity);

        }
    }
}


// ################################################################################################################
// #                                               K E Y P R E S S E D                                            #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn keypressed(graphics: &mut Graphics, game: &mut Option<Lunarlander>, key: &Keys) {
    
}

// ################################################################################################################
// #                                                    D R A W                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn draw(graphics: &mut Graphics, game: &mut Option<Lunarlander>, fonts_manager: &mut Option<FontsManager>) {
   
    graphics.set_color(Color::BLUE);

    if let Some(game) = game {
        // #########################################  Draw lander ###############################################
        if let Some(lander) = &mut game.lander {

            // Ship   
            if let Some(image) = &lander.image { 
                graphics.draw_full(
                    image, 
                    lander.position.x, 
                    lander.position.y, 
                    lander.angle, 
                    lander.scale.sx, 
                    lander.scale.sy, 
                    0., 
                    0.,
                );

            } 

            // Engine
            if let Some(engine) = &lander.engine {
                if let EngineState::Activate = engine.state {
                    if let Some(image) = &engine.image { 
                        graphics.draw_full(
                            image, 
                            engine.position.x, 
                            engine.position.y, 
                            engine.angle, 
                            lander.scale.sx, 
                            lander.scale.sy, 
                            0., 
                            0.,
                        );
                    } 
                }
            }

            // Debug
            if let Some(fonts_manager) = fonts_manager {
                let mut s_debug = "Debug : ".to_string();
                s_debug = s_debug 
                        + " angle : " + &lander.angle.to_string()
                        + " vx : " + &lander.velocity.x.to_string()
                        + " vy : " + &lander.velocity.x.to_string();
                graphics.print(fonts_manager, s_debug, 0., 0., Option::None);
            }
        }

    }

}

// ################################################################################################################
// #                                                    Q U I T                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn quit(graphics: &mut Graphics, game: &mut Option<Lunarlander>) {
    println!("Bye");
}
