#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCLKDIS` reader - Disable Clock Gating"]
pub struct GCLKDIS_R(crate::FieldReader<bool, bool>);
impl GCLKDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCLKDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLKDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLKDIS` writer - Disable Clock Gating"]
pub struct GCLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> GCLKDIS_W<'a> {
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
#[doc = "Field `ICDIS` reader - "]
pub struct ICDIS_R(crate::FieldReader<bool, bool>);
impl ICDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICDIS` writer - "]
pub struct ICDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ICDIS_W<'a> {
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
#[doc = "Field `DCDIS` reader - "]
pub struct DCDIS_R(crate::FieldReader<bool, bool>);
impl DCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDIS` writer - "]
pub struct DCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDIS_W<'a> {
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
#[doc = "Field `PRGCSIZE` reader - "]
pub struct PRGCSIZE_R(crate::FieldReader<u8, u8>);
impl PRGCSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRGCSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRGCSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGCSIZE` writer - "]
pub struct PRGCSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGCSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    pub fn gclkdis(&self) -> GCLKDIS_R {
        GCLKDIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icdis(&self) -> ICDIS_R {
        ICDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdis(&self) -> DCDIS_R {
        DCDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn prgcsize(&self) -> PRGCSIZE_R {
        PRGCSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Clock Gating"]
    #[inline(always)]
    pub fn gclkdis(&mut self) -> GCLKDIS_W {
        GCLKDIS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn icdis(&mut self) -> ICDIS_W {
        ICDIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdis(&mut self) -> DCDIS_W {
        DCDIS_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn prgcsize(&mut self) -> PRGCSIZE_W {
        PRGCSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Controller Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0x20"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
