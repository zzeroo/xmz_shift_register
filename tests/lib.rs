extern crate xmz_shift_register;
use xmz_shift_register::{RegisterType, ShiftRegister};

#[test]
fn test_set() {
    let mut register = ShiftRegister::new(RegisterType::Mock);
    // after construction the data field must be 0'ed
    assert!(register.data == 0);
    // set the fifth bit in data field
    register.set(5);
    assert!(register.data == 0b0001_0000);
}

// Set should only set new bits, it should keep the data field of the ShiftRegister struct intakt.
#[test]
fn test_multiple_set() {
    let mut register = ShiftRegister::new(RegisterType::Mock);
    register.set(1);
    assert!(register.data == 0b1);
    register.set(5);
    assert!(register.data == 0b0001_0001);
    register.set(10);
    assert!(register.data == 0b0010_0001_0001);
}

#[test]
fn test_get_one_bit() {
    let mut register = ShiftRegister::new(RegisterType::Mock);
    // befor all value sould be null, tested already
    assert!(register.get(5) == false);
    register.set(5);
    assert!(register.get(5) == true);
}

#[test]
fn test_toggle_one_bit() {
    let mut register = ShiftRegister::new(RegisterType::Mock);
    assert!(register.get(5) == false);
    // Toggle on
    register.toggle(5);
    assert!(register.get(5) == true);
    // Toggle off
    register.toggle(5);
    assert!(register.get(5) == false);

    register.toggle(5);
    assert!(register.get(5) == true);
    register.toggle(5);
    assert!(register.get(5) == false);
}


/// `shift_out()` should not change the data in self.data field.
#[test]
fn test_shift_out_should_not_change_data() {
    let mut register = ShiftRegister::new(RegisterType::Mock);
    register.set(1);
    register.set(2);
    assert!(register.data == 0b11);
    register.shift_out();
    assert!(register.data == 0b11);
}

#[test]
#[should_panic]
#[ignore]
fn test_export_pins_on_led() {
    let led = ShiftRegister::new(RegisterType::LED);
    led.export_pins();
}

#[test]
#[should_panic]
#[ignore]
fn test_export_pins_on_relais() {
    let relais = ShiftRegister::new(RegisterType::Relais);
    relais.export_pins();
}

#[test]
fn test_export_pins_on_mock() {
    let mock = ShiftRegister::new(RegisterType::Mock);
    mock.export_pins();
}
