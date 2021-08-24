#[doc = "Register `LCCWR` reader"]
pub struct R(crate::R<LCCWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCCWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCCWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCCWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCCWR` writer"]
pub struct W(crate::W<LCCWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCCWR_SPEC>;
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
impl From<crate::W<LCCWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCCWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWTHRES` reader - Low Threshold"]
pub struct LOWTHRES_R(crate::FieldReader<u16, u16>);
impl LOWTHRES_R {
    pub(crate) fn new(bits: u16) -> Self {
        LOWTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOWTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWTHRES` writer - Low Threshold"]
pub struct LOWTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `HIGHTHRES` reader - High Threshold"]
pub struct HIGHTHRES_R(crate::FieldReader<u16, u16>);
impl HIGHTHRES_R {
    pub(crate) fn new(bits: u16) -> Self {
        HIGHTHRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHTHRES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGHTHRES` writer - High Threshold"]
pub struct HIGHTHRES_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHTHRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&self) -> LOWTHRES_R {
        LOWTHRES_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&self) -> HIGHTHRES_R {
        HIGHTHRES_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low Threshold"]
    #[inline(always)]
    pub fn lowthres(&mut self) -> LOWTHRES_W {
        LOWTHRES_W { w: self }
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn highthres(&mut self) -> HIGHTHRES_W {
        HIGHTHRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last Channel Compare Window Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lccwr](index.html) module"]
pub struct LCCWR_SPEC;
impl crate::RegisterSpec for LCCWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lccwr::R](R) reader structure"]
impl crate::Readable for LCCWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lccwr::W](W) writer structure"]
impl crate::Writable for LCCWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCCWR to value 0"]
impl crate::Resettable for LCCWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}