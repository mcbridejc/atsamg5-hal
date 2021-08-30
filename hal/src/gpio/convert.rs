use super::*;

/// Const assert hack
struct Assert<const L: u8, const R: u8>;

impl<const L: u8, const R: u8> Assert<L, R> {
    pub const LESS: u8 = R - L - 1;
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Alternate<A>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Analog, P, N>> for Pin<Alternate<A>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_alternate::<A>()
    }
}

// impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<AlternateOD<B>, P, N>>
//     for Pin<Alternate<A>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<B>, P, N>) -> Self {
//         f.into_alternate::<A>()
//     }
// }

// impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Input<MODE>, P, N>>
//     for Pin<AlternateOD<A>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Input<MODE>, P, N>) -> Self {
//         f.into_alternate_open_drain::<A>()
//     }
// }

// impl<MODE, const P: char, const N: u8, const A: u8> From<Pin<Output<MODE>, P, N>>
//     for Pin<AlternateOD<A>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Output<MODE>, P, N>) -> Self {
//         f.into_alternate_open_drain::<A>()
//     }
// }

// impl<const P: char, const N: u8, const A: u8> From<Pin<Analog, P, N>>
//     for Pin<AlternateOD<A>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Analog, P, N>) -> Self {
//         f.into_alternate_open_drain::<A>()
//     }
// }

// impl<const P: char, const N: u8, const A: u8, const B: u8> From<Pin<Alternate<B>, P, N>>
//     for Pin<AlternateOD<A>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Alternate<B>, P, N>) -> Self {
//         f.into_alternate_open_drain::<A>()
//     }
// }

impl<const P: char, const N: u8> From<Pin<Input<Floating>, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<Floating>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullUp>, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullUp>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<PullDown>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<PullDown>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_pull_down_input()
    }
}

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Input<PullDown>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_pull_down_input()
//     }
// }

impl<const P: char, const N: u8> From<Pin<Input<Floating>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<Floating>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullDown>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullDown>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<PullUp>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<PullUp>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_pull_up_input()
    }
}

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Input<PullUp>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_pull_up_input()
//     }
// }

impl<const P: char, const N: u8> From<Pin<Input<PullDown>, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullDown>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Input<PullUp>, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Input<PullUp>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Output<MODE>, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Input<Floating>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_floating_input()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Input<Floating>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_floating_input()
    }
}

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Input<Floating>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_floating_input()
//     }
// }

// impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>>
//     for Pin<Output<OpenDrain>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Input<MODE>, P, N>) -> Self {
//         f.into_open_drain_output()
//     }
// }

// impl<const P: char, const N: u8> From<Pin<Output<PushPull>, P, N>>
//     for Pin<Output<OpenDrain>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Output<PushPull>, P, N>) -> Self {
//         f.into_open_drain_output()
//     }
// }

// impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Output<OpenDrain>, P, N> {
//     #[inline(always)]
//     fn from(f: Pin<Analog, P, N>) -> Self {
//         f.into_open_drain_output()
//     }
// }

// impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
//     for Pin<Output<OpenDrain>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Alternate<A>, P, N>) -> Self {
//         f.into_open_drain_output()
//     }
// }

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Output<OpenDrain>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_open_drain_output()
//     }
// }

impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Input<MODE>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

// impl<const P: char, const N: u8> From<Pin<Output<OpenDrain>, P, N>>
//     for Pin<Output<PushPull>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<Output<OpenDrain>, P, N>) -> Self {
//         f.into_push_pull_output()
//     }
// }

