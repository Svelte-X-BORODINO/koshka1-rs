pub struct VideoController {
        vram: [u8; 80 * 25],
        buf: [u8; 2000],
    }


impl VideoController {
   
    
        fn init_video(&mut self) {
            self.vram = [u8; 0];
            self.buf = [u8; 0];
        }
    
        fn disp(txt: impl std::fmt::Display) {
            println!("{}", txt);
        }
    
        fn dispd(msg: impl std::fmt::Display) {
            println!("[*] {}", msg);
        }
    
}
