#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x0c - Master Configuration Register"]
    pub matrix_mcfg: [crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>; 3],
    _reserved1: [u8; 0x34],
    #[doc = "0x40..0x50 - Slave Configuration Register"]
    pub matrix_scfg: [crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>; 4],
    _reserved2: [u8; 0x30],
    #[doc = "0x80 - Priority Register A for Slave 0"]
    pub matrix_pras0: crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x88 - Priority Register A for Slave 1"]
    pub matrix_pras1: crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x90 - Priority Register A for Slave 2"]
    pub matrix_pras2: crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>,
    _reserved5: [u8; 0x04],
    #[doc = "0x98 - Priority Register A for Slave 3"]
    pub matrix_pras3: crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>,
    _reserved6: [u8; 0x78],
    #[doc = "0x114 - System I/O Configuration Register"]
    pub ccfg_sysio: crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>,
    #[doc = "0x118 - Dynamic Clock Gating Register"]
    pub ccfg_dynckg: crate::Reg<ccfg_dynckg::CCFG_DYNCKG_SPEC>,
    #[doc = "0x11c - I2S Clock Source Selection Register"]
    pub ccfg_i2sclksel: crate::Reg<ccfg_i2sclksel::CCFG_I2SCLKSEL_SPEC>,
    #[doc = "0x120 - USB Management Register"]
    pub ccfg_usbmr: crate::Reg<ccfg_usbmr::CCFG_USBMR_SPEC>,
    _reserved10: [u8; 0xc0],
    #[doc = "0x1e4 - Write Protection Mode Register"]
    pub matrix_wpmr: crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>,
    #[doc = "0x1e8 - Write Protection Status Register"]
    pub matrix_wpsr: crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>,
}
#[doc = "MATRIX_MCFG register accessor: an alias for `Reg<MATRIX_MCFG_SPEC>`"]
pub type MATRIX_MCFG = crate::Reg<matrix_mcfg::MATRIX_MCFG_SPEC>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG register accessor: an alias for `Reg<MATRIX_SCFG_SPEC>`"]
pub type MATRIX_SCFG = crate::Reg<matrix_scfg::MATRIX_SCFG_SPEC>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 register accessor: an alias for `Reg<MATRIX_PRAS0_SPEC>`"]
pub type MATRIX_PRAS0 = crate::Reg<matrix_pras0::MATRIX_PRAS0_SPEC>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 register accessor: an alias for `Reg<MATRIX_PRAS1_SPEC>`"]
pub type MATRIX_PRAS1 = crate::Reg<matrix_pras1::MATRIX_PRAS1_SPEC>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 register accessor: an alias for `Reg<MATRIX_PRAS2_SPEC>`"]
pub type MATRIX_PRAS2 = crate::Reg<matrix_pras2::MATRIX_PRAS2_SPEC>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 register accessor: an alias for `Reg<MATRIX_PRAS3_SPEC>`"]
pub type MATRIX_PRAS3 = crate::Reg<matrix_pras3::MATRIX_PRAS3_SPEC>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "CCFG_SYSIO register accessor: an alias for `Reg<CCFG_SYSIO_SPEC>`"]
pub type CCFG_SYSIO = crate::Reg<ccfg_sysio::CCFG_SYSIO_SPEC>;
#[doc = "System I/O Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_DYNCKG register accessor: an alias for `Reg<CCFG_DYNCKG_SPEC>`"]
pub type CCFG_DYNCKG = crate::Reg<ccfg_dynckg::CCFG_DYNCKG_SPEC>;
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "CCFG_I2SCLKSEL register accessor: an alias for `Reg<CCFG_I2SCLKSEL_SPEC>`"]
pub type CCFG_I2SCLKSEL = crate::Reg<ccfg_i2sclksel::CCFG_I2SCLKSEL_SPEC>;
#[doc = "I2S Clock Source Selection Register"]
pub mod ccfg_i2sclksel;
#[doc = "CCFG_USBMR register accessor: an alias for `Reg<CCFG_USBMR_SPEC>`"]
pub type CCFG_USBMR = crate::Reg<ccfg_usbmr::CCFG_USBMR_SPEC>;
#[doc = "USB Management Register"]
pub mod ccfg_usbmr;
#[doc = "MATRIX_WPMR register accessor: an alias for `Reg<MATRIX_WPMR_SPEC>`"]
pub type MATRIX_WPMR = crate::Reg<matrix_wpmr::MATRIX_WPMR_SPEC>;
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR register accessor: an alias for `Reg<MATRIX_WPSR_SPEC>`"]
pub type MATRIX_WPSR = crate::Reg<matrix_wpsr::MATRIX_WPSR_SPEC>;
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
