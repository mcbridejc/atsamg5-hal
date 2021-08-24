#[doc = "Register `PMC_OCR` reader"]
pub struct R(crate::R<PMC_OCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_OCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_OCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_OCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_OCR` writer"]
pub struct W(crate::W<PMC_OCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_OCR_SPEC>;
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
impl From<crate::W<PMC_OCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_OCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL8` reader - RC Oscillator Calibration bits for 8 MHz"]
pub struct CAL8_R(crate::FieldReader<u8, u8>);
impl CAL8_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL8` writer - RC Oscillator Calibration bits for 8 MHz"]
pub struct CAL8_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `SEL8` reader - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub struct SEL8_R(crate::FieldReader<bool, bool>);
impl SEL8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL8` writer - Selection of RC Oscillator Calibration bits for 8 MHz"]
pub struct SEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL8_W<'a> {
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
#[doc = "Field `CAL16` reader - RC Oscillator Calibration bits for 16 MHz"]
pub struct CAL16_R(crate::FieldReader<u8, u8>);
impl CAL16_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAL16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL16` writer - RC Oscillator Calibration bits for 16 MHz"]
pub struct CAL16_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `SEL16` reader - Selection of RC Oscillator Calibration bits for 16 MHz"]
pub struct SEL16_R(crate::FieldReader<bool, bool>);
impl SEL16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL16` writer - Selection of RC Oscillator Calibration bits for 16 MHz"]
pub struct SEL16_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL16_W<'a> {
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
#[doc = "Field `CAL24` reader - RC Oscillator Calibration bits for 24 MHz"]
pub struct CAL24_R(crate::FieldReader<u8, u8>);
impl CAL24_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAL24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL24` writer - RC Oscillator Calibration bits for 24 MHz"]
pub struct CAL24_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `SEL24` reader - Selection of RC Oscillator Calibration bits for 24 MHz"]
pub struct SEL24_R(crate::FieldReader<bool, bool>);
impl SEL24_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL24` writer - Selection of RC Oscillator Calibration bits for 24 MHz"]
pub struct SEL24_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL24_W<'a> {
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
impl R {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn cal16(&self) -> CAL16_R {
        CAL16_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn sel16(&self) -> SEL16_R {
        SEL16_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn cal24(&self) -> CAL24_R {
        CAL24_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn sel24(&self) -> SEL24_R {
        SEL24_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&mut self) -> CAL8_W {
        CAL8_W { w: self }
    }
    #[doc = "Bit 7 - Selection of RC Oscillator Calibration bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&mut self) -> SEL8_W {
        SEL8_W { w: self }
    }
    #[doc = "Bits 8:14 - RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn cal16(&mut self) -> CAL16_W {
        CAL16_W { w: self }
    }
    #[doc = "Bit 15 - Selection of RC Oscillator Calibration bits for 16 MHz"]
    #[inline(always)]
    pub fn sel16(&mut self) -> SEL16_W {
        SEL16_W { w: self }
    }
    #[doc = "Bits 16:22 - RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn cal24(&mut self) -> CAL24_W {
        CAL24_W { w: self }
    }
    #[doc = "Bit 23 - Selection of RC Oscillator Calibration bits for 24 MHz"]
    #[inline(always)]
    pub fn sel24(&mut self) -> SEL24_W {
        SEL24_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_ocr](index.html) module"]
pub struct PMC_OCR_SPEC;
impl crate::RegisterSpec for PMC_OCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_ocr::R](R) reader structure"]
impl crate::Readable for PMC_OCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_ocr::W](W) writer structure"]
impl crate::Writable for PMC_OCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_OCR to value 0x0040_4040"]
impl crate::Resettable for PMC_OCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_4040
    }
}
