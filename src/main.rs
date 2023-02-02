use let_engine::*;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::{
    Event,
    WindowEvent, KeyboardInput, VirtualKeyCode
};
use std::{
    time::Instant,
    sync::mpsc::channel
};

fn main() {
    let app_info = AppInfo {
        app_name: "Let Engine Test",
    };
    let window_builder = WindowBuilder::new()
        .with_resizable(true)
        .with_title("Test Window")
        .with_min_inner_size(LogicalSize::new(200, 200))
        .with_inner_size(LogicalSize::new(800, 600))
        // .with_window_icon(Some(
        //     winit::window::Icon::from_rgba(iconbytes, icondimension.1, icondimension.0).unwrap(),
        // ))
        .with_always_on_top(false)
        .with_decorations(true);
    let mut resources: Resources = Resources::new();

    // resources.add_font("Bani-Regular", include_bytes!("../assets/fonts/Bani-Regular.ttf"));
    resources.add_texture("rusty", include_bytes!("../assets/textures/rusty.png"));
    

    let (mut game, event_loop) = GameBuilder::new()
        .with_app_info(app_info)
        .with_window_builder(window_builder)
        .with_resources(resources)
        .build();
    
    let mut dt = Instant::now();

    //objects

    let (playersender, playerreceiver) = channel();
    game.objects.push(playerreceiver);

    let player: Object = Object { position: [0.0, 0.0], size: [0.5, 0.5], rotation: 0.0, color: [1.0, 1.0, 1.0, 1.0], graphics: Some(VisualObject::new(Display::Data).data(Data::square()).texture("rusty")) };

    
    event_loop.run(move |event, _, control_flow|{
        control_flow.set_poll();
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input,
                    ..
                },
                ..
            } => {
                if input.virtual_keycode == Some(VirtualKeyCode::Escape) {
                    control_flow.set_exit();
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            },
            Event::RedrawEventsCleared => {
                playersender.send(player.clone()).unwrap();
                game.update();
                let delta_time = dt.elapsed().as_secs_f64();
                dt = Instant::now();
                println!("FPS: {},\nDT: {}\n", 1.0 / delta_time, delta_time);
            },
            _ => ()
        }
    });
}