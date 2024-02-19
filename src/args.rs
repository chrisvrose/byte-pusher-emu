use clap::Parser;

#[derive(Debug,Parser)]
#[command(version, about, long_about = "Byte Pusher Emulator")]
pub struct BytePusherArgs{
    #[arg(short,long)]
    pub file_name:Option<String>
}