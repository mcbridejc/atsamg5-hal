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
#[doc = "POR Core Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTEN_A {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    NOT_ENABLE = 0,
    #[doc = "1: The core reset signal vddcore_nreset is asserted when a brownout detection occurs."]
    ENABLE = 1,
}
impl From<BODRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTEN` reader - POR Core Reset Enable"]
pub struct BODRSTEN_R(crate::FieldReader<bool, BODRSTEN_A>);
impl BODRSTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTEN_A {
        match self.bits {
            false => BODRSTEN_A::NOT_ENABLE,
            true => BODRSTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENABLE`"]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        **self == BODRSTEN_A::NOT_ENABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODRSTEN_A::ENABLE
    }
}
impl core::ops::Deref for BODRSTEN_R {
    type Target = crate::FieldReader<bool, BODRSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRSTEN` writer - POR Core Reset Enable"]
pub struct BODRSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a brownout detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut W {
        self.variant(BODRSTEN_A::NOT_ENABLE)
    }
    #[doc = "The core reset signal vddcore_nreset is asserted when a brownout detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTEN_A::ENABLE)
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
#[doc = "POR Core Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODDIS_A {
    #[doc = "0: The core brownout detector is enabled."]
    ENABLE = 0,
    #[doc = "1: The core brownout detector is disabled."]
    DISABLE = 1,
}
impl From<BODDIS_A> for bool {
    #[inline(always)]
    fn from(variant: BODDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODDIS` reader - POR Core Disable"]
pub struct BODDIS_R(crate::FieldReader<bool, BODDIS_A>);
impl BODDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODDIS_A {
        match self.bits {
            false => BODDIS_A::ENABLE,
            true => BODDIS_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == BODDIS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == BODDIS_A::DISABLE
    }
}
impl core::ops::Deref for BODDIS_R {
    type Target = crate::FieldReader<bool, BODDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODDIS` writer - POR Core Disable"]
pub struct BODDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BODDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The core brownout detector is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODDIS_A::ENABLE)
    }
    #[doc = "The core brownout detector is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODDIS_A::DISABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCBYPASS_A {
    #[doc = "0: No effect. Clock selection depends on XTALSEL value."]
    NO_EFFECT = 0,
    #[doc = "1: The 32 kHz crystal oscillator is bypassed if XTALSEL=1. OSCBYPASS must be set prior to write XTALSEL=1."]
    BYPASS = 1,
}
impl From<OSCBYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: OSCBYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub struct OSCBYPASS_R(crate::FieldReader<bool, OSCBYPASS_A>);
impl OSCBYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCBYPASS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCBYPASS_A {
        match self.bits {
            false => OSCBYPASS_A::NO_EFFECT,
            true => OSCBYPASS_A::BYPASS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == OSCBYPASS_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        **self == OSCBYPASS_A::BYPASS
    }
}
impl core::ops::Deref for OSCBYPASS_R {
    type Target = crate::FieldReader<bool, OSCBYPASS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub struct OSCBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCBYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCBYPASS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect. Clock selection depends on XTALSEL value."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(OSCBYPASS_A::NO_EFFECT)
    }
    #[doc = "The 32 kHz crystal oscillator is bypassed if XTALSEL=1. OSCBYPASS must be set prior to write XTALSEL=1."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OSCBYPASS_A::BYPASS)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Cache Data SRAM Power Switch\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDPSWITCH_A {
    #[doc = "0: The cache data SRAM is not powered."]
    OFF = 0,
    #[doc = "1: The cache data SRAM is powered."]
    ON = 1,
}
impl From<CDPSWITCH_A> for bool {
    #[inline(always)]
    fn from(variant: CDPSWITCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDPSWITCH` reader - Cache Data SRAM Power Switch"]
pub struct CDPSWITCH_R(crate::FieldReader<bool, CDPSWITCH_A>);
impl CDPSWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        CDPSWITCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDPSWITCH_A {
        match self.bits {
            false => CDPSWITCH_A::OFF,
            true => CDPSWITCH_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CDPSWITCH_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CDPSWITCH_A::ON
    }
}
impl core::ops::Deref for CDPSWITCH_R {
    type Target = crate::FieldReader<bool, CDPSWITCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CDPSWITCH` writer - Cache Data SRAM Power Switch"]
pub struct CDPSWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CDPSWITCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDPSWITCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The cache data SRAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CDPSWITCH_A::OFF)
    }
    #[doc = "The cache data SRAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CDPSWITCH_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Cache Tag SRAM Power Switch\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTPSWITCH_A {
    #[doc = "0: The cache tag SRAM is not powered."]
    OFF = 0,
    #[doc = "1: The cache tag SRAM is powered."]
    ON = 1,
}
impl From<CTPSWITCH_A> for bool {
    #[inline(always)]
    fn from(variant: CTPSWITCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTPSWITCH` reader - Cache Tag SRAM Power Switch"]
pub struct CTPSWITCH_R(crate::FieldReader<bool, CTPSWITCH_A>);
impl CTPSWITCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTPSWITCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTPSWITCH_A {
        match self.bits {
            false => CTPSWITCH_A::OFF,
            true => CTPSWITCH_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == CTPSWITCH_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == CTPSWITCH_A::ON
    }
}
impl core::ops::Deref for CTPSWITCH_R {
    type Target = crate::FieldReader<bool, CTPSWITCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTPSWITCH` writer - Cache Tag SRAM Power Switch"]
pub struct CTPSWITCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTPSWITCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTPSWITCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The cache tag SRAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(CTPSWITCH_A::OFF)
    }
    #[doc = "The cache tag SRAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(CTPSWITCH_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ONE` reader - This bit must always be set to 1."]
pub struct ONE_R(crate::FieldReader<bool, bool>);
impl ONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONE` writer - This bit must always be set to 1."]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "165: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 165,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Password Key"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            165 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == KEY_A::PASSWD
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - POR Core Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&self) -> BODRSTEN_R {
        BODRSTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - POR Core Disable"]
    #[inline(always)]
    pub fn boddis(&self) -> BODDIS_R {
        BODDIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OSCBYPASS_R {
        OSCBYPASS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Cache Data SRAM Power Switch"]
    #[inline(always)]
    pub fn cdpswitch(&self) -> CDPSWITCH_R {
        CDPSWITCH_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Cache Tag SRAM Power Switch"]
    #[inline(always)]
    pub fn ctpswitch(&self) -> CTPSWITCH_R {
        CTPSWITCH_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - This bit must always be set to 1."]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - POR Core Reset Enable"]
    #[inline(always)]
    pub fn bodrsten(&mut self) -> BODRSTEN_W {
        BODRSTEN_W { w: self }
    }
    #[doc = "Bit 13 - POR Core Disable"]
    #[inline(always)]
    pub fn boddis(&mut self) -> BODDIS_W {
        BODDIS_W { w: self }
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&mut self) -> OSCBYPASS_W {
        OSCBYPASS_W { w: self }
    }
    #[doc = "Bit 21 - Cache Data SRAM Power Switch"]
    #[inline(always)]
    pub fn cdpswitch(&mut self) -> CDPSWITCH_W {
        CDPSWITCH_W { w: self }
    }
    #[doc = "Bit 22 - Cache Tag SRAM Power Switch"]
    #[inline(always)]
    pub fn ctpswitch(&mut self) -> CTPSWITCH_W {
        CTPSWITCH_W { w: self }
    }
    #[doc = "Bit 23 - This bit must always be set to 1."]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
#[doc = "`reset()` method sets MR to value 0x00e0_5a00"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00e0_5a00
    }
}
