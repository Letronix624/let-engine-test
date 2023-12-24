use std::{sync::Arc, time::Duration};

use hashbrown::HashMap;
use let_engine::prelude::*;
const TICK_SPEED: f32 = 1.0 / 180.0;

fn main() {
    let window_builder = WindowBuilder::new()
        .resizable(true)
        .title("Diarrhée")
        .min_inner_size(vec2(150.0, 150.0))
        .inner_size(vec2(1000.0, 700.0))
        .clear_color([0.3, 0.3, 0.3, 0.8])
        .decorations(true);
    let tick_reporter = TickReporter::new();

    let tick_settings_builder = TickSettingsBuilder::default()
        .reporter(tick_reporter.clone())
        .tick_wait(Duration::from_secs_f32(TICK_SPEED))
        .build()
        .unwrap();

    let engine = Engine::new(
        EngineSettingsBuilder::default()
            .window_settings(window_builder)
            .tick_settings(tick_settings_builder)
            .build()
            .unwrap(),
    )
    .unwrap();

    let game = Game::new(tick_reporter);

    engine.start(game);
}

struct Game {
    layer: Arc<Layer>,
    exit: bool,

    txt: String,
    last: bool,
    last2: bool,
    right: bool,
    mouse_lock: Vec2,
    camera_lock: Vec2,
    egui_focused: bool,
    fixed: bool,
    color: [f32; 4],
    object_transform: Transform,
    rotation: f32,
    select: bool,
    selected_object: Option<Object>,
    targeted_object: Option<Object>,
    spawned_objects: HashMap<usize, Object>,
    place_indicator: Object,
    square: Appearance,
    rtext: Label,
    gtext: Label,
    btext: Label,
    arrow: Object,
    arrow_model: ModelData,
    camera: Object,
    tick_reporter: TickReporter,
}

