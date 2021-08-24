use crate::target_device::{self, PMC};
pub struct GenericClockController {

}

impl GenericClockController {
  pub fn new (
    pmc: &mut PMC,
  ) -> Self {
    
    // Use PLL A for main 120MHz clock, and 
    // PLL B for USB 48MHz clock. 
    // TODO: Support selecting internal 32kHz or external crystal
    // For now, select slow clock input before calling this, and note that
    // you will need an external crystal for USB.
    enable_pll_a(3662, pmc);

    // Use PLLA as master clock
    pmc.pmc_mckr.modify(|_r, w| 
      w.pres().clk_1()
      .plladiv2().clear_bit()
    );
    while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}
    pmc.pmc_mckr.modify(|_r, w| 
      w.css().plla_clk()
    );
    while pmc.pmc_sr.read().mckrdy().bit_is_clear() {}

    // Enable USB clock using PLL B
    // TODO: Needs to be optional
    enable_pll_b(1465, pmc);
    unsafe {
      pmc.pmc_usb.write(|w|
        w.usbdiv().bits(0) // divide by 1
        .usbs().set_bit() // use PLLB as source
      );
    }
    Self {}
  }
}

fn enable_pll_a(mul: u16, pmc: &mut PMC) {
  // TODO: Can I statically check this? 
  if mul < 8 || mul > 7500 {
    panic!("Out of range multiplier for PLLA")
  }
  unsafe {
    pmc.ckgr_pllar.write(|w| 
      w.mula().bits(mul - 1)
      .pllaen().bits(1)
      .pllacount().bits(50) 
    );
    // Wait for PLL lock
    while pmc.pmc_sr.read().locka().bit_is_clear() {}
  }
}

fn enable_pll_b(mul: u16, pmc: &mut PMC) {
  // TODO: Can I statically check this? 
  if mul < 8 || mul > 7500 {
    panic!("Out of range multiplier for PLLB")
  }
  unsafe {
    pmc.ckgr_pllbr.write(|w| 
      w.mulb().bits(mul - 1)
      .pllben().bits(1)
      .pllbcount().bits(50) 
    );
    // Wait for PLL lock
    while pmc.pmc_sr.read().locka().bit_is_clear() {}
  }
}
