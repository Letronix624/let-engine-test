use let_engine::physics::*;
use let_engine::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::MouseButton;
use winit::event::MouseScrollDelta;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::window::WindowBuilder;

use core::f32::consts::PI;

fn main() {
    let window_builder = WindowBuilder::new()
        .with_resizable(true)
        .with_title("Test Window")
        .with_min_inner_size(PhysicalSize::new(150.0, 150.0))
        .with_inner_size(PhysicalSize::new(1000.0, 700.0))
        .with_decorations(true)
        .with_transparent(true)
        .with_visible(false);
    let (mut game, event_loop) = GameBuilder::new()
        .with_window_builder(window_builder)
        .build();

    let time = game.time.clone();
    let resources = game.resources.clone();
    let input = game.input.clone();

    let font = resources.load_font(include_bytes!("../assets/fonts/Px437_CL_Stingray_8x16.ttf"));
    let layer = game.scene.new_layer();
    let mut txt = String::from("Left mouse button: spawn object\rRight mouse button: remove object\rMiddle mouse: Zoom and pan\rEdit this text with the keyboard.");
    let fsize = 35.0;
    let mut rtext = Label::new(
        &resources,
        &font,
        LabelCreateInfo {
            appearance: Appearance::new_color([1.0, 0.0, 0.0, 1.0])
                .transform(Transform::default().size(vec2(2.0, 2.0))),
            text: txt.clone(),
            scale: vec2(fsize, fsize),
            align: NW,
            ..Default::default()
        },
    );
    let mut gtext = Label::new(
        &resources,
        &font,
        LabelCreateInfo {
            appearance: Appearance::new_color([0.0, 1.0, 0.0, 1.0])
                .transform(Transform::default().size(vec2(2.0, 2.0))),
            text: txt.clone(),
            scale: vec2(fsize, fsize),
            align: CENTER,
            ..Default::default()
        },
    );
    let mut btext = Label::new(
        &resources,
        &font,
        LabelCreateInfo {
            appearance: Appearance::new_color([0.0, 0.0, 1.0, 1.0])
                .transform(Transform::default().size(vec2(2.0, 2.0))),
            text: txt.clone(),
            scale: vec2(fsize, fsize),
            align: SO,
            ..Default::default()
        },
    );
    layer.add_object(None, &mut rtext).unwrap();
    layer.add_object(None, &mut gtext).unwrap();
    layer.add_object(None, &mut btext).unwrap();
    let mut camera = Camera::default();
    camera.camera.mode = CameraScaling::Limited;
    layer.add_object(None, &mut camera).unwrap();
    game.set_clear_background_color([0.3, 0.0, 0.3, 0.1]);
    layer.set_camera(camera.clone());
    resources.get_window().set_visible(true);

    let mut test = Object::default();
    test.appearance = Appearance::new().data(make_circle!(10, 0.5));
    test.appearance.transform.size = vec2(0.1, 0.1);

    test.transform.position = layer.side_to_world(N, (1000.0, 700.0));
    layer.add_object(None, &mut test).unwrap();

    let mut last = false;
    let mut last2 = false;
    let mut right = false;
    let mut mouselock = vec2(0.0, 0.0);
    let mut camera_lock = vec2(0.0, 0.0);
    let mut egui_focused = false;
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        match &event {
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
                        _ if *c != '\u{7f}' => txt.push(*c),
                        _ => {}
                    }
                    rtext.update_text(txt.clone());
                    gtext.update_text(txt.clone());
                    btext.update_text(txt.clone());
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    if let MouseScrollDelta::LineDelta(_, y) = delta {
                        let zoom = camera.camera.zoom;
                        camera.camera.zoom = zoom + *y as f32 * zoom * 0.1;
                    };
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                game.update_gui(|ctx| {
                    egui::TopBottomPanel::top("test").show(&ctx, |ui| {
                        let butt = ui.button(egui::RichText::new("an X").size(50.0));
                        if butt.clicked() {
                            control_flow.set_exit();
                        }
                    });
                    egui_focused = ctx.is_pointer_over_area();
                });
                if egui_focused {
                    return;
                }
                {
                    if input.mouse_down(&MouseButton::Left) && !last {
                        let mut object = ColliderObject::default();
                        object.collider = Some(ColliderBuilder::square(0.03, 0.03).build());
                        object.appearance = Appearance::new().data(Data::square());
                        object.appearance.transform.size = vec2(0.03, 0.03);
                        object.transform.position = input.cursor_to_world(&layer);
                        object.transform.rotation = PI * 0.25;
                        layer.add_object(None, &mut object).unwrap();
                        println!("{}", time.fps());
                    }
                    last = input.mouse_down(&MouseButton::Left);

                    if input.mouse_down(&MouseButton::Right) && !last2 {
                        let id = layer.query_collider_at(input.cursor_to_world(&layer));
                        if let Some(id) = id {
                            layer.remove_object(id).unwrap();
                        }
                    }
                    last2 = input.mouse_down(&MouseButton::Right);
                }
                {
                    let cp = input.scaled_cursor(&layer);
                    if input.mouse_down(&MouseButton::Middle) && !right {
                        mouselock = cp;
                        camera_lock = camera.transform.position;
                    }
                    if input.mouse_down(&MouseButton::Middle) {
                        let zoom = camera.camera.zoom;
                        let shift = vec2(
                            (mouselock[0] - cp[0]) * (1.0 / zoom) * 0.5 + camera_lock[0],
                            (mouselock[1] - cp[1]) * (1.0 / zoom) * 0.5 + camera_lock[1],
                        );
                        //times camera mode please
                        camera.transform.position = shift;
                    }
                    right = input.mouse_down(&MouseButton::Middle);
                }
                layer.set_camera(camera.clone());
            }
            _ => (),
        }
        game.update(&event);
    });
}