impl Game {
    pub fn new(tick_reporter: TickReporter) -> Self {
        let layer = SCENE.new_layer();
        let font =
            Font::from_bytes(include_bytes!("../assets/fonts/Px437_CL_Stingray_8x16.ttf")).unwrap();
        let txt = String::from("Left mouse button: spawn object\rRight mouse button: remove object\rMiddle mouse: Zoom and pan\rEdit this text with the keyboard.");
        let fsize = 35.0;
        let mut rtext = Label::new(
            &font,
            LabelCreateInfo {
                appearance: Appearance::new()
                    .color([1.0, 0.0, 0.0, 1.0])
                    .transform(Transform::default().size(vec2(2.0, 2.0))),
                text: txt.clone(),
                scale: vec2(fsize, fsize),
                align: NW,
                ..Default::default()
            },
        );
        let mut gtext = Label::new(
            &font,
            LabelCreateInfo {
                appearance: Appearance::new()
                    .color([0.0, 1.0, 0.0, 1.0])
                    .transform(Transform::default().size(vec2(2.0, 2.0))),
                text: txt.clone(),
                scale: vec2(fsize, fsize),
                align: CENTER,
                ..Default::default()
            },
        );
        let mut btext = Label::new(
            &font,
            LabelCreateInfo {
                appearance: Appearance::new()
                    .color([0.0, 0.0, 1.0, 1.0])
                    .transform(Transform::default().size(vec2(2.0, 2.0))),
                text: txt.clone(),
                scale: vec2(fsize, fsize),
                align: SO,
                ..Default::default()
            },
        );
        rtext.init(&layer);
        gtext.init(&layer);
        btext.init(&layer);
        let mut camera = Object::default();
        camera.appearance.set_visible(false);
        layer.set_camera_settings(CameraSettings::default().mode(CameraScaling::Expand));
        camera.init(&layer);
        //game.set_clear_background_color([0.35, 0.3, 0.31, 1.0]);
        layer.set_camera(&camera).unwrap();

        let place_indicator_material = Material::new(
            materials::MaterialSettingsBuilder::default()
                .topology(materials::Topology::LineStrip)
                .line_width(2.0)
                .build()
                .unwrap(),
        )
        .unwrap();

        let indicator_model = ModelData::new(Data {
            vertices: vec![
                vert(-1.0, -1.0),
                vert(1.0, -1.0),
                vert(1.0, 1.0),
                vert(-1.0, 1.0),
            ],
            indices: vec![0, 1, 2, 3, 0],
        })
        .unwrap();
        let arrow_model = ModelData::new(Data {
            vertices: vec![
                vert(0.0, 0.0),    //pos from
                vert(1.0, 0.0),    //pos length pythagoras
                vert(0.95, 0.02),  //left arrow piece
                vert(0.95, -0.02), //right arrow piece
            ],
            indices: vec![0, 1, 2, 3, 1],
        })
        .unwrap();

        let mut place_indicator = Object::default();
        place_indicator.appearance = Appearance::new()
            .material(Some(place_indicator_material.clone()))
            .model(Model::Custom(indicator_model));

        place_indicator.init(&layer);

        let mut arrow = Object::default();
        arrow.appearance = Appearance::new()
            .material(Some(place_indicator_material))
            .model(Model::Custom(arrow_model.clone()))
            .visible(false);
        arrow.init(&layer);

        let rusty = Material::new_default_textured_instance(
            &Texture::from_bytes(
                include_bytes!("../assets/textures/twister_tex.png"),
                ImageFormat::Png,
                4,
                TextureSettings::default(),
            )
            .unwrap(),
        );

        let square = Appearance::new_instanced(Model::Square, Some(rusty));

        let mut platform = Object::default();
        platform.appearance = square.clone().color([0.7, 0.7, 0.7, 1.0]);
        platform.transform.size = vec2(5.0, 0.1);
        platform.transform.position = layer.side_to_world(S, vec2(1000.0, 700.0));

        platform.set_collider(Some(
            ColliderBuilder::square(5.0, 0.1).restitution(0.0).build(),
        ));
        platform.set_rigid_body(Some(RigidBodyBuilder::fixed().build()));

        platform.init(&layer);

        let last = false;
        let last2 = false;
        let right = false;
        let mouse_lock = vec2(0.0, 0.0);
        let camera_lock = vec2(0.0, 0.0);
        let egui_focused = false;
        let physics_params = IntegrationParameters {
            dt: TICK_SPEED,
            max_stabilization_iterations: 3,
            allowed_linear_error: 0.0001,
            prediction_distance: 0.001,
            ..Default::default()
        };
        layer.set_physics_parameters(physics_params);

        let fixed = false;

        let color: [f32; 4] = [0.7, 0.3, 0.3, 1.0]; // default color

        let object_transform: Transform = (vec2(0.0, 0.0), vec2(0.07, 0.07), 0.0).into();
        let rotation: f32 = 0.0;

        let select = false;
        let selected_object: Option<Object> = None;
        let targeted_object: Option<Object> = None;
        let mut spawned_objects: HashMap<usize, Object> = HashMap::new();
        spawned_objects.insert(platform.id(), platform);
        Self {
            layer,
            txt,
            exit: false,
            last,
            last2,
            right,
            mouse_lock,
            camera_lock,
            egui_focused,
            fixed,
            color,
            object_transform,
            rotation,
            select,
            selected_object,
            targeted_object,
            spawned_objects,
            place_indicator,
            square,
            rtext,
            gtext,
            btext,
            arrow,
            arrow_model,
            camera,
            tick_reporter,
        }
    }
}