impl<const P: char, const N: u8> From<Pin<Analog, P, N>> for Pin<Output<PushPull>, P, N> {
    #[inline(always)]
    fn from(f: Pin<Analog, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>>
    for Pin<Output<PushPull>, P, N>
{
    #[inline(always)]
    fn from(f: Pin<Alternate<A>, P, N>) -> Self {
        f.into_push_pull_output()
    }
}

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Output<PushPull>, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_push_pull_output()
//     }
// }

// impl<MODE, const P: char, const N: u8> From<Pin<Input<MODE>, P, N>> for Pin<Analog, P, N> {
//     #[inline(always)]
//     fn from(f: Pin<Input<MODE>, P, N>) -> Self {
//         f.into_analog()
//     }
// }

// impl<MODE, const P: char, const N: u8> From<Pin<Output<MODE>, P, N>> for Pin<Analog, P, N> {
//     #[inline(always)]
//     fn from(f: Pin<Output<MODE>, P, N>) -> Self {
//         f.into_analog()
//     }
// }

// impl<const P: char, const N: u8, const A: u8> From<Pin<Alternate<A>, P, N>> for Pin<Analog, P, N> {
//     #[inline(always)]
//     fn from(f: Pin<Alternate<A>, P, N>) -> Self {
//         f.into_analog()
//     }
// }

// impl<const P: char, const N: u8, const A: u8> From<Pin<AlternateOD<A>, P, N>>
//     for Pin<Analog, P, N>
// {
//     #[inline(always)]
//     fn from(f: Pin<AlternateOD<A>, P, N>) -> Self {
//         f.into_analog()
//     }
// }

impl<MODE, const P: char, const N: u8> Pin<MODE, P, N> {
    fn set_alternate<const A: u8>(&mut self) {
        #[allow(path_statements, clippy::no_effect)]
        {
            Assert::<A, 4>::LESS;
        }

        unsafe {
            (*Pio::<P>::ptr()).pdr.write_with_zero(|w| w.bits(1<<N));
            match A {
                0 => {
                    (*Pio::<P>::ptr()).abcdsr[0].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                    (*Pio::<P>::ptr()).abcdsr[1].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                },
                1 => {
                    (*Pio::<P>::ptr()).abcdsr[0].modify(|r, w| w.bits(r.bits() | (1<<N)));
                    (*Pio::<P>::ptr()).abcdsr[1].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                },
                2 => {
                    (*Pio::<P>::ptr()).abcdsr[0].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                    (*Pio::<P>::ptr()).abcdsr[1].modify(|r, w| w.bits(r.bits() | (1<<N)));
                },
                3 => {
                    (*Pio::<P>::ptr()).abcdsr[0].modify(|r, w| w.bits(r.bits() | (1<<N)));
                    (*Pio::<P>::ptr()).abcdsr[1].modify(|r, w| w.bits(r.bits() | (1<<N)));
                },
                _ => {
                    (*Pio::<P>::ptr()).abcdsr[0].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                    (*Pio::<P>::ptr()).abcdsr[1].modify(|r, w| w.bits(r.bits() & !(1<<N)));
                },
            }
        }
    }
    /// Configures the pin to operate alternate mode
    pub fn into_alternate<const A: u8>(mut self) -> Pin<Alternate<A>, P, N> {
        self.set_alternate::<A>();
        Pin::new()
    }

    // /// Configures the pin to operate in alternate open drain mode
    // #[allow(path_statements)]
    // pub fn into_alternate_open_drain<const A: u8>(self) -> Pin<AlternateOD<A>, P, N> {
    //     self.into_alternate::<A>().set_open_drain()
    // }

    
    /// Configures the pin to operate in AF0 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_a(self) -> Pin<Alternate<0>, P, N> {
        self.into_alternate::<0>()
    }

    /// Configures the pin to operate in AF1 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_b(self) -> Pin<Alternate<1>, P, N> {
        self.into_alternate::<1>()
    }

