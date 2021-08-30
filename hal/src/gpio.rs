//! General Purpose Input / Output
//!
//! The GPIO pins are organised into groups of 16 pins which can be accessed through the
//! `gpioa`, `gpiob`... modules. To get access to the pins, you first need to convert them into a
//! HAL designed struct from the `pac` struct using the [split](trait.GpioExt.html#tymethod.split) function.
//! ```rust
//! // Acquire the GPIOC peripheral
//! // NOTE: `dp` is the device peripherals from the `PAC` crate
//! let mut gpioa = dp.GPIOA.split();
//! ```
//!
//! This gives you a struct containing all the pins `px0..px15`.
//! By default pins are in floating input mode. You can change their modes.
//! For example, to set `pa5` high, you would call
//!
//! ```rust
//! let output = gpioa.pa5.into_push_pull_output();
//! output.set_high();
//! ```
//!
//! ## Modes
//!
//! Each GPIO pin can be set to various modes:
//!
//! - **Alternate**: Pin mode required when the pin is driven by other peripherals
//! - **AlternateOD**: Pin mode required when the pin is driven by other peripherals and has open drain
//! - **Analog**: Analog input to be used with ADC.
//! - Input
//!     - **PullUp**: Input connected to high with a weak pull up resistor. Will be high when nothing
//!     is connected
//!     - **PullDown**: Input connected to high with a weak pull up resistor. Will be low when nothing
//!     is connected
//!     - **Floating**: Input not pulled to high or low. Will be undefined when nothing is connected
//! - Output
//!     - **PushPull**: Output which either drives the pin high or low
//!     - **OpenDrain**: Output which leaves the gate floating, or pulls it do ground in drain
//!     mode. Can be used as an input in the `open` configuration
//!
//! ## Changing modes
//! The simplest way to change the pin mode is to use the `into_<mode>` functions. These return a
//! new struct with the correct mode that you can use the input or output functions on.
//!
//! If you need a more temporary mode change, and can not use the `into_<mode>` functions for
//! ownership reasons, you can use the closure based `with_<mode>` functions to temporarily change the pin type, do
//! some output or input, and then have it change back once done.

use core::convert::Infallible;
use core::marker::PhantomData;

pub use embedded_hal::digital::v2::PinState;
use embedded_hal::digital::v2::{
    InputPin, IoPin, OutputPin, StatefulOutputPin, ToggleableOutputPin,
};

mod convert;
//pub(crate) use convert::Const;
//mod partially_erased;
//pub use partially_erased::{PEPin, PartiallyErasedPin};
//mod erased;
//pub use erased::{EPin, ErasedPin};

/// A filler pin type
pub struct NoPin;

/// Extension trait to split a GPIO peripheral in independent pins and registers
pub trait GpioExt {
    /// The parts to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self) -> Self::Parts;
}

pub trait PinExt {
    type Mode;
    /// Return pin number
    fn pin_id(&self) -> u8;
    /// Return port number
    fn port_id(&self) -> u8;
}

/// Some alternate mode (type state)
pub struct Alternate<const A: u8>;

// Compatibility constants
pub const AFA: u8 = 0;
pub const AFB: u8 = 1;
pub const AFC: u8 = 2;
pub const AFD: u8 = 3;

/// Input mode (type state)
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input (type state)
pub struct Floating;

/// Pulled down input (type state)
pub struct PullDown;

/// Pulled up input (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Multidrive (open drain) mode (type state)
//pub struct MultiDrive;

/// Push pull output (type state)
pub struct PushPull;

/// Analog mode (type state)
pub struct Analog;

