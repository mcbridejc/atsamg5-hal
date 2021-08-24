#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TWI Control Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - TWI Master Mode Register"]
    pub mmr: crate::Reg<mmr::MMR_SPEC>,
    #[doc = "0x08 - TWI Slave Mode Register"]
    pub smr: crate::Reg<smr::SMR_SPEC>,
    #[doc = "0x0c - TWI Internal Address Register"]
    pub iadr: crate::Reg<iadr::IADR_SPEC>,
    #[doc = "0x10 - TWI Clock Waveform Generator Register"]
    pub cwgr: crate::Reg<cwgr::CWGR_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - TWI Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x24 - TWI Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x28 - TWI Interrupt Disable Register"]
    pub idr: crate::Reg<idr::IDR_SPEC>,
    #[doc = "0x2c - TWI Interrupt Mask Register"]
    pub imr: crate::Reg<imr::IMR_SPEC>,
    #[doc = "0x30 - TWI Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x34 - TWI Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x38 - TWI SMBus Timing Register"]
    pub smbtr: crate::Reg<smbtr::SMBTR_SPEC>,
    _reserved12: [u8; 0x04],
    #[doc = "0x40 - TWI Alternative Command Register"]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    #[doc = "0x44 - TWI Filter Register"]
    pub filtr: crate::Reg<filtr::FILTR_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x4c - TWI SleepWalking Matching Register"]
    pub swmr: crate::Reg<swmr::SWMR_SPEC>,
    _reserved15: [u8; 0x94],
    #[doc = "0xe4 - TWI Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - TWI Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved17: [u8; 0x14],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: crate::Reg<rpr::RPR_SPEC>,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: crate::Reg<rcr::RCR_SPEC>,
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: crate::Reg<tpr::TPR_SPEC>,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: crate::Reg<tcr::TCR_SPEC>,
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: crate::Reg<rnpr::RNPR_SPEC>,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: crate::Reg<rncr::RNCR_SPEC>,
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: crate::Reg<tnpr::TNPR_SPEC>,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: crate::Reg<tncr::TNCR_SPEC>,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: crate::Reg<ptcr::PTCR_SPEC>,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: crate::Reg<ptsr::PTSR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "TWI Control Register"]
pub mod cr;
#[doc = "MMR register accessor: an alias for `Reg<MMR_SPEC>`"]
pub type MMR = crate::Reg<mmr::MMR_SPEC>;
#[doc = "TWI Master Mode Register"]
pub mod mmr;
#[doc = "SMR register accessor: an alias for `Reg<SMR_SPEC>`"]
pub type SMR = crate::Reg<smr::SMR_SPEC>;
#[doc = "TWI Slave Mode Register"]
pub mod smr;
#[doc = "IADR register accessor: an alias for `Reg<IADR_SPEC>`"]
pub type IADR = crate::Reg<iadr::IADR_SPEC>;
#[doc = "TWI Internal Address Register"]
pub mod iadr;
#[doc = "CWGR register accessor: an alias for `Reg<CWGR_SPEC>`"]
pub type CWGR = crate::Reg<cwgr::CWGR_SPEC>;
#[doc = "TWI Clock Waveform Generator Register"]
pub mod cwgr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "TWI Status Register"]
pub mod sr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "TWI Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "TWI Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "TWI Interrupt Mask Register"]
pub mod imr;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "TWI Receive Holding Register"]
pub mod rhr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "TWI Transmit Holding Register"]
pub mod thr;
#[doc = "SMBTR register accessor: an alias for `Reg<SMBTR_SPEC>`"]
pub type SMBTR = crate::Reg<smbtr::SMBTR_SPEC>;
#[doc = "TWI SMBus Timing Register"]
pub mod smbtr;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "TWI Alternative Command Register"]
pub mod acr;
#[doc = "FILTR register accessor: an alias for `Reg<FILTR_SPEC>`"]
pub type FILTR = crate::Reg<filtr::FILTR_SPEC>;
#[doc = "TWI Filter Register"]
pub mod filtr;
#[doc = "SWMR register accessor: an alias for `Reg<SWMR_SPEC>`"]
pub type SWMR = crate::Reg<swmr::SWMR_SPEC>;
#[doc = "TWI SleepWalking Matching Register"]
pub mod swmr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "TWI Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "TWI Write Protection Status Register"]
pub mod wpsr;
#[doc = "RPR register accessor: an alias for `Reg<RPR_SPEC>`"]
pub type RPR = crate::Reg<rpr::RPR_SPEC>;
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "RCR register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "TPR register accessor: an alias for `Reg<TPR_SPEC>`"]
pub type TPR = crate::Reg<tpr::TPR_SPEC>;
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "TCR register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::Reg<tcr::TCR_SPEC>;
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "RNPR register accessor: an alias for `Reg<RNPR_SPEC>`"]
pub type RNPR = crate::Reg<rnpr::RNPR_SPEC>;
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "RNCR register accessor: an alias for `Reg<RNCR_SPEC>`"]
pub type RNCR = crate::Reg<rncr::RNCR_SPEC>;
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "TNPR register accessor: an alias for `Reg<TNPR_SPEC>`"]
pub type TNPR = crate::Reg<tnpr::TNPR_SPEC>;
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "TNCR register accessor: an alias for `Reg<TNCR_SPEC>`"]
pub type TNCR = crate::Reg<tncr::TNCR_SPEC>;
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "PTCR register accessor: an alias for `Reg<PTCR_SPEC>`"]
pub type PTCR = crate::Reg<ptcr::PTCR_SPEC>;
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "PTSR register accessor: an alias for `Reg<PTSR_SPEC>`"]
pub type PTSR = crate::Reg<ptsr::PTSR_SPEC>;
#[doc = "Transfer Status Register"]
pub mod ptsr;
