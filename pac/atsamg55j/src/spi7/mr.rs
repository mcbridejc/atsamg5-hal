#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub struct MSTR_R(crate::FieldReader<bool, bool>);
impl MSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
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
#[doc = "Field `PS` reader - Peripheral Select"]
pub struct PS_R(crate::FieldReader<bool, bool>);
impl PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Peripheral Select"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
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
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub struct PCSDEC_R(crate::FieldReader<bool, bool>);
impl PCSDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSDEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCSDEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub struct PCSDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSDEC_W<'a> {
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
#[doc = "Bit Rate Source Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRSRCCLK_A {
    #[doc = "0: The peripheral clock is the source clock for the bit rate generation."]
    PERIPH_CLK = 0,
    #[doc = "1: PMC PCKx is the source clock for the bit rate generation, thus the bit rate can be independent of the core/peripheral clock."]
    PMC_PCK = 1,
}
impl From<BRSRCCLK_A> for bool {
    #[inline(always)]
    fn from(variant: BRSRCCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSRCCLK` reader - Bit Rate Source Clock"]
pub struct BRSRCCLK_R(crate::FieldReader<bool, BRSRCCLK_A>);
impl BRSRCCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRSRCCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRSRCCLK_A {
        match self.bits {
            false => BRSRCCLK_A::PERIPH_CLK,
            true => BRSRCCLK_A::PMC_PCK,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK`"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        **self == BRSRCCLK_A::PERIPH_CLK
    }
    #[doc = "Checks if the value of the field is `PMC_PCK`"]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        **self == BRSRCCLK_A::PMC_PCK
    }
}
impl core::ops::Deref for BRSRCCLK_R {
    type Target = crate::FieldReader<bool, BRSRCCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRSRCCLK` writer - Bit Rate Source Clock"]
pub struct BRSRCCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRSRCCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BRSRCCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The peripheral clock is the source clock for the bit rate generation."]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut W {
        self.variant(BRSRCCLK_A::PERIPH_CLK)
    }
    #[doc = "PMC PCKx is the source clock for the bit rate generation, thus the bit rate can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut W {
        self.variant(BRSRCCLK_A::PMC_PCK)
    }
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
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub struct MODFDIS_R(crate::FieldReader<bool, bool>);
impl MODFDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODFDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODFDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub struct MODFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFDIS_W<'a> {
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
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub struct WDRBT_R(crate::FieldReader<bool, bool>);
impl WDRBT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDRBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRBT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub struct WDRBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRBT_W<'a> {
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
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub struct LLB_R(crate::FieldReader<bool, bool>);
impl LLB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LLB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub struct LLB_W<'a> {
    w: &'a mut W,
}
impl<'a> LLB_W<'a> {
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
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPMODE_A {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison condition must be met to start reception of all incoming characters until REQCLR is set."]
    START_CONDITION = 1,
}
impl From<CMPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub struct CMPMODE_R(crate::FieldReader<bool, CMPMODE_A>);
impl CMPMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            false => CMPMODE_A::FLAG_ONLY,
            true => CMPMODE_A::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        **self == CMPMODE_A::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        **self == CMPMODE_A::START_CONDITION
    }
}
impl core::ops::Deref for CMPMODE_R {
    type Target = crate::FieldReader<bool, CMPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPMODE_A::FLAG_ONLY)
    }
    #[doc = "Comparison condition must be met to start reception of all incoming characters until REQCLR is set."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPMODE_A::START_CONDITION)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub struct PCS_R(crate::FieldReader<u8, u8>);
impl PCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub struct DLYBCS_R(crate::FieldReader<u8, u8>);
impl DLYBCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub struct DLYBCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&self) -> BRSRCCLK_R {
        BRSRCCLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DLYBCS_R {
        DLYBCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PCSDEC_W {
        PCSDEC_W { w: self }
    }
    #[doc = "Bit 3 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&mut self) -> BRSRCCLK_W {
        BRSRCCLK_W { w: self }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> MODFDIS_W {
        MODFDIS_W { w: self }
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W {
        WDRBT_W { w: self }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W {
        LLB_W { w: self }
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DLYBCS_W {
        DLYBCS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