/// GPIO Pin speed selection
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Speed {
    Low = 0,
    Medium = 1,
    High = 2,
    VeryHigh = 3,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Edge {
    Rising,
    Falling,
    RisingFalling,
}

// mod sealed {
//     /// Marker trait that show if `ExtiPin` can be implemented
//     pub trait Interruptable {}
// }

// use sealed::Interruptable;
// impl<MODE> Interruptable for Output<MODE> {}
// impl<MODE> Interruptable for Input<MODE> {}

/// External Interrupt Pin
// pub trait ExtiPin {
//     fn make_interrupt_source(&mut self, syscfg: &mut SysCfg);
//     fn trigger_on_edge(&mut self, exti: &mut EXTI, level: Edge);
//     fn enable_interrupt(&mut self, exti: &mut EXTI);
//     fn disable_interrupt(&mut self, exti: &mut EXTI);
//     fn clear_interrupt_pending_bit(&mut self);
//     fn check_interrupt(&self) -> bool;
// }

// impl<PIN> ExtiPin for PIN
// where
//     PIN: PinExt,
//     PIN::Mode: Interruptable,
// {
//     /// Make corresponding EXTI line sensitive to this pin
//     #[inline(always)]
//     fn make_interrupt_source(&mut self, syscfg: &mut SysCfg) {
//         let i = self.pin_id();
//         let port = self.port_id() as u32;
//         let offset = 4 * (i % 4);
//         match i {
//             0..=3 => {
//                 syscfg.exticr1.modify(|r, w| unsafe {
//                     w.bits((r.bits() & !(0xf << offset)) | (port << offset))
//                 });
//             }
//             4..=7 => {
//                 syscfg.exticr2.modify(|r, w| unsafe {
//                     w.bits((r.bits() & !(0xf << offset)) | (port << offset))
//                 });
//             }
//             8..=11 => {
//                 syscfg.exticr3.modify(|r, w| unsafe {
//                     w.bits((r.bits() & !(0xf << offset)) | (port << offset))
//                 });
//             }
//             12..=15 => {
//                 syscfg.exticr4.modify(|r, w| unsafe {
//                     w.bits((r.bits() & !(0xf << offset)) | (port << offset))
//                 });
//             }
//             _ => unreachable!(),
//         }
//     }

//     /// Generate interrupt on rising edge, falling edge or both
//     #[inline(always)]
//     fn trigger_on_edge(&mut self, exti: &mut EXTI, edge: Edge) {
//         let i = self.pin_id();
//         match edge {
//             Edge::Rising => {
//                 exti.rtsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
//                 exti.ftsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
//             }
//             Edge::Falling => {
//                 exti.ftsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
//                 exti.rtsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << i)) });
//             }
//             Edge::RisingFalling => {
//                 exti.rtsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
//                 exti.ftsr
//                     .modify(|r, w| unsafe { w.bits(r.bits() | (1 << i)) });
//             }
//         }
//     }

//     /// Enable external interrupts from this pin.
//     #[inline(always)]
//     fn enable_interrupt(&mut self, exti: &mut EXTI) {
//         exti.imr
//             .modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_id())) });
//     }

//     /// Disable external interrupts from this pin
//     #[inline(always)]
//     fn disable_interrupt(&mut self, exti: &mut EXTI) {
//         exti.imr
//             .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << self.pin_id())) });
//     }

//     /// Clear the interrupt pending bit for this pin
//     #[inline(always)]
//     fn clear_interrupt_pending_bit(&mut self) {
//         unsafe { (*EXTI::ptr()).pr.write(|w| w.bits(1 << self.pin_id())) };
//     }

//     /// Reads the interrupt pending bit for this pin
//     #[inline(always)]
//     fn check_interrupt(&self) -> bool {
//         unsafe { ((*EXTI::ptr()).pr.read().bits() & (1 << self.pin_id())) != 0 }
//     }
// }

/// Generic pin type
///
/// - `MODE` is one of the pin modes (see [Modes](crate::gpio#modes) section).
/// - `P` is port name: `A` for GPIOA, `B` for GPIOB, etc.
/// - `N` is pin number: from `0` to `15`.
pub struct Pin<MODE, const P: char, const N: u8> {
    _mode: PhantomData<MODE>,
}
impl<MODE, const P: char, const N: u8> Pin<MODE, P, N> {
    const fn new() -> Self {
        Self { _mode: PhantomData }
    }
}

impl<MODE, const P: char, const N: u8> PinExt for Pin<MODE, P, N> {
    type Mode = MODE;

    #[inline(always)]
    fn pin_id(&self) -> u8 {
        N
    }
    #[inline(always)]
    fn port_id(&self) -> u8 {
        P as u8 - b'A'
    }
}

// impl<const P: char, const N: u8> Pin<Output<OpenDrain>, P, N> {
//     /// Enables / disables the internal pull up
//     pub fn internal_pull_up(&mut self, on: bool) {
//         if on {
//             unsafe {
//                 (*Gpio::<P>::ptr())
//                     .puer
//                     .write(|w| w.bits(1<<N))
//             };
//         } else {
//             unsafe {
//                 (*Gpio::<P>::ptr())
//                     .pudr
//                     ..write(|w| w.bits(1<<N))
//             };
//         }
//     }
// }


impl<const P: char, const N: u8, const A: u8> Pin<Alternate<A>, P, N> {
    /// Enables / disables the internal pull up
    pub fn internal_pull_up(self, on: bool) -> Self {
        if on {
            unsafe {
                (*Pio::<P>::ptr())
                    .puer
                    .write_with_zero(|w| w.bits(1<<N))
            };
        } else {
            unsafe {
                (*Pio::<P>::ptr())
                    .pudr
                    .write_with_zero(|w| w.bits(1<<N))
            };
        }

        self
    }
}

// impl<const P: char, const N: u8, const A: u8> Pin<Alternate<A>, P, N> {
//     /// Turns pin alternate configuration pin into open drain
//     pub fn set_open_drain(self) -> Pin<AlternateOD<A>, P, N> {
//         let offset = { N };
//         unsafe {
//             (*Gpio::<P>::ptr())
//                 .otyper
//                 .modify(|r, w| w.bits(r.bits() | (1 << offset)))
//         };

//         Pin::new()
//     }
// }

// impl<MODE, const P: char, const N: u8> Pin<MODE, P, N> {
//     /// Erases the pin number from the type
//     ///
//     /// This is useful when you want to collect the pins into an array where you
//     /// need all the elements to have the same type
//     pub fn erase_number(self) -> PEPin<MODE, P> {
//         PEPin::new(N)
//     }
//     #[deprecated(since = "0.10.0", note = "Please use erase_number instead")]
//     pub fn downgrade(self) -> PEPin<MODE, P> {
//         self.erase_number()
//     }

//     /// Erases the pin number and the port from the type
//     ///
//     /// This is useful when you want to collect the pins into an array where you
//     /// need all the elements to have the same type
//     pub fn erase(self) -> EPin<MODE> {
//         EPin::new(P as u8 - b'A', N)
//     }
//     #[deprecated(since = "0.10.0", note = "Please use erase instead")]
//     pub fn downgrade2(self) -> EPin<MODE> {
//         self.erase()
//     }
// }

impl<MODE, const P: char, const N: u8> Pin<MODE, P, N> {
    /// Set the output of the pin regardless of its mode.
    /// Primarily used to set the output value of the pin
    /// before changing its mode to an output to avoid
    /// a short spike of an incorrect value
    #[inline(always)]
    fn _set_state(&mut self, state: PinState) {
        match state {
            PinState::High => self._set_high(),
            PinState::Low => self._set_low(),
        }
    }
    #[inline(always)]
    fn _set_high(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Pio::<P>::ptr()).sodr.write_with_zero(|w| w.bits(1 << N)) }
    }
    #[inline(always)]
    fn _set_low(&mut self) {
        // NOTE(unsafe) atomic write to a stateless register
        unsafe { (*Pio::<P>::ptr()).codr.write_with_zero(|w| w.bits(1 << (16 + N))) }
    }
    #[inline(always)]
    fn _is_set_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Pio::<P>::ptr()).odsr.read().bits() & (1 << N) == 0 }
    }
    #[inline(always)]
    fn _is_low(&self) -> bool {
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Pio::<P>::ptr()).pdsr.read().bits() & (1 << N) == 0 }
    }
}

