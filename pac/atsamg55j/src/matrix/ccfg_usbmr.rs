#[doc = "Register `CCFG_USBMR` reader"]
pub struct R(crate::R<CCFG_USBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_USBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_USBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_USBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_USBMR` writer"]
pub struct W(crate::W<CCFG_USBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_USBMR_SPEC>;
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
impl From<crate::W<CCFG_USBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_USBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBMODE` reader - USB Mode Selection"]
pub struct USBMODE_R(crate::FieldReader<bool, bool>);
impl USBMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBMODE` writer - USB Mode Selection"]
pub struct USBMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBMODE_W<'a> {
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
#[doc = "Field `USBHTSSC` reader - USB Transceiver Suspend Software Control"]
pub struct USBHTSSC_R(crate::FieldReader<bool, bool>);
impl USBHTSSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHTSSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBHTSSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHTSSC` writer - USB Transceiver Suspend Software Control"]
pub struct USBHTSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHTSSC_W<'a> {
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
#[doc = "Field `USBHTSC` reader - USB Host Transceiver Suspend Control"]
pub struct USBHTSC_R(crate::FieldReader<bool, bool>);
impl USBHTSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBHTSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBHTSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBHTSC` writer - USB Host Transceiver Suspend Control"]
pub struct USBHTSC_W<'a> {
    w: &'a mut W,
}
impl<'a> USBHTSC_W<'a> {
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
impl R {
    #[doc = "Bit 0 - USB Mode Selection"]
    #[inline(always)]
    pub fn usbmode(&self) -> USBMODE_R {
        USBMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB Transceiver Suspend Software Control"]
    #[inline(always)]
    pub fn usbhtssc(&self) -> USBHTSSC_R {
        USBHTSSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Host Transceiver Suspend Control"]
    #[inline(always)]
    pub fn usbhtsc(&self) -> USBHTSC_R {
        USBHTSC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Mode Selection"]
    #[inline(always)]
    pub fn usbmode(&mut self) -> USBMODE_W {
        USBMODE_W { w: self }
    }
    #[doc = "Bit 1 - USB Transceiver Suspend Software Control"]
    #[inline(always)]
    pub fn usbhtssc(&mut self) -> USBHTSSC_W {
        USBHTSSC_W { w: self }
    }
    #[doc = "Bit 2 - USB Host Transceiver Suspend Control"]
    #[inline(always)]
    pub fn usbhtsc(&mut self) -> USBHTSC_W {
        USBHTSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Management Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_usbmr](index.html) module"]
pub struct CCFG_USBMR_SPEC;
impl crate::RegisterSpec for CCFG_USBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_usbmr::R](R) reader structure"]
impl crate::Readable for CCFG_USBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_usbmr::W](W) writer structure"]
impl crate::Writable for CCFG_USBMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_USBMR to value 0"]
impl crate::Resettable for CCFG_USBMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
