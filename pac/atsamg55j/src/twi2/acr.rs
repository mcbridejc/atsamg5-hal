#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAL` reader - Data Length"]
pub struct DATAL_R(crate::FieldReader<u8, u8>);
impl DATAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAL` writer - Data Length"]
pub struct DATAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DIR` reader - Transfer Direction"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - Transfer Direction"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PEC` reader - PEC Request (SMBus Mode only)"]
pub struct PEC_R(crate::FieldReader<bool, bool>);
impl PEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEC` writer - PEC Request (SMBus Mode only)"]
pub struct PEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `NDATAL` reader - Next Data Length"]
pub struct NDATAL_R(crate::FieldReader<u8, u8>);
impl NDATAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDATAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDATAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDATAL` writer - Next Data Length"]
pub struct NDATAL_W<'a> {
    w: &'a mut W,
}
impl<'a> NDATAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `NDIR` reader - Next Transfer Direction"]
pub struct NDIR_R(crate::FieldReader<bool, bool>);
impl NDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDIR` writer - Next Transfer Direction"]
pub struct NDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> NDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NPEC` reader - Next PEC Request (SMBus Mode only)"]
pub struct NPEC_R(crate::FieldReader<bool, bool>);
impl NPEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPEC` writer - Next PEC Request (SMBus Mode only)"]
pub struct NPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NPEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn datal(&self) -> DATAL_R {
        DATAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    pub fn ndatal(&self) -> NDATAL_R {
        NDATAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    pub fn ndir(&self) -> NDIR_R {
        NDIR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn npec(&self) -> NPEC_R {
        NPEC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn datal(&mut self) -> DATAL_W {
        DATAL_W { w: self }
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W {
        PEC_W { w: self }
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    pub fn ndatal(&mut self) -> NDATAL_W {
        NDATAL_W { w: self }
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    pub fn ndir(&mut self) -> NDIR_W {
        NDIR_W { w: self }
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn npec(&mut self) -> NPEC_W {
        NPEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWI Alternative Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
