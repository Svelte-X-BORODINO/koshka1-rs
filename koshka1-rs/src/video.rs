pub struct VideoController {
    vram: [u8, 80 * 25];
    buf: [u8, 2000];
} 

trait Funcs {
    fn init_video(&mut self);
    fn disp(txt: &str);
    fn dispd(msg: &str);
}

impl Funcs for VideoController {
    fn init_cpu() {
        

    }
}