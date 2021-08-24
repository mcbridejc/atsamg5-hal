#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FLEXCOM Mode register"]
    pub mr: crate::Reg<mr::MR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - FLEXCOM Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - FLEXCOM Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
}
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "FLEXCOM Mode register"]
pub mod mr;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "FLEXCOM Receive Holding Register"]
pub mod rhr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "FLEXCOM Transmit Holding Register"]
pub mod thr;
