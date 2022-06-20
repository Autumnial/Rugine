// use macroquad::*;
use macroquad::prelude::*;

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
    loop{
        let start = std::time::Instant::now();
        clear_background(BLACK);

        // just so we have smth to look at :3 
        let elapsed = start.elapsed();
        if elapsed.as_millis() != 0 {
            println!("fps: {}", 1000/ elapsed.as_millis());
        } else{
            println!("took: {} ms", elapsed.as_millis())
        }

        next_frame().await
        
    }
}
