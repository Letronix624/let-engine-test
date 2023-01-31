use let_engine::*;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;

fn main() {
    let app_info = game::AppInfo {
        AppName: "Test"
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

    game::Game::init(app_info, window_builder);
}