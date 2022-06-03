use entrypoint::Result;

fn splash_position() -> Result<(isize, isize)> {
    let monitor = {
        let init_pos = winit::dpi::Position::Logical(winit::dpi::LogicalPosition::new(0.0, 0.0));
        let init_size = winit::dpi::Size::Logical(winit::dpi::LogicalSize::new(1.0, 1.0));
        let event_loop = winit::event_loop::EventLoop::new();
        winit::window::WindowBuilder::new()
            .with_visible(false)
            .with_decorations(false)
            .with_inner_size(init_size)
            .with_position(init_pos)
            .build(&event_loop)?
            .current_monitor()
            .or(event_loop.primary_monitor())
            .or(event_loop.available_monitors().take(1).next())
    };
    let image = &entrypoint::SPLASH_IMAGE;
    let (pos_x, pos_y) = if let Some(monitor) = monitor {
        let size = monitor.size();
        let (width, height) = if cfg!(target_os = "macos") {
            let size = size.to_logical::<f64>(monitor.scale_factor());
            (size.width, size.height)
        } else {
            (size.width as f64, size.height as f64)
        };
        let pos_x = ((width - image.width() as f64) / 2.0) as isize;
        let pos_y = ((height - image.height() as f64) / 2.0) as isize;
        (pos_x, pos_y)
    } else {
        (20, 20)
    };
    Ok((pos_x, pos_y))
}

fn main() {
    let (pos_x, pos_y) = splash_position().unwrap();
    print!("{pos_x} {pos_y}");
}
