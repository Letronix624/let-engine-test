use let_engine::*;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use winit::dpi::LogicalSize;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
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
        .with_inner_size(LogicalSize::new(800, 600))
        .with_always_on_top(false)
        .with_decorations(true)
        .with_transparent(false);

    let mut resources = Resources::new();
    resources.add_texture("rusty", include_bytes!("../assets/textures/rustyl2.png"));
    resources.add_font(
        "Bani-Regular",
        include_bytes!("../assets/fonts/Bani-Regular.ttf"),
    );

    let (mut game, event_loop) = GameBuilder::new()
        .with_window_builder(window_builder)
        .with_app_info(app_info)
        .with_resources(resources)
        .build();

    let mut objects = vec![];

    let mut player1 = Object::new();
    player1.position = [-0.5, -0.5];
    player1.size = [0.45, 0.45];
    player1.color = [1.0, 0.0, 0.0, 1.0];
    let graphics = VisualObject::new_square();
    player1.graphics = Some(graphics);
    let player1 = Arc::new(Mutex::new(ObjectNode::new(
        player1.clone(),
        vec![],
    )));
    objects.push(player1.clone());

    let mut player = Object::new();
    player.position = [1.0, -0.0];
    player.size = [1.0, 1.0];
    player.color = [0.0, 1.0, 0.0, 1.0];
    let graphics = VisualObject::new_square();
    player.graphics = Some(graphics);
    objects[0].lock().unwrap().children.push(Arc::new(Mutex::new(ObjectNode::new(player, vec![]))));

    let mut player = Object::new();
    player.position = [0.0, 1.0];
    player.size = [1.0, 1.0];
    player.color = [0.0, 0.0, 1.0, 1.0];
    let graphics = VisualObject::new_square();
    player.graphics = Some(graphics);
    objects[0].lock().unwrap().children.push(Arc::new(Mutex::new(ObjectNode::new(player, vec![]))));

    let mut player = Object::new();
    player.position = [1.0, 0.0];
    player.size = [1.0, 1.0];
    player.color = [1.0, 1.0, 0.0, 1.0];
    let graphics = VisualObject::new_square();
    player.graphics = Some(graphics);
    objects[0].lock().unwrap().children[1].lock().unwrap().children.push(Arc::new(Mutex::new(ObjectNode::new(player, vec![]))));


    let mut dt = Instant::now();

    let timeline = Instant::now();

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

                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    control_flow.set_exit();
                }
            }
            Event::RedrawEventsCleared => {
                objects[0].lock().unwrap().object.position = [
                    (timeline.elapsed().as_secs_f32() * 2.0).cos() / 3.0 -0.5,
                    (timeline.elapsed().as_secs_f32() * 2.0).sin() / 3.0 -0.5,
                ];
                game.objects = objects.clone();
                game.update();
                println!("{}", 1.0 / dt.elapsed().as_secs_f64());
                dt = Instant::now();
            }
            _ => (),
        }
    });
}
