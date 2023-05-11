use let_engine::*;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event::MouseButton;
use winit::window::WindowBuilder;
fn main() {
    let window_builder = WindowBuilder::new()
        .with_resizable(false)
        .with_title("Test Window")
        .with_min_inner_size(PhysicalSize::new(150.0, 150.0))
        .with_inner_size(PhysicalSize::new(1000.0, 700.0))
        .with_decorations(true)
        .with_transparent(true)
        .with_visible(false);
    let (mut game, event_loop) = GameBuilder::new()
        .with_window_builder(window_builder)
        .build();

    let resources = game.resources.clone();
    let input = game.input.clone();

    let font = resources.load_font(include_bytes!("../assets/fonts/Px437_CL_Stingray_8x16.ttf"));
    let layer = game.scene.new_layer();
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
    let rtext = layer.add_object(None, rtext).unwrap();
    let gtext = layer.add_object(None, gtext).unwrap();
    let btext = layer.add_object(None, btext).unwrap();

    resources.queue_to_label(&rtext, &font, &txt, fsize, NW);
    resources.queue_to_label(&gtext, &font, &txt, fsize, CENTER);
    resources.queue_to_label(&btext, &font, &txt, fsize, SO);
    let mut camera = Object::new();
    camera.position = [0.0, 0.0];
    let camera = layer.add_object(None, camera).unwrap();
    camera.lock().camera = Some(CameraOption {
        zoom: 1.0,
        mode: CameraScaling::Expand,
    });
    game.set_clear_background_color([0.3, 0.0, 0.3, 0.1]);
    layer.set_camera(&camera).unwrap();
    resources.get_window().set_visible(true);

    let mut test = Object::new().graphics(Some(
        Appearance::new().size([0.1; 2]).data(make_circle!(10, 0.5))
    ));

    test.position = layer.side_to_world(N, (1000.0, 700.0));
    let _test = layer.add_object(None, test).unwrap();


    let mut last = false;

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
                    resources.queue_to_label(&rtext, &font, &txt, fsize, NW);
                    resources.queue_to_label(&gtext, &font, &txt, fsize, CENTER);
                    resources.queue_to_label(&btext, &font, &txt, fsize, SO);
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                {
                    if input.mouse_down(&MouseButton::Left) && !last {
                        let mut object = Object::new().graphics(Some(
                            Appearance::new().size([0.03; 2]).data(Data::square())
                        ));
                        object.position = input.cursor_to_world(&layer, (1000.0, 700.0));
                        {
                            let object = layer.add_object(None, object).unwrap();
                            println!("{}", object.lock().get_id());
                        }
                    }
                    last = input.mouse_down(&MouseButton::Left);
                }
            }
            _ => (),
        }
    });
}
