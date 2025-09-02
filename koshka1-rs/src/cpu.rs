#[path = "./video.rs"]
mod video;
use video::VideoController;

#[derive(Debug)] 
pub struct KoshkaCPU {
    k: [u8; 5],
    l_cache: [u8; 16384],
    kadv: String,    
    kadv2: String,   
}

impl KoshkaCPU {
    // Метод new должен быть статическим и возвращать экземпляр
    pub fn new() -> Self {
        Self {
            k: [0; 5],
            l_cache: [0; 16384],
            kadv: String::new(),  
            kadv2: String::new(), 
        }
    }

    fn panic_cpu(reason: impl std::fmt::Display) {
        VideoController::disp(&format!("panic cpu#0 res={}", reason));
        VideoController::dispd("Panic recieved. CPU halted.");
        loop {}
    }

    pub fn print_status(&self) {
        println!("{:#?}", self);
    }

    pub fn kadd(&mut self, a: u8, b: u8) {
        VideoController::dispd(&format!("kadd({}, {}): {}", a, b, a + b));
    }

    pub fn ksub(&mut self, a: u8, b: u8) {
        VideoController::dispd(&format!("ksub({}, {}): {}", a, b, a - b));
    }

    pub fn kmul(&mut self, a: u8, b: u8) {
        VideoController::dispd(&format!("kmul({}, {}): {}", a, b, a * b));
    }

    pub fn kdiv(&mut self, a: u8, b: u8) {
        if b == 0 {
            Self::panic_cpu("div_by_zero") // Вызов статического метода
        }
        VideoController::dispd(&format!("kdiv({}, {}): {}", a, b, a / b));
    }
}