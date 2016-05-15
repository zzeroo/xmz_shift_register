extern crate xmz_shift_register;
use xmz_shift_register::{ShiftRegister, RegisterType};
use std::thread::sleep;
use std::time::Duration;

fn testlauf(mut register: ShiftRegister) {
    register.set(1);
    register.set(3);
    register.shift_out();
    sleep(Duration::from_millis(300));
    register.toggle(1);
    register.toggle(3);
    register.shift_out();
    sleep(Duration::from_millis(300));
    for i in 1..10 {
        register.set(i);
        register.shift_out();
        sleep(Duration::from_millis(300));
    }
}


fn main() {
    let register_led = ShiftRegister::new(RegisterType::RELAIS);
    testlauf(register_led);
}
