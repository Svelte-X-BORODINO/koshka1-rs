use crate::video::{disp, dispd};

#[derive(Debug)]
struct KoshkaCPU {
    k: [u8; 5],
    l_cache: [u8; 16384],
    kadv: str,
    kadv2: str,
}

impl KoshkaCPU {
    fn init_cpu(&mut self) {
        self.k = [0; 5];
        self.l_cache = [0; 16384];
        self.kadv = "".to_string();
        self.kadv2 = "".to_string();
    }

    fn panic_cpu(reason: impl std::fmt::Display) -> Result<()> {
        disp("panic cpu#0 res={}", reason);
        dispd("Panic recieved. CPU halted.");
        loop {}
        Ok(())
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
        if b == 0 {
            panic_cpu("div_by_zero")
        }
        dispd("kdiv({}, {}): {}", a, b, a / b);
    }

}
