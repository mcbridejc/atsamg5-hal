#[doc = "Register `CMPR` reader"]
pub struct R(crate::R<CMPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPR` writer"]
pub struct W(crate::W<CMPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPR_SPEC>;
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
impl From<crate::W<CMPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub struct VAL1_R(crate::FieldReader<u16, u16>);
impl VAL1_R {
    pub(crate) fn new(bits: u16) -> Self {
        VAL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub struct VAL1_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub struct VAL2_R(crate::FieldReader<u16, u16>);
impl VAL2_R {
    pub(crate) fn new(bits: u16) -> Self {
        VAL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub struct VAL2_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> VAL2_R {
        VAL2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> VAL1_W {
        VAL1_W { w: self }
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> VAL2_W {
        VAL2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Comparison Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpr](index.html) module"]
pub struct CMPR_SPEC;
impl crate::RegisterSpec for CMPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpr::R](R) reader structure"]
impl crate::Readable for CMPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpr::W](W) writer structure"]
impl crate::Writable for CMPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPR to value 0"]
impl crate::Resettable for CMPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
