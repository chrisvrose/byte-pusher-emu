use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, author)]
pub struct BytePusherArgs {
    #[arg(short, long, help = "ROM file to load")]
    pub file_name: Option<String>,
    #[arg(short, long, help = "Scale at which to draw", default_value_t = 2.0)]
    pub draw_scale: f32,
}