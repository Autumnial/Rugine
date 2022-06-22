// use macroquad::*;
use macroquad::{prelude::*};

mod engine;

fn window_conf() -> Conf {
    Conf{
        window_title: String::from("owo"),
        window_height: 800,
        window_width: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    // let mut direction = true;
    let mut rec1 = engine::Square::new(40.0, 40.0, 80.0, engine::Colour{r: 0.5, g: 0.0, b: 0.0, a: 1.0}, );
    let mut rec2 = engine::Square::new(100., 100., 60., engine::Colour{r:0.4, g:0.6, b:0.5, a:1.});
    loop{
        clear_background(BLACK);


        // if direction{
        //     rec1.colour += engine::Colour{r: 0.01, g: 0.01, b: 0.01, a: 0. };
        // } else {
        //     rec1.colour -= engine::Colour{r: 0., g: 0.01, b: 0.01, a: 0.};
        // }

        // if is_key_down(KeyCode::Space){
        //     direction = !direction;
        // }
        rec1.update();
        rec2.update();
        rec1.draw();
        rec2.draw();


        next_frame().await
        
    }
}

