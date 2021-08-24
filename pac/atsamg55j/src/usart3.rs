#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_cr: [u8; 0x04],
    _reserved_1_mr: [u8; 0x04],
    _reserved_2_ier: [u8; 0x04],
    _reserved_3_idr: [u8; 0x04],
    _reserved_4_imr: [u8; 0x04],
    _reserved_5_csr: [u8; 0x04],
    #[doc = "0x18 - USART Receive Holding Register"]
    pub rhr: crate::Reg<rhr::RHR_SPEC>,
    #[doc = "0x1c - USART Transmit Holding Register"]
    pub thr: crate::Reg<thr::THR_SPEC>,
    #[doc = "0x20 - USART Baud Rate Generator Register"]
    pub brgr: crate::Reg<brgr::BRGR_SPEC>,
    #[doc = "0x24 - USART Receiver Time-out Register"]
    pub rtor: crate::Reg<rtor::RTOR_SPEC>,
    #[doc = "0x28 - USART Transmitter Timeguard Register"]
    pub ttgr: crate::Reg<ttgr::TTGR_SPEC>,
    _reserved11: [u8; 0x14],
    #[doc = "0x40 - USART FI DI Ratio Register"]
    pub fidi: crate::Reg<fidi::FIDI_SPEC>,
    #[doc = "0x44 - USART Number of Errors Register"]
    pub ner: crate::Reg<ner::NER_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x4c - USART IrDA Filter Register"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    _reserved14: [u8; 0x04],
    #[doc = "0x54 - USART LIN Mode Register"]
    pub linmr: crate::Reg<linmr::LINMR_SPEC>,
    #[doc = "0x58 - USART LIN Identifier Register"]
    pub linir: crate::Reg<linir::LINIR_SPEC>,
    #[doc = "0x5c - USART LIN Baud Rate Register"]
    pub linbrr: crate::Reg<linbrr::LINBRR_SPEC>,
    _reserved17: [u8; 0x30],
    #[doc = "0x90 - USART Comparison Register"]
    pub cmpr: crate::Reg<cmpr::CMPR_SPEC>,
    _reserved18: [u8; 0x50],
    #[doc = "0xe4 - USART Write Protection Mode Register"]
    pub wpmr: crate::Reg<wpmr::WPMR_SPEC>,
    #[doc = "0xe8 - USART Write Protection Status Register"]
    pub wpsr: crate::Reg<wpsr::WPSR_SPEC>,
    _reserved20: [u8; 0x14],
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
impl RegisterBlock {
    #[doc = "0x00 - USART Control Register"]
    #[inline(always)]
    pub fn spi_mode_cr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x00 - USART Control Register"]
    #[inline(always)]
    pub fn cr(&self) -> &crate::Reg<cr::CR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<cr::CR_SPEC>)
        }
    }
    #[doc = "0x04 - USART Mode Register"]
    #[inline(always)]
    pub fn spi_mode_mr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x04 - USART Mode Register"]
    #[inline(always)]
    pub fn mr(&self) -> &crate::Reg<mr::MR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<mr::MR_SPEC>)
        }
    }
    #[doc = "0x08 - USART Interrupt Enable Register"]
    #[inline(always)]
    pub fn lin_mode_ier_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x08 - USART Interrupt Enable Register"]
    #[inline(always)]
    pub fn spi_mode_ier_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x08 - USART Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x0c - USART Interrupt Disable Register"]
    #[inline(always)]
    pub fn lin_mode_idr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART Interrupt Disable Register"]
    #[inline(always)]
    pub fn spi_mode_idr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x0c - USART Interrupt Disable Register"]
    #[inline(always)]
    pub fn idr(&self) -> &crate::Reg<idr::IDR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(12usize)
                as *const crate::Reg<idr::IDR_SPEC>)
        }
    }
    #[doc = "0x10 - USART Interrupt Mask Register"]
    #[inline(always)]
    pub fn lin_mode_imr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x10 - USART Interrupt Mask Register"]
    #[inline(always)]
    pub fn spi_mode_imr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x10 - USART Interrupt Mask Register"]
    #[inline(always)]
    pub fn imr(&self) -> &crate::Reg<imr::IMR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(16usize)
                as *const crate::Reg<imr::IMR_SPEC>)
        }
    }
    #[doc = "0x14 - USART Channel Status Register"]
    #[inline(always)]
    pub fn lin_mode_csr_lin_mode(
        &self,
    ) -> &crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>)
        }
    }
    #[doc = "0x14 - USART Channel Status Register"]
    #[inline(always)]
    pub fn spi_mode_csr_spi_mode(
        &self,
    ) -> &crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>)
        }
    }
    #[doc = "0x14 - USART Channel Status Register"]
    #[inline(always)]
    pub fn csr(&self) -> &crate::Reg<csr::CSR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(20usize)
                as *const crate::Reg<csr::CSR_SPEC>)
        }
    }
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "USART Control Register"]
pub mod cr;
#[doc = "SPI_MODE_CR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_CR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CR_SPI_MODE = crate::Reg<spi_mode_cr_spi_mode::SPI_MODE_CR_SPI_MODE_SPEC>;
#[doc = "USART Control Register"]
pub mod spi_mode_cr_spi_mode;
#[doc = "MR register accessor: an alias for `Reg<MR_SPEC>`"]
pub type MR = crate::Reg<mr::MR_SPEC>;
#[doc = "USART Mode Register"]
pub mod mr;
#[doc = "SPI_MODE_MR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_MR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_MR_SPI_MODE = crate::Reg<spi_mode_mr_spi_mode::SPI_MODE_MR_SPI_MODE_SPEC>;
#[doc = "USART Mode Register"]
pub mod spi_mode_mr_spi_mode;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "USART Interrupt Enable Register"]
pub mod ier;
#[doc = "SPI_MODE_IER_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IER_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IER_SPI_MODE = crate::Reg<spi_mode_ier_spi_mode::SPI_MODE_IER_SPI_MODE_SPEC>;
#[doc = "USART Interrupt Enable Register"]
pub mod spi_mode_ier_spi_mode;
#[doc = "LIN_MODE_IER_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IER_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IER_LIN_MODE = crate::Reg<lin_mode_ier_lin_mode::LIN_MODE_IER_LIN_MODE_SPEC>;
#[doc = "USART Interrupt Enable Register"]
pub mod lin_mode_ier_lin_mode;
#[doc = "IDR register accessor: an alias for `Reg<IDR_SPEC>`"]
pub type IDR = crate::Reg<idr::IDR_SPEC>;
#[doc = "USART Interrupt Disable Register"]
pub mod idr;
#[doc = "SPI_MODE_IDR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IDR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IDR_SPI_MODE = crate::Reg<spi_mode_idr_spi_mode::SPI_MODE_IDR_SPI_MODE_SPEC>;
#[doc = "USART Interrupt Disable Register"]
pub mod spi_mode_idr_spi_mode;
#[doc = "LIN_MODE_IDR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IDR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IDR_LIN_MODE = crate::Reg<lin_mode_idr_lin_mode::LIN_MODE_IDR_LIN_MODE_SPEC>;
#[doc = "USART Interrupt Disable Register"]
pub mod lin_mode_idr_lin_mode;
#[doc = "IMR register accessor: an alias for `Reg<IMR_SPEC>`"]
pub type IMR = crate::Reg<imr::IMR_SPEC>;
#[doc = "USART Interrupt Mask Register"]
pub mod imr;
#[doc = "SPI_MODE_IMR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_IMR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_IMR_SPI_MODE = crate::Reg<spi_mode_imr_spi_mode::SPI_MODE_IMR_SPI_MODE_SPEC>;
#[doc = "USART Interrupt Mask Register"]
pub mod spi_mode_imr_spi_mode;
#[doc = "LIN_MODE_IMR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_IMR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_IMR_LIN_MODE = crate::Reg<lin_mode_imr_lin_mode::LIN_MODE_IMR_LIN_MODE_SPEC>;
#[doc = "USART Interrupt Mask Register"]
pub mod lin_mode_imr_lin_mode;
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "USART Channel Status Register"]
pub mod csr;
#[doc = "SPI_MODE_CSR_SPI_MODE register accessor: an alias for `Reg<SPI_MODE_CSR_SPI_MODE_SPEC>`"]
pub type SPI_MODE_CSR_SPI_MODE = crate::Reg<spi_mode_csr_spi_mode::SPI_MODE_CSR_SPI_MODE_SPEC>;
#[doc = "USART Channel Status Register"]
pub mod spi_mode_csr_spi_mode;
#[doc = "LIN_MODE_CSR_LIN_MODE register accessor: an alias for `Reg<LIN_MODE_CSR_LIN_MODE_SPEC>`"]
pub type LIN_MODE_CSR_LIN_MODE = crate::Reg<lin_mode_csr_lin_mode::LIN_MODE_CSR_LIN_MODE_SPEC>;
#[doc = "USART Channel Status Register"]
pub mod lin_mode_csr_lin_mode;
#[doc = "RHR register accessor: an alias for `Reg<RHR_SPEC>`"]
pub type RHR = crate::Reg<rhr::RHR_SPEC>;
#[doc = "USART Receive Holding Register"]
pub mod rhr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "USART Transmit Holding Register"]
pub mod thr;
#[doc = "BRGR register accessor: an alias for `Reg<BRGR_SPEC>`"]
pub type BRGR = crate::Reg<brgr::BRGR_SPEC>;
#[doc = "USART Baud Rate Generator Register"]
pub mod brgr;
#[doc = "RTOR register accessor: an alias for `Reg<RTOR_SPEC>`"]
pub type RTOR = crate::Reg<rtor::RTOR_SPEC>;
#[doc = "USART Receiver Time-out Register"]
pub mod rtor;
#[doc = "TTGR register accessor: an alias for `Reg<TTGR_SPEC>`"]
pub type TTGR = crate::Reg<ttgr::TTGR_SPEC>;
#[doc = "USART Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FIDI register accessor: an alias for `Reg<FIDI_SPEC>`"]
pub type FIDI = crate::Reg<fidi::FIDI_SPEC>;
#[doc = "USART FI DI Ratio Register"]
pub mod fidi;
#[doc = "NER register accessor: an alias for `Reg<NER_SPEC>`"]
pub type NER = crate::Reg<ner::NER_SPEC>;
#[doc = "USART Number of Errors Register"]
pub mod ner;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "USART IrDA Filter Register"]
pub mod if_;
#[doc = "LINMR register accessor: an alias for `Reg<LINMR_SPEC>`"]
pub type LINMR = crate::Reg<linmr::LINMR_SPEC>;
#[doc = "USART LIN Mode Register"]
pub mod linmr;
#[doc = "LINIR register accessor: an alias for `Reg<LINIR_SPEC>`"]
pub type LINIR = crate::Reg<linir::LINIR_SPEC>;
#[doc = "USART LIN Identifier Register"]
pub mod linir;
#[doc = "LINBRR register accessor: an alias for `Reg<LINBRR_SPEC>`"]
pub type LINBRR = crate::Reg<linbrr::LINBRR_SPEC>;
#[doc = "USART LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "CMPR register accessor: an alias for `Reg<CMPR_SPEC>`"]
pub type CMPR = crate::Reg<cmpr::CMPR_SPEC>;
#[doc = "USART Comparison Register"]
pub mod cmpr;
#[doc = "WPMR register accessor: an alias for `Reg<WPMR_SPEC>`"]
pub type WPMR = crate::Reg<wpmr::WPMR_SPEC>;
#[doc = "USART Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR register accessor: an alias for `Reg<WPSR_SPEC>`"]
pub type WPSR = crate::Reg<wpsr::WPSR_SPEC>;
#[doc = "USART Write Protection Status Register"]
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
