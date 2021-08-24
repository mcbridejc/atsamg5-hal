#[doc = "Register `CSR0_ISOCHRONOUS` reader"]
pub struct R(crate::R<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR0_ISOCHRONOUS` writer"]
pub struct W(crate::W<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXCOMP` reader - Generates an IN Packet with Data Previously Written in the DPR"]
pub struct TXCOMP_R(crate::FieldReader<bool, bool>);
impl TXCOMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXCOMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXCOMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXCOMP` writer - Generates an IN Packet with Data Previously Written in the DPR"]
pub struct TXCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCOMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RX_DATA_BK0` reader - Receive Data Bank 0"]
pub struct RX_DATA_BK0_R(crate::FieldReader<bool, bool>);
impl RX_DATA_BK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_BK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_BK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_BK0` writer - Receive Data Bank 0"]
pub struct RX_DATA_BK0_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_BK0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXSETUP` reader - Received Setup"]
pub struct RXSETUP_R(crate::FieldReader<bool, bool>);
impl RXSETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSETUP` writer - Received Setup"]
pub struct RXSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSETUP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ISOERROR` reader - A CRC error has been detected in an isochronous transfer"]
pub struct ISOERROR_R(crate::FieldReader<bool, bool>);
impl ISOERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOERROR` writer - A CRC error has been detected in an isochronous transfer"]
pub struct ISOERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOERROR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TXPKTRDY` reader - Transmit Packet Ready"]
pub struct TXPKTRDY_R(crate::FieldReader<bool, bool>);
impl TXPKTRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXPKTRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXPKTRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXPKTRDY` writer - Transmit Packet Ready"]
pub struct TXPKTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPKTRDY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `FORCESTALL` reader - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub struct FORCESTALL_R(crate::FieldReader<bool, bool>);
impl FORCESTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCESTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCESTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCESTALL` writer - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub struct FORCESTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCESTALL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RX_DATA_BK1` reader - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub struct RX_DATA_BK1_R(crate::FieldReader<bool, bool>);
impl RX_DATA_BK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DATA_BK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DATA_BK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_DATA_BK1` writer - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub struct RX_DATA_BK1_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_BK1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DIR` reader - Transfer Direction (only available for control endpoints) (Read/Write)"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Transfer Direction (only available for control endpoints) (Read/Write)"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Endpoint Type (Read/Write)"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EPTYPE_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous OUT"]
    ISO_OUT = 1,
    #[doc = "5: Isochronous IN"]
    ISO_IN = 5,
    #[doc = "2: Bulk OUT"]
    BULK_OUT = 2,
    #[doc = "6: Bulk IN"]
    BULK_IN = 6,
    #[doc = "3: Interrupt OUT"]
    INT_OUT = 3,
    #[doc = "7: Interrupt IN"]
    INT_IN = 7,
}
impl From<EPTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: EPTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EPTYPE` reader - Endpoint Type (Read/Write)"]
pub struct EPTYPE_R(crate::FieldReader<u8, EPTYPE_A>);
impl EPTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        EPTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPTYPE_A> {
        match self.bits {
            0 => Some(EPTYPE_A::CTRL),
            1 => Some(EPTYPE_A::ISO_OUT),
            5 => Some(EPTYPE_A::ISO_IN),
            2 => Some(EPTYPE_A::BULK_OUT),
            6 => Some(EPTYPE_A::BULK_IN),
            3 => Some(EPTYPE_A::INT_OUT),
            7 => Some(EPTYPE_A::INT_IN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        **self == EPTYPE_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO_OUT`"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        **self == EPTYPE_A::ISO_OUT
    }
    #[doc = "Checks if the value of the field is `ISO_IN`"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        **self == EPTYPE_A::ISO_IN
    }
    #[doc = "Checks if the value of the field is `BULK_OUT`"]
    #[inline(always)]
    pub fn is_bulk_out(&self) -> bool {
        **self == EPTYPE_A::BULK_OUT
    }
    #[doc = "Checks if the value of the field is `BULK_IN`"]
    #[inline(always)]
    pub fn is_bulk_in(&self) -> bool {
        **self == EPTYPE_A::BULK_IN
    }
    #[doc = "Checks if the value of the field is `INT_OUT`"]
    #[inline(always)]
    pub fn is_int_out(&self) -> bool {
        **self == EPTYPE_A::INT_OUT
    }
    #[doc = "Checks if the value of the field is `INT_IN`"]
    #[inline(always)]
    pub fn is_int_in(&self) -> bool {
        **self == EPTYPE_A::INT_IN
    }
}
impl core::ops::Deref for EPTYPE_R {
    type Target = crate::FieldReader<u8, EPTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type (Read/Write)"]
pub struct EPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(EPTYPE_A::CTRL)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_OUT)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::ISO_IN)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulk_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_OUT)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulk_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::BULK_IN)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn int_out(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_OUT)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn int_in(self) -> &'a mut W {
        self.variant(EPTYPE_A::INT_IN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DTGLE` reader - Data Toggle (Read-only)"]
pub struct DTGLE_R(crate::FieldReader<bool, bool>);
impl DTGLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTGLE` writer - Data Toggle (Read-only)"]
pub struct DTGLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `EPEDS` reader - Endpoint Enable Disable"]
pub struct EPEDS_R(crate::FieldReader<bool, bool>);
impl EPEDS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEDS` writer - Endpoint Enable Disable"]
pub struct EPEDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEDS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RXBYTECNT` reader - Number of Bytes Available in the FIFO (Read-only)"]
pub struct RXBYTECNT_R(crate::FieldReader<u16, u16>);
impl RXBYTECNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXBYTECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBYTECNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBYTECNT` writer - Number of Bytes Available in the FIFO (Read-only)"]
pub struct RXBYTECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBYTECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&self) -> RX_DATA_BK0_R {
        RX_DATA_BK0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&self) -> RXSETUP_R {
        RXSETUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&self) -> ISOERROR_R {
        ISOERROR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TXPKTRDY_R {
        TXPKTRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&self) -> FORCESTALL_R {
        FORCESTALL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&self) -> RX_DATA_BK1_R {
        RX_DATA_BK1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints) (Read/Write)"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Endpoint Type (Read/Write)"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Data Toggle (Read-only)"]
    #[inline(always)]
    pub fn dtgle(&self) -> DTGLE_R {
        DTGLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&self) -> EPEDS_R {
        EPEDS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO (Read-only)"]
    #[inline(always)]
    pub fn rxbytecnt(&self) -> RXBYTECNT_R {
        RXBYTECNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TXCOMP_W {
        TXCOMP_W { w: self }
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&mut self) -> RX_DATA_BK0_W {
        RX_DATA_BK0_W { w: self }
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&mut self) -> RXSETUP_W {
        RXSETUP_W { w: self }
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&mut self) -> ISOERROR_W {
        ISOERROR_W { w: self }
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&mut self) -> TXPKTRDY_W {
        TXPKTRDY_W { w: self }
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&mut self) -> FORCESTALL_W {
        FORCESTALL_W { w: self }
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&mut self) -> RX_DATA_BK1_W {
        RX_DATA_BK1_W { w: self }
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints) (Read/Write)"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 8:10 - Endpoint Type (Read/Write)"]
    #[inline(always)]
    pub fn eptype(&mut self) -> EPTYPE_W {
        EPTYPE_W { w: self }
    }
    #[doc = "Bit 11 - Data Toggle (Read-only)"]
    #[inline(always)]
    pub fn dtgle(&mut self) -> DTGLE_W {
        DTGLE_W { w: self }
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&mut self) -> EPEDS_W {
        EPEDS_W { w: self }
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO (Read-only)"]
    #[inline(always)]
    pub fn rxbytecnt(&mut self) -> RXBYTECNT_W {
        RXBYTECNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isochronous_csr0_isochronous](index.html) module"]
pub struct ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC;
impl crate::RegisterSpec for ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isochronous_csr0_isochronous::R](R) reader structure"]
impl crate::Readable for ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isochronous_csr0_isochronous::W](W) writer structure"]
impl crate::Writable for ISOCHRONOUS_CSR0_ISOCHRONOUS_SPEC {
    type Writer = W;
}