impl<MODE, const P: char, const N: u8> Pin<Output<MODE>, P, N> {
    #[inline(always)]
    pub fn set_high(&mut self) {
        self._set_high()
    }

    #[inline(always)]
    pub fn set_low(&mut self) {
        self._set_low()
    }

    #[inline(always)]
    pub fn get_state(&self) -> PinState {
        if self.is_set_low() {
            PinState::Low
        } else {
            PinState::High
        }
    }

    #[inline(always)]
    pub fn set_state(&mut self, state: PinState) {
        match state {
            PinState::Low => self.set_low(),
            PinState::High => self.set_high(),
        }
    }

    #[inline(always)]
    pub fn is_set_high(&self) -> bool {
        !self.is_set_low()
    }

    #[inline(always)]
    pub fn is_set_low(&self) -> bool {
        self._is_set_low()
    }

    #[inline(always)]
    pub fn toggle(&mut self) {
        if self.is_set_low() {
            self.set_high()
        } else {
            self.set_low()
        }
    }
}

impl<MODE, const P: char, const N: u8> OutputPin for Pin<Output<MODE>, P, N> {
    type Error = Infallible;

    #[inline(always)]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_high();
        Ok(())
    }

    #[inline(always)]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_low();
        Ok(())
    }
}

impl<MODE, const P: char, const N: u8> StatefulOutputPin for Pin<Output<MODE>, P, N> {
    #[inline(always)]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_high())
    }

    #[inline(always)]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_set_low())
    }
}

