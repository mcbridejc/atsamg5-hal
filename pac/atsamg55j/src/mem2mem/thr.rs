#[doc = "Register `THR` reader"]
pub struct R(crate::R<THR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THR` writer"]
pub struct W(crate::W<THR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR_SPEC>;
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
impl From<crate::W<THR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THDATA` reader - Transfer Holding Data"]
pub struct THDATA_R(crate::FieldReader<u32, u32>);
impl THDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        THDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THDATA` writer - Transfer Holding Data"]
pub struct THDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> THDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Transfer Holding Data"]
    #[inline(always)]
    pub fn thdata(&self) -> THDATA_R {
        THDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transfer Holding Data"]
    #[inline(always)]
    pub fn thdata(&mut self) -> THDATA_W {
        THDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory to Memory Transfer Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](index.html) module"]
pub struct THR_SPEC;
impl crate::RegisterSpec for THR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thr::R](R) reader structure"]
impl crate::Readable for THR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thr::W](W) writer structure"]
impl crate::Writable for THR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for THR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
