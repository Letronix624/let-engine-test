use std::{collections::HashSet, time::Duration};

use let_engine::prelude::{
    gpu::{VulkanTypes, model::ModelId},
    *,
};
use let_engine_widgets::labels::{Label, LabelCreateInfo, Labelifier};
const TICK_SPEED: f32 = 1.0 / 180.0;

type Ctx<'a> = EngineContext<'a>;

fn main() {
    let window_builder = WindowBuilder::new()
        .resizable(true)
        .title("test 1")
        .min_inner_size(uvec2(150, 150))
        .inner_size(uvec2(1000, 700))
        // .clear_color([0.3, 0.3, 0.3, 0.8])
        .decorations(true);

    let tick_settings_builder = TickSettingsBuilder::default()
        .tick_wait(Duration::from_secs_f32(TICK_SPEED))
        .build()
        .unwrap();

    let_engine::start(
        EngineSettings::default()
            .window(window_builder)
            .tick_system(tick_settings_builder),
        Game::new,
    )
    .unwrap();
}

struct Game {
    txt: String,
    last: bool,
    last2: bool,
    right: bool,
    mouse_lock: Vec2,
    camera_lock: Vec2,
    egui_focused: bool,
    fixed: bool,
    color: Color,
    object_transform: Transform,
    rotation: f32,
    select: bool,
    selected_object: Option<ObjectId>,
    targeted_object: Option<ObjectId>,
    spawned_objects: HashSet<ObjectId>,
    place_indicator: ObjectId,
    square: Appearance<VulkanTypes>,
    labelifier: Labelifier<VulkanTypes>,
    rtext: Label<VulkanTypes>,
    gtext: Label<VulkanTypes>,
    btext: Label<VulkanTypes>,
    arrow: ObjectId,
    arrow_model: ModelId<Vec2>,
    fps_cap: f64,
}

