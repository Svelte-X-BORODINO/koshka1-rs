
#[derive(Debug)]
struct KoshkaCPU {
    k: [u8; 5],
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
}

impl Funcs for KoshkaCPU {
    fn init_cpu(&mut self) {
        self.k = [0; 5];
        self.kadv = "".to_string();
        self.kadv2 = "".to_string();
        
    }
} 
