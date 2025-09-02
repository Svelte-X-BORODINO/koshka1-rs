#[path = "./video.rs"]
mod video;
use video::VideoController;

#[path = "./cpu.rs"]
mod cpu;
use cpu::KoshkaCPU;

mod progs;
use crate::progs::minimal_os::MinimalOS; 

fn main() {
    KoshkaCPU::new();
    VideoController::dispd("CPU Initialized");
    VideoController::new();
    VideoController::dispd("Video Initialized");
    MinimalOS::init();
}