impl<MODE, const P: char, const N: u8> ToggleableOutputPin for Pin<Output<MODE>, P, N> {
    type Error = Infallible;

    #[inline(always)]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.toggle();
        Ok(())
    }
}

// impl<const P: char, const N: u8> Pin<Output<OpenDrain>, P, N> {
//     #[inline(always)]
//     pub fn is_high(&self) -> bool {
//         !self.is_low()
//     }

//     #[inline(always)]
//     pub fn is_low(&self) -> bool {
//         self._is_low()
//     }
// }

// impl<const P: char, const N: u8> InputPin for Pin<Output<OpenDrain>, P, N> {
//     type Error = Infallible;

//     #[inline(always)]
//     fn is_high(&self) -> Result<bool, Self::Error> {
//         Ok(self.is_high())
//     }

//     #[inline(always)]
//     fn is_low(&self) -> Result<bool, Self::Error> {
//         Ok(self.is_low())
//     }
// }

impl<MODE, const P: char, const N: u8> Pin<Input<MODE>, P, N> {
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        !self.is_low()
    }

    #[inline(always)]
    pub fn is_low(&self) -> bool {
        self._is_low()
    }
}

impl<MODE, const P: char, const N: u8> InputPin for Pin<Input<MODE>, P, N> {
    type Error = Infallible;

    #[inline(always)]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self.is_high())
    }

    #[inline(always)]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self.is_low())
    }
}

// impl<const P: char, const N: u8> IoPin<Self, Self> for Pin<Output<OpenDrain>, P, N> {
//     type Error = Infallible;
//     fn into_input_pin(self) -> Result<Self, Self::Error> {
//         Ok(self)
//     }
//     fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
//         self.set_state(state);
//         Ok(self)
//     }
// }

impl<const P: char, const N: u8> IoPin<Pin<Input<Floating>, P, N>, Self>
    for Pin<Output<PushPull>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Pin<Input<Floating>, P, N>, Self::Error> {
        Ok(self.into_floating_input())
    }
    fn into_output_pin(mut self, state: PinState) -> Result<Self, Self::Error> {
        self.set_state(state);
        Ok(self)
    }
}

impl<const P: char, const N: u8> IoPin<Self, Pin<Output<PushPull>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    type Error = Infallible;
    fn into_input_pin(self) -> Result<Self, Self::Error> {
        Ok(self)
    }
    fn into_output_pin(self, state: PinState) -> Result<Pin<Output<PushPull>, P, N>, Self::Error> {
        Ok(self.into_push_pull_output_in_state(state))
    }
}