    /// Configures the pin to operate in AF2 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_c(self) -> Pin<Alternate<2>, P, N> {
        self.into_alternate::<2>()
    }

    /// Configures the pin to operate in AF3 mode
    #[deprecated(since = "0.10.0")]
    pub fn into_alternate_d(self) -> Pin<Alternate<3>, P, N> {
        self.into_alternate::<3>()
    }

    // /// Configures the pin to operate in AF0 open drain mode
    // #[deprecated(since = "0.10.0")]
    // pub fn into_alternate_af0_open_drain(self) -> Pin<AlternateOD<0>, P, N> {
    //     self.into_alternate_open_drain::<0>()
    // }

    // /// Configures the pin to operate in AF1 open drain mode
    // #[deprecated(since = "0.10.0")]
    // pub fn into_alternate_af1_open_drain(self) -> Pin<AlternateOD<1>, P, N> {
    //     self.into_alternate_open_drain::<1>()
    // }

    // /// Configures the pin to operate in AF2 open drain mode
    // #[deprecated(since = "0.10.0")]
    // pub fn into_alternate_af2_open_drain(self) -> Pin<AlternateOD<2>, P, N> {
    //     self.into_alternate_open_drain::<2>()
    // }

    // /// Configures the pin to operate in AF3 open drain mode
    // #[deprecated(since = "0.10.0")]
    // pub fn into_alternate_af3_open_drain(self) -> Pin<AlternateOD<3>, P, N> {
    //     self.into_alternate_open_drain::<3>()
    // }

    /// Configures the pin to operate as a floating input pin
    pub fn into_floating_input(mut self) -> Pin<Input<Floating>, P, N> {
        self.float();
        //self.pull_mode::<PullMode::Floating>();
        self.disable_output();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled down input pin
    pub fn into_pull_down_input(mut self) -> Pin<Input<PullDown>, P, N> {
        //self.pull_mode::<PullMode::PullDown>();
        self.pull_down();
        self.disable_output();
        Pin::new()
    }

    /// Configures the pin to operate as a pulled up input pin
    pub fn into_pull_up_input(mut self) -> Pin<Input<PullUp>, P, N> {
        //self.pull_mode::<PullMode::PullUp>();
        self.pull_up();
        self.disable_output();
        Pin::new()
    }

    // /// Configures the pin to operate as an open drain output pin
    // /// Initial state will be low.
    // pub fn into_open_drain_output(mut self) -> Pin<Output<OpenDrain>, P, N> {
    //     self.mode::<Output<OpenDrain>>();
    //     Pin::new()
    // }

    // /// Configures the pin to operate as an open-drain output pin.
    // /// `initial_state` specifies whether the pin should be initially high or low.
    // pub fn into_open_drain_output_in_state(
    //     mut self,
    //     initial_state: PinState,
    // ) -> Pin<Output<OpenDrain>, P, N> {
    //     self._set_state(initial_state);
    //     self.mode::<Output<OpenDrain>>();
    //     Pin::new()
    // }

    /// Configures the pin to operate as an push pull output pin
    /// Initial state will be low.
    pub fn into_push_pull_output(mut self) -> Pin<Output<PushPull>, P, N> {
        self._set_low();
        self.enable_output();
        self.float();
        Pin::new()
    }

    /// Configures the pin to operate as an push-pull output pin.
    /// `initial_state` specifies whether the pin should be initially high or low.
    pub fn into_push_pull_output_in_state(
        mut self,
        initial_state: PinState,
    ) -> Pin<Output<PushPull>, P, N> {
        self._set_state(initial_state);
        self.enable_output();
        self.float();
        Pin::new()
    }

    // /// Configures the pin to operate as an analog input pin
    // pub fn into_analog(mut self) -> Pin<Analog, P, N> {
    //     self.mode::<Analog>();
    //     Pin::new()
    // }

    /// Sets pullup state
    #[inline(always)]
    fn float(&mut self) {
        unsafe {
            (*Pio::<P>::ptr()).ppddr.write_with_zero(|w| w.bits(1<<N));
            (*Pio::<P>::ptr()).pudr.write_with_zero(|w| w.bits(1<<N));
        }
    }

    #[inline(always)]
    fn pull_up(&mut self) {
        unsafe {
            (*Pio::<P>::ptr()).ppddr.write_with_zero(|w| w.bits(1<<N));
            (*Pio::<P>::ptr()).puer.write_with_zero(|w| w.bits(1<<N));
        }
    }

    #[inline(always)]
    fn pull_down(&mut self) {
        unsafe {
            (*Pio::<P>::ptr()).pudr.write_with_zero(|w| w.bits(1<<N));
            (*Pio::<P>::ptr()).ppder.write_with_zero(|w| w.bits(1<<N));
        }
    }

    /// Enables the output driver for `self`
    fn enable_output(&mut self) {
        unsafe {
            (*Pio::<P>::ptr()).oer.write_with_zero(|w| w.bits(1<<N));
        }
    }

    /// Disables the output driver for `self`
    fn disable_output(&mut self) {
        unsafe {
            (*Pio::<P>::ptr()).odr.write_with_zero(|w| w.bits(1<<N));
        }
    }
}

// impl<MODE, const P: char, const N: u8> Pin<MODE, P, N>
// where
//     MODE: PinMode,
// {
//     fn with_mode<M, F, R>(&mut self, f: F) -> R
//     where
//         M: PinMode,
//         F: FnOnce(&mut Pin<M, P, N>) -> R,
//     {
//         self.mode::<M>();

//         // This will reset the pin back to the original mode when dropped.
//         // (so either when `with_mode` returns or when `f` unwinds)
//         let _resetti = ResetMode { pin: self };

//         let mut witness = Pin::new();

//         f(&mut witness)
//     }

//     /// Temporarily configures this pin as a floating input.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     pub fn with_floating_input<R>(
//         &mut self,
//         f: impl FnOnce(&mut Pin<Input<Floating>, P, N>) -> R,
//     ) -> R {
//         self.with_mode(f)
//     }

//     /// Temporarily configures this pin as a pulled-down input.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     pub fn with_pull_down_input<R>(
//         &mut self,
//         f: impl FnOnce(&mut Pin<Input<PullDown>, P, N>) -> R,
//     ) -> R {
//         self.with_mode(f)
//     }