impl Game {
    pub fn new(ctx: Ctx) -> Result<Self, ()> {
        ctx.scene.root_view_mut().set_scaling(CameraScaling::Expand);
        ctx.scene.root_view_mut().camera_mut().size = Vec2::splat(0.001);
        ctx.gpu
            .settings_mut(|settings| settings.clear_color = Color::from_rgb(0.35, 0.3, 0.31));

        let place_indicator_material = ctx
            .gpu
            .load_material::<Vec2>(&Material::new(
                MaterialSettingsBuilder::default()
                    .topology(Topology::LineStrip)
                    .line_width(2.0)
                    .build()
                    .unwrap(),
                GraphicsShaders::new_default(),
            ))
            .unwrap();

        let indicator_model = ctx
            .gpu
            .load_model(&model!(
                vec![
                    vec2(-1.0, -1.0),
                    vec2(1.0, -1.0),
                    vec2(1.0, 1.0),
                    vec2(-1.0, 1.0),
                ],
                vec![0, 1, 2, 3, 0]
            ))
            .unwrap();
        let arrow_model = ctx
            .gpu
            .load_model(&model!(
                vec![
                    vec2(0.0, 0.0),    //pos from
                    vec2(1.0, 0.0),    //pos length pythagoras
                    vec2(0.95, 0.02),  //left arrow piece
                    vec2(0.95, -0.02), //right arrow piece
                ],
                vec![0, 1, 2, 3, 1]
            ))
            .unwrap();

        let color = Color::from_rgba(0.7, 0.3, 0.3, 1.0); // default color
        let color_buffer = ctx
            .gpu
            .load_buffer(&Buffer::from_data(
                BufferUsage::Uniform,
                BufferAccess::Fixed,
                color,
            ))
            .unwrap();

        let place_indicator = ObjectBuilder::new(
            AppearanceBuilder::default()
                .material(place_indicator_material)
                .model(indicator_model)
                .descriptors(&[
                    (Location::new(0, 0), Descriptor::Mvp),
                    (Location::new(1, 0), Descriptor::buffer(color_buffer)),
                ])
                .build(&ctx.gpu)
                .unwrap(),
        );

        let place_indicator = ctx
            .scene
            .add_object(ctx.scene.root_layer_id(), place_indicator)
            .unwrap();

        let arrow = ObjectBuilder::new(
            AppearanceBuilder::default()
                .visible(false)
                .material(place_indicator_material)
                .model(arrow_model)
                .descriptors(&[
                    (Location::new(0, 0), Descriptor::Mvp),
                    (Location::new(1, 0), Descriptor::buffer(color_buffer)),
                ])
                .build(&ctx.gpu)
                .unwrap(),
        );
        let arrow = ctx
            .scene
            .add_object(ctx.scene.root_layer_id(), arrow)
            .unwrap();

        let last = false;
        let last2 = false;
        let right = false;
        let mouse_lock = Vec2::ZERO;
        let camera_lock = Vec2::ZERO;
        let egui_focused = false;
        ctx.scene
            .root_layer_mut()
            .set_physics_parameters(IntegrationParameters {
                dt: TICK_SPEED,
                normalized_allowed_linear_error: 0.0001,
                normalized_prediction_distance: 0.001,
                ..Default::default()
            });

        let fixed = false;

        let object_transform: Transform = (vec2(0.0, 0.0), vec2(0.07, 0.07), 0.0).into();
        let rotation: f32 = 0.0;

        let select = false;
        let selected_object = None;
        let targeted_object = None;
        let mut spawned_objects = HashSet::new();
        let txt = String::from(
            "Left mouse button: spawn object\rRight mouse button: remove object\rMiddle mouse: Zoom and pan\rEdit this text with the keyboard.",
        );
        let mut labelifier = Labelifier::new(&ctx.gpu).unwrap();

        let font = labelifier
            .font_from_vec(
                let_engine::asset_system::asset("fonts/Px437_CL_Stingray_8x16.ttf")
                    .unwrap()
                    .to_vec(),
            )
            .unwrap();
        let fsize = 35.0;
        let rtext = Label::new(
            LabelCreateInfo {
                transform: Transform::with_size(Vec2::splat(0.001)),
                text_color: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
                text: txt.clone(),
                scale: Vec2::splat(fsize),
                extent: UVec2::splat(2000),
                font,
                align: Direction::Nw,
            },
            &mut labelifier,
            &ctx.gpu,
        )
        .unwrap();
        let gtext = Label::new(
            LabelCreateInfo {
                transform: Transform::with_size(Vec2::splat(0.001)),
                text_color: Color::from_rgba(0.0, 1.0, 0.0, 1.0),
                text: txt.clone(),
                scale: Vec2::splat(fsize),
                extent: UVec2::splat(2000),
                font,
                align: Direction::Center,
            },
            &mut labelifier,
            &ctx.gpu,
        )
        .unwrap();
        let btext = Label::new(
            LabelCreateInfo {
                transform: Transform::with_size(Vec2::splat(0.001)),
                text_color: Color::from_rgba(0.0, 0.0, 1.0, 1.0),
                text: txt.clone(),
                scale: Vec2::splat(fsize),
                extent: UVec2::splat(2000),
                font,
                align: Direction::So,
            },
            &mut labelifier,
            &ctx.gpu,
        )
        .unwrap();

        ctx.scene
            .add_object(
                ctx.scene.root_layer_id(),
                ObjectBuilder::new(rtext.appearance().build(&ctx.gpu).unwrap()),
            )
            .unwrap();
        ctx.scene
            .add_object(
                ctx.scene.root_layer_id(),
                ObjectBuilder::new(gtext.appearance().build(&ctx.gpu).unwrap()),
            )
            .unwrap();
        ctx.scene
            .add_object(
                ctx.scene.root_layer_id(),
                ObjectBuilder::new(btext.appearance().build(&ctx.gpu).unwrap()),
            )
            .unwrap();

        let rusty = ctx
            .gpu
            .load_texture(
                &Texture::from_bytes(
                    let_engine::asset_system::asset("textures/twister_tex.png")
                        .unwrap()
                        .to_vec(),
                    ImageFormat::Png,
                    TextureSettingsBuilder::default()
                        .format(Format::Rgba8Unorm)
                        .build()
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap();

        let square_material = ctx
            .gpu
            .load_material::<TVert>(&Material::default_textured())
            .unwrap();

        let square = Model::new(
            vec![
                tvert(1.0, 1.0, 1.0, 1.0),
                tvert(1.0, -1.0, 1.0, -1.0),
                tvert(-1.0, 1.0, -1.0, 1.0),
                tvert(-1.0, 1.0, -1.0, 1.0),
                tvert(1.0, -1.0, 1.0, -1.0),
                tvert(-1.0, -1.0, -1.0, -1.0),
            ],
            BufferAccess::Fixed,
        );

        let square = AppearanceBuilder::default()
            .model(ctx.gpu.load_model::<TVert>(&square).unwrap())
            .material(square_material)
            .descriptors(&[
                (Location::new(0, 0), Descriptor::Mvp),
                (Location::new(1, 0), Descriptor::buffer(color_buffer)),
                (Location::new(2, 0), Descriptor::Texture(rusty)),
            ])
            .build(&ctx.gpu)
            .unwrap();
        // ::new_instanced(Some(Model::Square), Some(rusty));

        let mut platform = ObjectBuilder::new(square.clone());
        platform.transform.size = vec2(5.0, 0.1);
        platform.transform.position = vec2(0.0, 1.0);

        platform.set_collider(Some(
            ColliderBuilder::square(5.0, 0.1).restitution(0.0).build(),
        ));
        platform.set_rigid_body(Some(RigidBodyBuilder::fixed().build()));

        let platform = ctx
            .scene
            .add_object(ctx.scene.root_layer_id(), platform)
            .unwrap();
        spawned_objects.insert(platform);

        Ok(Self {
            txt,
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
            fps_cap: 180.0,
            labelifier,
        })
    }
}

impl let_engine::Game for Game {
    fn update(&mut self, ctx: Ctx) -> Result<(), ()> {
        self.labelifier.update(&ctx.gpu).unwrap();
        if self.egui_focused {
            return Ok(());
        }

        let cursor_to_world = ctx.input.cursor_to_world(ctx.scene.root_view());

        if !self.select {
            self.object_transform.position = cursor_to_world;
            ctx.scene
                .object_mut(self.place_indicator)
                .unwrap()
                .transform = Transform::with_position_rotation(
                self.object_transform.position,
                self.object_transform.rotation,
            );

            {
                let apperance = &mut ctx
                    .scene
                    .object_mut(self.place_indicator)
                    .unwrap()
                    .appearance;
                apperance.set_visible(true);
                apperance.transform_mut().size = self.object_transform.size;
            }
            {
                if ctx.input.mouse_down(&MouseButton::Left) && !self.last {
                    let mut object = ObjectBuilder::new(self.square.clone());
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
                    // object
                    //     .appearance
                    //     .set_layer(self.spawned_objects.len() as u32 % 4)
                    //     .unwrap();
                    object.transform = self.object_transform;
                    object.transform.size = vec2(1.0, 1.0);
                    let mut transform = *object.appearance.transform();
                    transform.size = self.object_transform.size;
                    object.appearance.set_transform(transform);
                    let object = ctx
                        .scene
                        .add_object(ctx.scene.root_layer_id(), object)
                        .unwrap();
                    self.spawned_objects.insert(object);
                }
                self.last = ctx.input.mouse_down(&MouseButton::Left);

                if ctx.input.mouse_down(&MouseButton::Right) && !self.last2 {
                    let ids = ctx.scene.root_layer().intersections_with_ray(
                        ctx.input.cursor_to_world(ctx.scene.root_view()),
                        vec2(0.0, 0.0),
                        0.0,
                        true,
                    );
                    for id in ids {
                        if self.spawned_objects.remove(&id) {
                            ctx.scene.remove_object(id);
                        }
                    }
                }
                self.last2 = ctx.input.mouse_down(&MouseButton::Right);
            }
        } else {
            if ctx.input.mouse_down(&MouseButton::Left)
                && !self.last
                && let Some(id) = ctx.scene.root_layer().cast_ray(
                    ctx.input.cursor_to_world(ctx.scene.root_view()),
                    vec2(0.0, 0.0),
                    0.0,
                    true,
                )
            {
                self.selected_object = self.spawned_objects.get(&id).cloned();
            }
            if ctx.input.mouse_down(&MouseButton::Left) {
                ctx.scene
                    .object_mut(self.arrow)
                    .unwrap()
                    .appearance
                    .set_visible(true);
                if let Some(id) = self.selected_object {
                    let object = ctx.scene.object(id).unwrap();
                    ctx.scene.object_mut(self.arrow).unwrap().transform.position =
                        object.transform.position;
                    let arrow = ctx.scene.object(self.arrow).unwrap();
                    let (length, angle) = if let Some(second_object) =
                        ctx.scene.root_layer().cast_ray(
                            ctx.input.cursor_to_world(ctx.scene.root_view()),
                            vec2(0.0, 0.0),
                            0.0,
                            true,
                        ) {
                        let object2 = ctx.scene.object(second_object).unwrap();
                        let position = object2.transform.position;
                        self.targeted_object = Some(second_object);
                        (
                            arrow.transform.position.distance(position),
                            angle_between(arrow.transform.position, position),
                        )
                    } else {
                        self.targeted_object = None;
                        (
                            arrow.transform.position.distance(cursor_to_world),
                            angle_between(arrow.transform.position, cursor_to_world),
                        )
                    };
                    if length == 0.0 {
                        ctx.scene
                            .object_mut(self.arrow)
                            .unwrap()
                            .appearance
                            .set_visible(false);
                    };
                    {
                        ctx.gpu
                            .model(self.arrow_model)
                            .unwrap()
                            .write_vertices(
                                |vertices| {
                                    vertices[1..3].copy_from_slice(&[
                                        vec2(length, 0.0),
                                        vec2(length - 0.05, 0.02),
                                        vec2(length - 0.05, -0.02),
                                    ]);
                                },
                                4,
                            )
                            .unwrap();
                        ctx.scene.object_mut(self.arrow).unwrap().transform.rotation = angle;
                    }
                } else {
                    ctx.scene
                        .object_mut(self.arrow)
                        .unwrap()
                        .appearance
                        .set_visible(false);
                };
            } else {
                ctx.scene
                    .object_mut(self.arrow)
                    .unwrap()
                    .appearance
                    .set_visible(false);
            }
            if !ctx.input.mouse_down(&MouseButton::Left)
                && self.last
                && let (Some(id), Some(target_id)) = (&self.selected_object, &self.targeted_object)
                && id != target_id
            {
                let object = ctx.scene.object(*id).unwrap();
                let target_object = ctx.scene.object(*target_id).unwrap();
                let _handle = ctx.scene.add_joint(
                    *id,
                    *target_id,
                    FixedJointBuilder::new()
                        .local_anchor1(target_object.transform.position - object.transform.position)
                        .local_anchor2(vec2(0.0, 0.0)),
                    true,
                );
                self.targeted_object = None;
            }
            self.last = ctx.input.mouse_down(&MouseButton::Left);
            if let Some(id) = self.selected_object {
                let object_transform = ctx.scene.object(id).unwrap().transform;
                let place_indicator = ctx.scene.object_mut(self.place_indicator).unwrap();
                place_indicator.appearance.set_visible(true);
                place_indicator.transform = object_transform;
                place_indicator.appearance.transform_mut().size = object_transform.size;
            } else {
                self.selected_object = None;
                ctx.scene
                    .object_mut(self.place_indicator)
                    .unwrap()
                    .appearance
                    .set_visible(false);
            }
        }

        {
            let camera = ctx.scene.root_view().camera();
            let cp = ctx.input.scaled_cursor(CameraScaling::Expand);
            if ctx.input.mouse_down(&MouseButton::Middle) && !self.right {
                self.mouse_lock = cp;
                self.camera_lock = camera.position;
            }
            if ctx.input.mouse_down(&MouseButton::Middle) {
                let shift = vec2(
                    (self.mouse_lock[0] - cp[0]) * camera.size.x + self.camera_lock[0],
                    (self.mouse_lock[1] - cp[1]) * camera.size.y + self.camera_lock[1],
                );
                //times camera mode please
                ctx.scene.root_view_mut().camera_mut().position = shift;
            }
            self.right = ctx.input.mouse_down(&MouseButton::Middle);
        }
        Ok(())
    }

    fn window(&mut self, ctx: Ctx, event: events::WindowEvent) -> Result<(), ()> {
        match event {
            WindowEvent::CloseRequested => ctx.exit(),
            WindowEvent::MouseWheel(ScrollDelta::LineDelta(delta)) => {
                let camera = ctx.scene.root_view_mut().camera_mut();
                camera.size = camera.size - Vec2::splat(delta.y) * camera.size * 0.1;
            }
            _ => (),
        }
        Ok(())
    }

    fn input(&mut self, ctx: Ctx, event: InputEvent) -> Result<(), ()> {
        if let InputEvent::KeyboardInput { input } = event {
            match input.key {
                Key::Named(NamedKey::Escape) => {
                    ctx.exit();
                }
                Key::Named(NamedKey::F11) => {
                    if input.state == ElementState::Released {
                        let window = ctx.window().unwrap();
                        window.set_fullscreen(if window.fullscreen().is_some() {
                            None
                        } else {
                            Some(Fullscreen::Borderless(None))
                        });
                    }
                }
                _ => (),
            }
            if let Some(text) = input.text
                && let ElementState::Pressed = input.state
            {
                if self.egui_focused {
                    return Ok(());
                };
                match &*text {
                    "\u{8}" => {
                        self.txt.pop();
                    }
                    _ if text != "\u{7f}" => self.txt += &text,
                    _ => {}
                }
                self.rtext.update_text(self.txt.clone()).unwrap();
                self.gtext.update_text(self.txt.clone()).unwrap();
                self.btext.update_text(self.txt.clone()).unwrap();
            }
        }
        Ok(())
    }

    fn egui(&mut self, ctx: Ctx, ectx: egui::Context) -> Result<(), ()> {
        egui::TopBottomPanel::top("test").show(&ectx, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.fixed, "Anchored");
                let mut time_scale = ctx.time.scale();
                let response =
                    ui.add(egui::Slider::new(&mut time_scale, 0.0..=2.0).text("Time scale"));
                if response.changed() {
                    ctx.time.set_scale(time_scale);
                }
                ui.add(
                    egui::Slider::new(&mut self.object_transform.size.x, 0.01..=1.0).text("Size X"),
                );
                ui.add(
                    egui::Slider::new(&mut self.object_transform.size.y, 0.01..=1.0).text("Size Y"),
                );
                ui.add(egui::Slider::new(&mut self.rotation, 0.0..=90.0).text("Rotation"));
                self.object_transform.rotation = self.rotation.to_radians();
            });
            let mut srgba: [u8; 4] = self.color.map(|x| (x * 255.0) as u8);
            let response = ui.color_edit_button_srgba_unmultiplied(&mut srgba);
            if response.changed() {
                self.color = Color::from(srgba.map(|x| x as f32 / 255.0));
            };

            ui.horizontal(|ui| {
                let response = ui.button(if self.select { "Spawn" } else { "Select" });
                if response.clicked() {
                    self.select = !self.select;
                }
                let text = if let Some(object) = &self.selected_object {
                    format!("Selected Object {:?}", object)
                } else {
                    "Selected None".to_string()
                };
                ui.label(text);
                if ui
                    .add(egui::Slider::new(&mut self.fps_cap, 10.0..=181.0).text("fps cap"))
                    .changed()
                {
                    if self.fps_cap > 180.0 {
                        ctx.time.set_framerate_limit(Duration::ZERO);
                    } else {
                        ctx.time.set_fps_limit(self.fps_cap);
                    }
                };
            });

            ui.label(egui::RichText::new(format!("FPS: {}", ctx.time.fps(),)).monospace());
        });
        self.egui_focused =
            ectx.is_pointer_over_area() || ectx.is_using_pointer() || ectx.wants_keyboard_input();
        Ok(())
    }
}

fn angle_between(x: Vec2, y: Vec2) -> f32 {
    let point = y - x;
    point.y.atan2(point.x)
}
