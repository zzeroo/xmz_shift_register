extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

pub struct ShiftRegister {
    pub oe_pin: Pin,
    pub ds_pin: Pin,
    pub clock_pin: Pin,
    pub latch_pin: Pin,
    pub data: u64,
}

impl ShiftRegister {
    /// Creates a new instance
    ///
    /// All member variables can set to custom values
    pub fn new(oe_pin: u64, ds_pin: u64, clock_pin: u64, latch_pin: u64) -> ShiftRegister {
        ShiftRegister {
            oe_pin: Pin::new(oe_pin),
            ds_pin: Pin::new(ds_pin),
            clock_pin: Pin::new(clock_pin),
            latch_pin: Pin::new(latch_pin),
            data: 0
        }
    }

    /// Default constructor for leds
    pub fn new_led() -> ShiftRegister {
        ShiftRegister::new(276, 38, 44, 40)
    }

    /// Default constructor for relais
    pub fn new_relais() -> ShiftRegister {
        ShiftRegister::new(277, 45, 39, 37)
    }

    /// Export the needed pins, panic if this fails
    fn export_pins(&self) {
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
    }

    /// Sets the directions of the given pins.
    fn set_pin_direction(&self) {
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

    }

    /// Sets one given bit.
    ///
    /// # Arguments
    /// * `num`     - Bit number to set. **This number is one based!**
    ///
    /// The parameter `num` is one, not zero, based. This mean `set(1)` set the bit 0 in the shift register,
    /// `set(3)` set the 2nd bit and so forth.
    ///
    /// # Examples
    ///
    /// Set one given bit:
    ///
    /// ```
    /// extern crate xmz_shift_register;
    ///
    /// let mut led = xmz_shift_register::ShiftRegister::new_led();
    /// assert_eq!(led.data, 0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000);
    ///
    /// led.set(10);
    /// assert_eq!(led.data, 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000);
    /// ```
    ///
    /// Set two or more bit bit:
    ///
    /// If two or more bits are set the state of prevous seted bits is not change. If You eg. set bit
    /// 10 and after this you set bit 5 then bits 10 and bit 5 are set.
    ///
    /// ```
    /// extern crate xmz_shift_register;
    ///
    /// let mut led = xmz_shift_register::ShiftRegister::new_led();
    ///
    /// led.set(10);
    /// assert_eq!(led.data, 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00000000);
    /// led.set(5);
    /// assert_eq!(led.data, 0b00000000_00000000_00000000_00000000_00000000_00000000_00000010_00010000);
    /// ```
    pub fn set(&mut self, num: u64) {
        self.data |= 1 << num - 1;
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

    /// Shift out the current data
    pub fn shift_out(&self) {
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
