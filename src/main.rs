use std::collections::HashMap;
use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlProfile},
    Sdl,
};

fn main() {
    println!("Hello, evomeca world!");
    println!(" ");
    println!(" ---              ---");
    println!(" --                -- ");
    println!(" -    BIENVENUE     -");
    println!("       DANS L ANTRE    ");

    let sdl = Sdl::init(InitFlags::EVERYTHING);

    let win = sdl
        .create_gl_window(CreateWinArgs { title: "Example GL Window", ..Default::default() })
        .unwrap();
    println!("GL window size: {:?}", win.get_window_size());
    println!("GL drawable size: {:?}", win.get_drawable_size());
    println!("GL_KHR_debug supported: {}", win.supports_extension("GL_KHR_debug"));
}


fn meca_factory(root: Root) {
    let pivot_positions = recursive_factory(root.root, root.x, root.y, );
}

fn recursive_factory(pivot: Pivot, x: f32, y:f32) {
    for i in 0..pivot.beams.len() {
        x
    }
}



struct Root {
    x: f32,
    y: f32,
    angle: f32,
    pivots: 

}

struct Pivot {
    velocity: f32,
    aperture: f32,
    beams: HashMap<Pivot, (f32, f32)>, // (angle, distance)
}





