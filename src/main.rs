use let_engine::*;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::window::WindowBuilder;
fn main() {
    let app_info = AppInfo {
        app_name: "Let Engine Test",
    };
    let window_builder = WindowBuilder::new()
        .with_resizable(false)
        .with_title("Test Window")
        .with_min_inner_size(PhysicalSize::new(150.0, 150.0))
        .with_inner_size(PhysicalSize::new(600.0, 600.0))
        .with_decorations(true)
        .with_transparent(true)
        .with_visible(false);
    let (mut game, event_loop) = GameBuilder::new()
        .with_window_builder(window_builder)
        .with_app_info(app_info)
        .build();
    let font = game.load_font(include_bytes!("../assets/fonts/FlowCircular-Regular.ttf"));
    let layer = game.new_layer();
    let mut txt = String::new();
    let fsize = 35.0;
    let rtext = Object::new().graphics(Some(
        Appearance::new_color([1.0, 0.0, 0.0, 1.0]).size([0.6; 2]),
    ));
    let gtext = Object::new().graphics(Some(
        Appearance::new_color([0.0, 1.0, 0.0, 1.0]).size([0.6; 2]),
    ));
    let btext = Object::new().graphics(Some(
        Appearance::new_color([0.0, 0.0, 1.0, 1.0]).size([0.6; 2]),
    ));

    let rtext = game.add_object(&layer, rtext).unwrap();
    let gtext = game.add_object(&layer, gtext).unwrap();
    let btext = game.add_object(&layer, btext).unwrap();

    game.queue_to_label(&rtext, &font, &txt, fsize, NW);
    game.queue_to_label(&gtext, &font, &txt, fsize, CENTER);
    game.queue_to_label(&btext, &font, &txt, fsize, SO);

    let camera = game.add_object(&layer, Object::new()).unwrap();
    camera.lock().camera = Some(CameraOption {
        zoom: 1.0,
        mode: CameraScaling::Expand,
    });
    game.set_camera(&layer, &camera).unwrap();
    game.get_window().set_visible(true);
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        game.update(&event);
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                        control_flow.set_exit();
                    }
                }
                WindowEvent::ReceivedCharacter(c) => {
                    match c {
                        '\u{8}' => {
                            txt.pop();
                        }
                        _ if c != '\u{7f}' => txt.push(c),
                        _ => {}
                    }
                    game.queue_to_label(&rtext, &font, &txt, fsize, NW);
                    game.queue_to_label(&gtext, &font, &txt, fsize, CENTER);
                    game.queue_to_label(&btext, &font, &txt, fsize, SO);
                }
                _ => (),
            },
            _ => (),
        }
    });
}