macro_rules! gpio {
    ($PIOX:ident, $gpiox:ident, $PEPin:ident, $port_id:expr, $PXn:ident, [
        $($PXi:ident: ($pxi:ident, $i:expr, $MODE:ty),)+
    ]) => {
        /// GPIO
        pub mod $gpiox {
            use crate::target_device::{$PIOX};
            //use crate::rcc::{Enable, Reset};
            use super::{
                Floating, Input,
            };

            /// GPIO parts
            pub struct Parts {
                $(
                    /// Pin
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl super::GpioExt for $PIOX {
                type Parts = Parts;

                fn split(self) -> Parts {
                    //unsafe {
                        // TODO: Need to implement peripheral clock control elsewhere for this to work

                        // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                        // let rcc = &(*RCC::ptr());

                        // // Enable clock.
                        // $GPIOX::enable(rcc);
                        // $GPIOX::reset(rcc);
                    //}
                    Parts {
                        $(
                            $pxi: $PXi::new(),
                        )+
                    }
                }
            }

            //pub type $PXn<MODE> = super::PEPin<MODE, $port_id>;

            $(
                pub type $PXi<MODE> = super::Pin<MODE, $port_id, $i>;
            )+

        }
    }
}

gpio!(PIOA, gpioa, PA, 'A', PAn, [
    PA0: (pa0, 0, Input<Floating>),
    PA1: (pa1, 1, Input<Floating>),
    PA2: (pa2, 2, Input<Floating>),
    PA3: (pa3, 3, Input<Floating>),
    PA4: (pa4, 4, Input<Floating>),
    PA5: (pa5, 5, Input<Floating>),
    PA6: (pa6, 6, Input<Floating>),
    PA7: (pa7, 7, Input<Floating>),
    PA8: (pa8, 8, Input<Floating>),
    PA9: (pa9, 9, Input<Floating>),
    PA10: (pa10, 10, Input<Floating>),
    PA11: (pa11, 11, Input<Floating>),
    PA12: (pa12, 12, Input<Floating>),
    PA13: (pa13, 13, Input<Floating>),
    PA14: (pa14, 14, Input<Floating>),
    PA15: (pa15, 15, Input<Floating>),
    PA16: (pa16, 16, Input<Floating>),
    PA17: (pa17, 17, Input<Floating>),
    PA18: (pa18, 18, Input<Floating>),
    PA19: (pa19, 19, Input<Floating>),
    PA20: (pa20, 20, Input<Floating>),
    PA21: (pa21, 21, Input<Floating>),
    PA22: (pa22, 22, Input<Floating>),
    PA23: (pa23, 23, Input<Floating>),
    PA24: (pa24, 24, Input<Floating>),
    PA25: (pa25, 25, Input<Floating>),
    PA26: (pa26, 26, Input<Floating>),
    PA27: (pa27, 27, Input<Floating>),
    PA28: (pa28, 28, Input<Floating>),
    PA29: (pa29, 29, Input<Floating>),
    PA30: (pa30, 30, Input<Floating>),
    PA31: (pa31, 31, Input<Floating>),
]);

gpio!(PIOB, gpiob, PB, 'B', PBn, [
    PB0: (pb0, 0, Input<Floating>),
    PB1: (pb1, 1, Input<Floating>),
    PB2: (pb2, 2, Input<Floating>),
    PB3: (pb3, 3, Input<Floating>),
    PB4: (pb4, 4, Input<Floating>),
    PB5: (pb5, 5, Input<Floating>),
    PB6: (pb6, 6, Input<Floating>),
    PB7: (pb7, 7, Input<Floating>),
    PB8: (pb8, 8, Input<Floating>),
    PB9: (pb9, 9, Input<Floating>),
    PB10: (pb10, 10, Input<Floating>),
    PB11: (pb11, 11, Input<Floating>),
    PB12: (pb12, 12, Input<Floating>),
    PB13: (pb13, 13, Input<Floating>),
    PB14: (pb14, 14, Input<Floating>),
    PB15: (pb15, 15, Input<Floating>),
]);

struct Pio<const P: char>;
impl<const P: char> Pio<P> {
    const fn ptr() -> *const crate::target_device::pioa::RegisterBlock {
        match P {
            'A' => crate::target_device::PIOA::ptr(),
            'B' => crate::target_device::PIOB::ptr() as _,
            _ => crate::target_device::PIOA::ptr(),
        }
    }
}