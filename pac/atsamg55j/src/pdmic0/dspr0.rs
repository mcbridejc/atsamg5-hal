#[doc = "Register `DSPR0` reader"]
pub struct R(crate::R<DSPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSPR0` writer"]
pub struct W(crate::W<DSPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSPR0_SPEC>;
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
impl From<crate::W<DSPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPFBYP` reader - High-Pass Filter Bypass"]
pub struct HPFBYP_R(crate::FieldReader<bool, bool>);
impl HPFBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPFBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPFBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPFBYP` writer - High-Pass Filter Bypass"]
pub struct HPFBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HPFBYP_W<'a> {
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
#[doc = "Field `SINBYP` reader - SINCC Filter Bypass"]
pub struct SINBYP_R(crate::FieldReader<bool, bool>);
impl SINBYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINBYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINBYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINBYP` writer - SINCC Filter Bypass"]
pub struct SINBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> SINBYP_W<'a> {
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
#[doc = "Field `SIZE` reader - Data Size"]
pub struct SIZE_R(crate::FieldReader<bool, bool>);
impl SIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Data Size"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
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
#[doc = "Oversampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: Oversampling ratio is 128"]
    _128 = 0,
    #[doc = "1: Oversampling ratio is 64"]
    _64 = 1,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSR` reader - Oversampling Ratio"]
pub struct OSR_R(crate::FieldReader<u8, OSR_A>);
impl OSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSR_A> {
        match self.bits {
            0 => Some(OSR_A::_128),
            1 => Some(OSR_A::_64),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == OSR_A::_128
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == OSR_A::_64
    }
}
impl core::ops::Deref for OSR_R {
    type Target = crate::FieldReader<u8, OSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSR` writer - Oversampling Ratio"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Oversampling ratio is 128"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(OSR_A::_128)
    }
    #[doc = "Oversampling ratio is 64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(OSR_A::_64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SCALE` reader - Data Scale"]
pub struct SCALE_R(crate::FieldReader<u8, u8>);
impl SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCALE` writer - Data Scale"]
pub struct SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SHIFT` reader - Data Shift"]
pub struct SHIFT_R(crate::FieldReader<u8, u8>);
impl SHIFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHIFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFT` writer - Data Shift"]
pub struct SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - High-Pass Filter Bypass"]
    #[inline(always)]
    pub fn hpfbyp(&self) -> HPFBYP_R {
        HPFBYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SINCC Filter Bypass"]
    #[inline(always)]
    pub fn sinbyp(&self) -> SINBYP_R {
        SINBYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Data Scale"]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Data Shift"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - High-Pass Filter Bypass"]
    #[inline(always)]
    pub fn hpfbyp(&mut self) -> HPFBYP_W {
        HPFBYP_W { w: self }
    }
    #[doc = "Bit 2 - SINCC Filter Bypass"]
    #[inline(always)]
    pub fn sinbyp(&mut self) -> SINBYP_W {
        SINBYP_W { w: self }
    }
    #[doc = "Bit 3 - Data Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 4:6 - Oversampling Ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bits 8:11 - Data Scale"]
    #[inline(always)]
    pub fn scale(&mut self) -> SCALE_W {
        SCALE_W { w: self }
    }
    #[doc = "Bits 12:15 - Data Shift"]
    #[inline(always)]
    pub fn shift(&mut self) -> SHIFT_W {
        SHIFT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dspr0](index.html) module"]
pub struct DSPR0_SPEC;
impl crate::RegisterSpec for DSPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dspr0::R](R) reader structure"]
impl crate::Readable for DSPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dspr0::W](W) writer structure"]
impl crate::Writable for DSPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSPR0 to value 0"]
impl crate::Resettable for DSPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
