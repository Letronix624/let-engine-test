use let_engine::*;
use image::{codecs::png, ImageDecoder};
use std::{
    sync::{Arc},
    time::Instant,
    io::Cursor
};
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::window::WindowBuilder;


fn main() {
    let app_info = AppInfo {
        app_name: "Let Engine Test",
    };
    let window_builder = WindowBuilder::new()
        .with_resizable(true)
        .with_title("Test Window")
        .with_min_inner_size(LogicalSize::new(200, 200))
        .with_inner_size(LogicalSize::new(1000, 1000))
        .with_always_on_top(false)
        .with_decorations(true)
        .with_transparent(true)
        .with_visible(false);
    

    let (mut game, event_loop) = GameBuilder::new()
        .with_window_builder(window_builder)
        .with_app_info(app_info)
        .build();

    game.load_font_bytes(
        "Rawr-Regular",
        include_bytes!("../assets/fonts/Hand-Regular.ttf"),
    );

    let layer = game.new_layer();

    // Objects
    let mut text = Object::new();

    text.graphics = game.get_font_data(
        "Rawr-Regular",
        "ABC abc Super Cool!!",
        70.0,
        [1.0; 4]
    );
    let mut text2 = text.clone();
    text2.position = [0.0, 0.3];

    let _text = game.add_object(&layer, text).unwrap();
    let _text2 = game.add_object(&layer, text2).unwrap();

    let camera = game.add_object(&layer, Object::new()).unwrap();

    camera.lock().camera = Some( CameraOption {zoom: 1.0, mode: CameraScaling::Limited} );

    game.set_camera(&layer, &camera).unwrap();

    
    
    let mut mist = false;

    game.get_window().set_visible(true);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                game.recreate_swapchain();
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if input.state == ElementState::Pressed {
                    println!("{:?}", input.virtual_keycode);
                }
                match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => {
                        control_flow.set_exit();
                    }
                    Some(VirtualKeyCode::A) => {
                        if input.state == ElementState::Pressed {
                            mist = true;
                        }
                        else {
                            mist = false;   
                        }
                    }
                    _ => ()
                }
            }
            Event::RedrawEventsCleared => {
                game.update();
                println!("{}", game.fps());
                // if mist
                // {
                //     println!("{}", game.contains_object(&background));   
                //     match game.remove_object(&background) {
                //         Ok(_) => println!("Removed success"),
                //         Err(_) => println!("No success.")
                //     };
                // }
                // println!("{}", 1.0 / dt.elapsed().as_secs_f64());

            }
            _ => (),
        }
    });
}
