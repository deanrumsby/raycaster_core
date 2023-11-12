// use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), EventLoopError> {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Raycaster")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    // let mut pixels = {
    //     let window_size = window.inner_size();
    //     let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    //
    //     Pixels::new(800, 600, surface_texture)?
    // };

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => elwt.exit(),

            // Event::MainEventsCleared => {
            //     pixels.render().unwrap();
            // }
            _ => (),
        }
    })
}
