use crate::video;

#[derive(Debug)]
struct KoshkaCPU {
    k: [u8; 5],
    l_cache: [u8; 16384],
    kadv: &str,
    kadv2: &str,
}
trait Funcs {
    fn init_cpu(&mut self);
    fn print_status(&self);
    fn kadd(&mut self, a: u8, b: u8);
    fn ksub(&mut self, a: u8, b: u8);
    fn kmul(&mut self, a: u8, b: u8);
    fn kdiv(&mut self, a: u8, b: u8);
    fn panic_cpu(reason: &str) -> Result<bool>;
}

impl Funcs for KoshkaCPU {
    fn init_cpu(&mut self) {
        self.k = [0; 5];
        self.l_cache = [0; 16384];
        self.kadv = "".to_string();
        self.kadv2 = "".to_string();
    }
    fn print_status(&self) {
        println!("{:#?}", self);
    }
    
    fn kadd(&mut self, a: u8, b: u8) {
        dispd("kadd({}, {}): {}", a, b, a + b);
    }
    
    fn ksub(&mut self, a: u8, b: u8) {
        dispd("ksub({}, {}): {}", a, b, a - b);
    }
    
    fn kmul(&mut self, a: u8, b: u8) {
        dispd("kmul({}, {}): {}", a, b, a * b);
    }
    
    fn kdiv(&mut self, a: u8, b: u8) {
        dispd("kdiv({}, {}): {}", a, b, a / b);
    }
    
    fn panic_cpu(reason: &str) -> Result<bool, &'static str> {
        disp("panic cpu#0");
        dispd("Panic recieved. CPU halted.");
        Ok(false);
    }
}