//     /// Temporarily configures this pin as a pulled-up input.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     pub fn with_pull_up_input<R>(
//         &mut self,
//         f: impl FnOnce(&mut Pin<Input<PullUp>, P, N>) -> R,
//     ) -> R {
//         self.with_mode(f)
//     }

//     /// Temporarily configures this pin as an analog pin.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     pub fn with_analog<R>(&mut self, f: impl FnOnce(&mut Pin<Analog, P, N>) -> R) -> R {
//         self.with_mode(f)
//     }

//     /// Temporarily configures this pin as an open drain output.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     /// The value of the pin after conversion is undefined. If you
//     /// want to control it, use `with_open_drain_output_in_state`
//     // pub fn with_open_drain_output<R>(
//     //     &mut self,
//     //     f: impl FnOnce(&mut Pin<Output<OpenDrain>, P, N>) -> R,
//     // ) -> R {
//     //     self.with_mode(f)
//     // }

//     /// Temporarily configures this pin as an open drain output .
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     /// Note that the new state is set slightly before conversion
//     /// happens. This can cause a short output glitch if switching
//     /// between output modes
//     // pub fn with_open_drain_output_in_state<R>(
//     //     &mut self,
//     //     state: PinState,
//     //     f: impl FnOnce(&mut Pin<Output<OpenDrain>, P, N>) -> R,
//     // ) -> R {
//     //     self._set_state(state);
//     //     self.with_mode(f)
//     // }

//     /// Temporarily configures this pin as a push-pull output.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     /// The value of the pin after conversion is undefined. If you
//     /// want to control it, use `with_push_pull_output_in_state`
//     pub fn with_push_pull_output<R>(
//         &mut self,
//         f: impl FnOnce(&mut Pin<Output<PushPull>, P, N>) -> R,
//     ) -> R {
//         self.with_mode(f)
//     }

//     /// Temporarily configures this pin as a push-pull output.
//     ///
//     /// The closure `f` is called with the reconfigured pin. After it returns,
//     /// the pin will be configured back.
//     /// Note that the new state is set slightly before conversion
//     /// happens. This can cause a short output glitch if switching
//     /// between output modes
//     pub fn with_push_pull_output_in_state<R>(
//         &mut self,
//         state: PinState,
//         f: impl FnOnce(&mut Pin<Output<PushPull>, P, N>) -> R,
//     ) -> R {
//         self._set_state(state);
//         self.with_mode(f)
//     }
// }

// struct ResetMode<'a, ORIG: PinMode, const P: char, const N: u8> {
//     pin: &'a mut Pin<ORIG, P, N>,
// }

// impl<'a, ORIG: PinMode, const P: char, const N: u8> Drop for ResetMode<'a, ORIG, P, N> {
//     fn drop(&mut self) {
//         self.pin.mode::<ORIG>();
//     }
// }

//pub struct Const<const A: u8>;

// pub trait SetAlternate<const A: u8> {
//     fn set_alt_mode(&mut self);
//     fn restore_mode(&mut self);
// }
// impl SetAlternate<0> for NoPin {
//     fn set_alt_mode(&mut self) {}
//     fn restore_mode(&mut self) {}
// }
// impl<MODE: PinMode, const P: char, const N: u8, const A: u8> SetAlternate<A> for Pin<MODE, P, N> {
//     fn set_alt_mode(&mut self) {
//         self.set_alternate::<A>();
//     }

//     fn restore_mode(&mut self) {
//         self.mode::<MODE>();
//     }
// }

// impl<const P: char, const N: u8, const A: u8> SetAlternate<A> for Pin<Alternate<A>, P, N> {
//     fn set_alt_mode(&mut self) {}
//     fn restore_mode(&mut self) {}
// }

// pub trait SetAlternateOD<const A: u8> {
//     fn set_alt_mode(&mut self);
//     fn restore_mode(&mut self);
// }
// impl SetAlternateOD<0> for NoPin {
//     fn set_alt_mode(&mut self) {}
//     fn restore_mode(&mut self) {}
// }
// impl<MODE: PinMode, const P: char, const N: u8, const A: u8> SetAlternateOD<A> for Pin<MODE, P, N> {
//     fn set_alt_mode(&mut self) {
//         self.set_alternate::<A>();
//         unsafe {
//             (*Gpio::<P>::ptr())
//                 .otyper
//                 .modify(|r, w| w.bits(r.bits() | (1 << N)))
//         };
//     }

//     fn restore_mode(&mut self) {
//         self.mode::<MODE>();
//     }
// }

// impl<const P: char, const N: u8, const A: u8> SetAlternateOD<A> for Pin<AlternateOD<A>, P, N> {
//     fn set_alt_mode(&mut self) {}
//     fn restore_mode(&mut self) {}
// }