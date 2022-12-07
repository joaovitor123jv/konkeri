use clap::Parser;

#[derive(Parser, Debug)]
pub struct CliArgs {
    ///	The window width
    #[arg(long, default_value_t = 800)]
    pub width: u32,

    ///	The window height
    #[arg(long, default_value_t = 600)]
    pub height: u32,

    ///	The viewport width
    #[arg(long, default_value_t = 1050)]
    pub viewport_width: u32,

    ///	The viewport height
    #[arg(long, default_value_t = 800)]
    pub viewport_height: u32,

    ///	The FPS limit
    #[arg(long, default_value_t = 60)]
    pub fps: u32,

    ///	The zoom level
    #[arg(long, default_value_t = 1.0)]
    pub zoom: f32,
}
