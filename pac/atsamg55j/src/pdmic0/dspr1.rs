#[doc = "Register `DSPR1` reader"]
pub struct R(crate::R<DSPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSPR1` writer"]
pub struct W(crate::W<DSPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSPR1_SPEC>;
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
impl From<crate::W<DSPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DGAIN` reader - Gain Correction"]
pub struct DGAIN_R(crate::FieldReader<u16, u16>);
impl DGAIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        DGAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DGAIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DGAIN` writer - Gain Correction"]
pub struct DGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DGAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
#[doc = "Field `OFFSET` reader - Offset Correction"]
pub struct OFFSET_R(crate::FieldReader<u16, u16>);
impl OFFSET_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET` writer - Offset Correction"]
pub struct OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Gain Correction"]
    #[inline(always)]
    pub fn dgain(&self) -> DGAIN_R {
        DGAIN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:31 - Offset Correction"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Gain Correction"]
    #[inline(always)]
    pub fn dgain(&mut self) -> DGAIN_W {
        DGAIN_W { w: self }
    }
    #[doc = "Bits 16:31 - Offset Correction"]
    #[inline(always)]
    pub fn offset(&mut self) -> OFFSET_W {
        OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSP Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dspr1](index.html) module"]
pub struct DSPR1_SPEC;
impl crate::RegisterSpec for DSPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dspr1::R](R) reader structure"]
impl crate::Readable for DSPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dspr1::W](W) writer structure"]
impl crate::Writable for DSPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSPR1 to value 0x01"]
impl crate::Resettable for DSPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
