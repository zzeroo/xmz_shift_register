extern crate xmz_shift_register;
use xmz_shift_register::ShiftRegister;

#[test]
fn test_set() {
    let mut register = ShiftRegister::new(0, 0, 0, 0);
    // after construction the data field must be 0'ed
    assert!(register.data == 0);
    // set the fifth bit in data field
    register.set(5);
    assert!(register.data == 0b0001_0000);
}

// Set should only set new bits, it should keep the data field of the ShiftRegister struct intakt.
#[test]
fn test_multiple_set() {
    let mut register = ShiftRegister::new(0, 0, 0, 0);
    register.set(1);
    assert!(register.data == 0b1);
    register.set(5);
    assert!(register.data == 0b0001_0001);
    register.set(10);
    assert!(register.data == 0b0010_0001_0001);
}

/// `shift_out()` should not change the data in self.data field.
#[test]
fn test_shift_out_should_not_change_data() {
    let mut register = ShiftRegister::new(0, 0, 0, 0);
    register.set(1);
    register.set(2);
    assert!(register.data == 0b11);
    register.shift_out();
    assert!(register.data == 0b11);
}
