extern crate sysfs_gpio;
use sysfs_gpio::{Direction, Pin};

// https://www.reddit.com/r/rust/comments/2umcxv/wait_rust_doesnt_have_function_overloading/
/// Types of differnt hardware.
///
/// There are shift register for led, for the relais and a mock type for testing.
/// Mock shift register don't call direct hardware.
pub enum RegisterType {
    LED,
    RELAIS,
    MOCK,
}

/// Datastructure representing the shift registers.
pub struct ShiftRegister {
    pub oe_pin: Pin,
    pub ds_pin: Pin,
    pub clock_pin: Pin,
    pub latch_pin: Pin,
    pub data: u64,
    pub register_type: RegisterType,
}

impl ShiftRegister {
    /// Creates a new instance
    ///
    pub fn new(register_type: RegisterType) -> ShiftRegister {
        match register_type {
            RegisterType::LED => ShiftRegister {
                oe_pin: Pin::new(276),
                ds_pin: Pin::new(38),
                clock_pin: Pin::new(44),
                latch_pin: Pin::new(40),
                data: 0, register_type: RegisterType::LED},
            RegisterType::RELAIS => ShiftRegister {
                oe_pin: Pin::new(277),
                ds_pin: Pin::new(45),
                clock_pin: Pin::new(39),
                latch_pin: Pin::new(37),
                data: 0, register_type: RegisterType::RELAIS},
            RegisterType::MOCK => ShiftRegister { oe_pin: Pin::new(0), ds_pin: Pin::new(0),
                clock_pin: Pin::new(0), latch_pin: Pin::new(0),
                data: 0, register_type: RegisterType::MOCK},
        }
    }

    /// Export the needed pins, panic if this fails
    ///
    pub fn export_pins(&self) {
        match self.register_type {
            RegisterType::LED | RegisterType::RELAIS => {
                match self.oe_pin.export() {
                    Ok(()) => (),
                    Err(err) => println!("!OE (output enabled) pin could not be exported: {}", err),
                }
                match self.ds_pin.export() {
                    Ok(()) => (),
                    Err(err) => println!("DATA pin could not be exported: {}", err),
                }
                match self.clock_pin.export() {
                    Ok(()) => (),
                    Err(err) => println!("CLOCK pin could not be exported: {}", err),
                }
                match self.latch_pin.export() {
                    Ok(()) => (),
                    Err(err) => println!("LATCH pin could not be exported: {}", err),
                }
            },
            _ => {},
        }
    }

    /// Get a boolean value depending on the pin state
    ///
    /// # Arguments
    /// * `num`     - Bit number to get. **This number is one based!**
    ///
    /// The parameter `num` is one, not zero, based. This mean `get(1)` get the bit 0 in the shift register,
    /// `get(3)` get the 2nd bit and so forth.
    ///
    pub fn get(&self, num: u64) -> bool {
        let result = (self.data >> num - 1) & 1;
        match result {
            0 => false,
            _ => true,
        }
    }

    /// Sets one given bit in data buffer
    ///
    /// # Arguments
    /// * `num`     - Bit number to set. **This number is one based!**
    ///
    /// The parameter `num` is one, not zero, based. This mean `set(1)` set the bit 0 in the shift register,
    /// `set(3)` set the 2nd bit and so forth.
    ///
    pub fn set(&mut self, num: u64) {
        self.data |= 1 << num - 1;
    }

    /// Toggle the given bit number in data buffer
    ///
    /// # Arguments
    /// * `num`     - Bit number to toggle. **This number is one based!**
    ///
    /// The parameter `num` is one, not zero, based. This mean `toggle(1)` toggle the bit 0 in the shift register,
    /// `toggle(3)` toggle the 2nd bit and so forth.
    ///
    pub fn toggle(&mut self, num: u64) {
        self.data ^= 1 << num - 1;
    }


    /// Sets the directions of the given pins.
    fn set_pin_direction(&self) {
        match self.register_type {
            RegisterType::LED | RegisterType::RELAIS => {
                match self.oe_pin.set_direction(Direction::Out) {
                    Ok(()) => { let _ = self.oe_pin.set_value(0); }, // !OE pin low == Shift register enabled.
                    Err(err) => println!("Could not set direction of DATA pin: {}", err),
                }

                match self.ds_pin.set_direction(Direction::Out) {
                    Ok(()) => { let _ = self.ds_pin.set_value(0); },
                    Err(err) => println!("Could not set direction of DATA pin: {}", err),
                }

                match self.clock_pin.set_direction(Direction::Out) {
                    Ok(()) => { let _ = self.clock_pin.set_value(0); },
                    Err(err) => println!("Could not set direction of CLOCK pin: {}", err),
                }

                match self.latch_pin.set_direction(Direction::Out) {
                    Ok(()) => { let _ = self.latch_pin.set_value(0); },
                    Err(err) => println!("Could not set direction of LATCH pin: {}", err),
                }
            },
            _ => {},
        }
    }

    /// Shift out the current data
    pub fn shift_out(&self) {
        match self.register_type {
            RegisterType::LED | RegisterType::RELAIS => {
                self.export_pins();
                self.set_pin_direction();

                // Clock in data
                for i in (0..64).rev() {
                    match (self.data >> i) & 1 {
                        1 => { self.ds_pin.set_value(1).unwrap(); },
                        _ => { self.ds_pin.set_value(0).unwrap(); },
                    }
                    self.clock_in();
                }
                self.latch_out();
            },
            _ => {},
        }
    }

    /// Toogles clock pin high->low
    fn clock_in(&self) {
        &self.clock_pin.set_value(1).unwrap();
        &self.clock_pin.set_value(0).unwrap();
    }

    /// Latch's the data out, latch pin high->low
    fn latch_out(&self) {
        &self.latch_pin.set_value(1).unwrap();
        &self.latch_pin.set_value(0).unwrap();
    }

    /// Initalises all shift register with zero's and shift this output.
    ///
    /// On startup the shift registers are in a unknown state,
    /// after this function, they are known zero.
    pub fn init(&mut self) {
        self.data = 0;
        self.shift_out();
    }
}
