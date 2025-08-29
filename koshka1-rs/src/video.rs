pub struct VideoController {
        vram: [u8; 80 * 25 * 2],
        buf: [u8; 2000],
    }


impl VideoController {
        pub fn init_video(&mut self) -> Self { 
            Self { 
                vram: [0; 80 * 25 * 2], buf: [0; 2000]  
            }
        }
    
        pub fn disp(txt: impl std::fmt::Display) {
            println!("{}", txt);
        }
    
        pub fn dispd(msg: impl std::fmt::Display) {
            println!("[*] {}", msg);
        }
}