impl let_engine::Game for Game {
    fn exit(&self) -> bool {
        self.exit
    }
    fn update(&mut self) {
        if self.egui_focused {
            return;
        }

        let cursor_to_world = INPUT.cursor_to_world(&self.layer);

        if !self.select {
            self.object_transform.position = cursor_to_world;
            self.place_indicator.transform = self.object_transform.size(vec2(1.0, 1.0));
            let appearance = self.place_indicator.appearance().clone();
            self.place_indicator.appearance = appearance.color(self.color).visible(true).transform(
                self.place_indicator
                    .appearance
                    .get_transform()
                    .size(self.object_transform.size),
            );
            {
                if INPUT.mouse_down(&MouseButton::Left) && !self.last {
                    let mut object = Object::default();
                    object.set_collider(Some(
                        ColliderBuilder::square(
                            self.object_transform.size.x,
                            self.object_transform.size.y,
                        ) //trimesh(shape_data.clone())
                        .restitution(0.0)
                        .restitution_combine_rule(CoefficientCombineRule::Min)
                        .build(),
                    ));
                    let rigid_body_type = if self.fixed {
                        RigidBodyType::Fixed
                    } else {
                        RigidBodyType::Dynamic
                    };
                    object.set_rigid_body(Some(RigidBodyBuilder::new(rigid_body_type).build()));
                    object.appearance = self.square.clone().color(self.color);
                    object
                        .appearance
                        .set_layer(self.spawned_objects.len() as u32 % 3)
                        .unwrap();
                    object.transform = self.object_transform;
                    object.transform.size = vec2(1.0, 1.0);
                    object.appearance.set_transform(
                        object
                            .appearance()
                            .clone()
                            .get_transform()
                            .size(self.object_transform.size),
                    );
                    object.init(&self.layer);
                    self.spawned_objects.insert(object.id(), object);
                }
                self.last = INPUT.mouse_down(&MouseButton::Left);

                if INPUT.mouse_down(&MouseButton::Right) && !self.last2 {
                    let ids = self.layer.intersections_with_ray(
                        INPUT.cursor_to_world(&self.layer),
                        vec2(0.0, 0.0),
                        0.0,
                        true,
                    );
                    for id in ids {
                        self.spawned_objects.get_mut(&id).unwrap().remove().unwrap();
                        self.spawned_objects.remove(&id);
                    }
                }
                self.last2 = INPUT.mouse_down(&MouseButton::Right);
            }
        } else {
            if INPUT.mouse_down(&MouseButton::Left) && !self.last {
                if let Some(id) = self.layer.cast_ray(
                    INPUT.cursor_to_world(&self.layer),
                    vec2(0.0, 0.0),
                    0.0,
                    true,
                ) {
                    self.selected_object = self.spawned_objects.get(&id).cloned();
                }
            }
            if INPUT.mouse_down(&MouseButton::Left) {
                self.arrow.appearance.set_visible(true);
                if let Some(object) = &mut self.selected_object {
                    object.update().unwrap();
                    self.arrow.transform.position = object.transform.position;
                    let (length, angle) = if let Some(second_object) = self.layer.cast_ray(
                        INPUT.cursor_to_world(&self.layer),
                        vec2(0.0, 0.0),
                        0.0,
                        true,
                    ) {
                        let object2 = self.spawned_objects.get_mut(&second_object).unwrap();
                        object2.update().unwrap();
                        let position = object2.transform.position;
                        self.targeted_object = Some(object2.clone());
                        (
                            self.arrow.transform.position.distance(position),
                            angle_between(self.arrow.transform.position, position),
                        )
                    } else {
                        self.targeted_object = None;
                        (
                            self.arrow.transform.position.distance(cursor_to_world),
                            angle_between(self.arrow.transform.position, cursor_to_world),
                        )
                    };
                    if length == 0.0 {
                        self.arrow.appearance.set_visible(false);
                    };
                    let mut data = self.arrow_model.get_data().clone();
                    data.vertices[1] = vert(length, 0.0);
                    data.vertices[2] = vert(length - 0.05, 0.02);
                    data.vertices[3] = vert(length - 0.05, -0.02);
                    self.arrow_model = ModelData::new(data).unwrap();
                    self.arrow
                        .appearance
                        .set_model(Model::Custom(self.arrow_model.clone()));
                    self.arrow.transform.rotation = angle;
                } else {
                    self.arrow.appearance.set_visible(false);
                };
            } else {
                self.arrow.appearance.set_visible(false);
            }
            if !INPUT.mouse_down(&MouseButton::Left) && self.last {
                if let (Some(object), Some(target_object)) =
                    (&self.selected_object, &self.targeted_object)
                {
                    if object.id() != target_object.id() {
                        let _handle = self.layer.add_joint(
                            object,
                            target_object,
                            FixedJointBuilder::new()
                                .local_anchor1(
                                    target_object.transform.position - object.transform.position,
                                )
                                .local_anchor2(vec2(0.0, 0.0)),
                            true,
                        );
                        self.targeted_object = None;
                    }
                }
            }
            self.last = INPUT.mouse_down(&MouseButton::Left);
            if let Some(object) = &mut self.selected_object {
                object.update().unwrap();
                self.place_indicator.transform = object.transform;
                let appearance = self
                    .place_indicator
                    .appearance()
                    .clone()
                    .color([1.0; 4])
                    .visible(true)
                    .transform(
                        self.place_indicator
                            .appearance()
                            .get_transform()
                            .size(object.appearance.get_transform().size),
                    );
                self.place_indicator.appearance = appearance;
            } else {
                self.selected_object = None;
                self.place_indicator.appearance =
                    self.place_indicator.appearance().clone().visible(false);
            }
        }
        self.layer.move_to_top(&self.place_indicator).unwrap();
        self.layer.move_to_top(&self.arrow).unwrap();
        self.place_indicator.sync().unwrap();
        self.arrow.sync().unwrap();
        {
            let cp = INPUT.scaled_cursor(&self.layer);
            if INPUT.mouse_down(&MouseButton::Middle) && !self.right {
                self.mouse_lock = cp;
                self.camera_lock = self.camera.transform.position;
            }
            if INPUT.mouse_down(&MouseButton::Middle) {
                let zoom = self.layer.zoom();
                let shift = vec2(
                    (self.mouse_lock[0] - cp[0]) * (1.0 / zoom) * 0.5 + self.camera_lock[0],
                    (self.mouse_lock[1] - cp[1]) * (1.0 / zoom) * 0.5 + self.camera_lock[1],
                );
                //times camera mode please
                self.camera.transform.position = shift;
            }
            self.right = INPUT.mouse_down(&MouseButton::Middle);
            self.camera.sync().unwrap();
        }
    }
    fn event(&mut self, event: Event) {
        match event {
            Event::Window(event) => match event {
                WindowEvent::CloseRequested => self.exit = true,
                WindowEvent::MouseWheel(ScrollDelta::LineDelta(delta)) => {
                    let zoom = self.layer.zoom();
                    self.layer.set_zoom(zoom + delta.y * zoom * 0.1);
                }
                _ => (),
            },
            Event::Egui(ctx) => {
                egui::TopBottomPanel::top("test").show(&ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.fixed, "Anchored");
                        let mut time_scale = TIME.scale();
                        let response = ui
                            .add(egui::Slider::new(&mut time_scale, 0.0..=2.0).text("Time scale"));
                        if response.changed() {
                            TIME.set_scale(time_scale);
                        }
                        ui.add(
                            egui::Slider::new(&mut self.object_transform.size.x, 0.01..=1.0)
                                .text("Size X"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.object_transform.size.y, 0.01..=1.0)
                                .text("Size Y"),
                        );
                        ui.add(egui::Slider::new(&mut self.rotation, 0.0..=90.0).text("Rotation"));
                        self.object_transform.rotation = self.rotation.to_radians();
                    });
                    let mut srgba: [u8; 4] = self.color.map(|x| (x * 255.0) as u8);
                    let response = ui.color_edit_button_srgba_unmultiplied(&mut srgba);
                    if response.changed() {
                        self.color = srgba.map(|x| x as f32 / 255.0);
                    };

                    ui.horizontal(|ui| {
                        let response = ui.button(if self.select { "Spawn" } else { "Select" });
                        if response.clicked() {
                            self.select = !self.select;
                        }
                        let text = if let Some(object) = &self.selected_object {
                            format!("Selected Object {}", object.id())
                        } else {
                            "Selected None".to_string()
                        };
                        ui.label(text);
                    });

                    ui.label(
                        egui::RichText::new(format!(
                            "FPS: {}\nTick_report: {:#?}",
                            TIME.fps(),
                            self.tick_reporter
                        ))
                        .monospace(),
                    );
                });
                self.egui_focused = ctx.is_pointer_over_area()
                    || ctx.is_using_pointer()
                    || ctx.wants_keyboard_input();
            }
            Event::Input(input) => match input {
                InputEvent::KeyboardInput { input } => {
                    if input.keycode == Some(VirtualKeyCode::Escape) {
                        self.exit = true;
                    } else if input.keycode == Some(VirtualKeyCode::F11)
                        && input.state == ElementState::Released
                    {
                        let window = SETTINGS.window().unwrap();
                        window.set_fullscreen(!window.fullscreen());
                    }
                }
                InputEvent::ReceivedCharacter(c) => {
                    if self.egui_focused {
                        return;
                    };
                    match c {
                        '\u{8}' => {
                            self.txt.pop();
                        }
                        _ if c != '\u{7f}' => self.txt.push(c),
                        _ => {}
                    }
                    self.rtext.update_text(self.txt.clone());
                    self.gtext.update_text(self.txt.clone());
                    self.btext.update_text(self.txt.clone());
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn angle_between(x: Vec2, y: Vec2) -> f32 {
    let point = y - x;
    point.y.atan2(point.x)
}
