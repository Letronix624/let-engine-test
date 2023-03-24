use let_engine::*;
use parking_lot::Mutex;
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


    // let image_bytes = include_bytes!("../assets/textures/placeholder.png");
    // let image_cursor = Cursor::new(image_bytes);

    // let decoder = png::PngDecoder::new(image_cursor).unwrap();
    // let dimensions = decoder.dimensions();
    // let mut buf = vec![0; (dimensions.0 * dimensions.1) as usize * 4];
    // decoder.read_image(&mut buf).unwrap();

    // game.load_texture("placeholder", buf, dimensions.0, dimensions.1);

    // game.load_font_bytes(
    //     "Rawr-Regular",
    //     include_bytes!("../assets/fonts/Rawr-Regular.ttf"),
    // );

    // let background = Arc::new(Mutex::new(Object::new_square()));
    // {
    //     let mut background = background.lock();
    //     background.position = [0.0, 1.0];
    //     background.size = [1.0; 2];
    //     background.graphics = Some(
    //         Appearance::new_square().texture("placeholder").material(1)
    //     );
    // }

    // game.add_object(&background);

    // let mut label = Object::new_square();
    // label.size = [1.0; 2];
    // label.position = [0.0, 0.5];
    // label.graphics = Some(
    //     game.get_font_data(
    //         "Rawr-Regular",
    //         "Test test nice nice",
    //         70.0,
    //         [1.0; 4]
    //     )
    // );

    // game.add_child_object(&background, &Arc::new(Mutex::new(label)));

    //let bg = background.lock().unwrap();

    //bg.size;

    let layer = game.new_layer();

    let someobj = Arc::new(Mutex::new(Object::new_square()));

    game.add_object(&layer, &someobj).unwrap();

    let other = Arc::new(Mutex::new(Object::new()));

    {
        let mut obj = other.lock();
        let mut some = someobj.lock();
        obj.size = [0.1; 2];
        obj.graphics = Some(
            Appearance::new()
                .data(make_circle!(10))
                .color([0.1, 0.0, 1.0, 1.0])
        );
        some.camera = Some(
            CameraOption {
                zoom: 1.5,
                mode: CameraScaling::Expand
            }
        );
        some.position = [1.0, 0.0];
        
    }

    game.add_object(&layer, &other).unwrap();

    game.set_camera(&layer, &someobj).unwrap();
    

    let mut dt = Instant::now();

    let mut mist = false;

    let mut time = Instant::now();

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
                let fps = (1.0 / dt.elapsed().as_secs_f64()) as u32;
                // if mist
                // {
                //     println!("{}", game.contains_object(&background));   
                //     match game.remove_object(&background) {
                //         Ok(_) => println!("Removed success"),
                //         Err(_) => println!("No success.")
                //     };
                // }
                // println!("{}", 1.0 / dt.elapsed().as_secs_f64());

                let secs = time.elapsed().as_secs_f32();

                {
                    let mut big = someobj.lock();
                    // other.position = [
                    //     secs.cos(),
                    //     secs.sin(),
                    // ];

                    big.rotation = secs;

                }

                

                
                
                dt = Instant::now();
            }
            _ => (),
        }
    });
}
