#[doc = "Register `HCRHDESCRIPTORA` reader"]
pub struct R(crate::R<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHDESCRIPTORA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHDESCRIPTORA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORA` writer"]
pub struct W(crate::W<HCRHDESCRIPTORA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORA_SPEC>;
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
impl From<crate::W<HCRHDESCRIPTORA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHDESCRIPTORA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDP` reader - Number of downstream ports (read-only)"]
pub struct NDP_R(crate::FieldReader<u8, u8>);
impl NDP_R {
    pub(crate) fn new(bits: u8) -> Self {
        NDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDP` writer - Number of downstream ports (read-only)"]
pub struct NDP_W<'a> {
    w: &'a mut W,
}
impl<'a> NDP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `PSM` reader - Power switching mode (read/write)"]
pub struct PSM_R(crate::FieldReader<bool, bool>);
impl PSM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSM` writer - Power switching mode (read/write)"]
pub struct PSM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSM_W<'a> {
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
#[doc = "Field `NPS` reader - No power switching (read/write)"]
pub struct NPS_R(crate::FieldReader<bool, bool>);
impl NPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPS` writer - No power switching (read/write)"]
pub struct NPS_W<'a> {
    w: &'a mut W,
}
impl<'a> NPS_W<'a> {
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
#[doc = "Field `DT` reader - Device type (read-only)"]
pub struct DT_R(crate::FieldReader<bool, bool>);
impl DT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT` writer - Device type (read-only)"]
pub struct DT_W<'a> {
    w: &'a mut W,
}
impl<'a> DT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `OCPM` reader - Overcurrent protection mode (read/write)"]
pub struct OCPM_R(crate::FieldReader<bool, bool>);
impl OCPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCPM` writer - Overcurrent protection mode (read/write)"]
pub struct OCPM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NOCP` reader - No overcurrent protection (read/write)"]
pub struct NOCP_R(crate::FieldReader<bool, bool>);
impl NOCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOCP` writer - No overcurrent protection (read/write)"]
pub struct NOCP_W<'a> {
    w: &'a mut W,
}
impl<'a> NOCP_W<'a> {
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
#[doc = "Field `POTPG` reader - Power-on to power-good time (read/write)"]
pub struct POTPG_R(crate::FieldReader<u8, u8>);
impl POTPG_R {
    pub(crate) fn new(bits: u8) -> Self {
        POTPG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POTPG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POTPG` writer - Power-on to power-good time (read/write)"]
pub struct POTPG_W<'a> {
    w: &'a mut W,
}
impl<'a> POTPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of downstream ports (read-only)"]
    #[inline(always)]
    pub fn ndp(&self) -> NDP_R {
        NDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Power switching mode (read/write)"]
    #[inline(always)]
    pub fn psm(&self) -> PSM_R {
        PSM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - No power switching (read/write)"]
    #[inline(always)]
    pub fn nps(&self) -> NPS_R {
        NPS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Device type (read-only)"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Overcurrent protection mode (read/write)"]
    #[inline(always)]
    pub fn ocpm(&self) -> OCPM_R {
        OCPM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - No overcurrent protection (read/write)"]
    #[inline(always)]
    pub fn nocp(&self) -> NOCP_R {
        NOCP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Power-on to power-good time (read/write)"]
    #[inline(always)]
    pub fn potpg(&self) -> POTPG_R {
        POTPG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of downstream ports (read-only)"]
    #[inline(always)]
    pub fn ndp(&mut self) -> NDP_W {
        NDP_W { w: self }
    }
    #[doc = "Bit 8 - Power switching mode (read/write)"]
    #[inline(always)]
    pub fn psm(&mut self) -> PSM_W {
        PSM_W { w: self }
    }
    #[doc = "Bit 9 - No power switching (read/write)"]
    #[inline(always)]
    pub fn nps(&mut self) -> NPS_W {
        NPS_W { w: self }
    }
    #[doc = "Bit 10 - Device type (read-only)"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W {
        DT_W { w: self }
    }
    #[doc = "Bit 11 - Overcurrent protection mode (read/write)"]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OCPM_W {
        OCPM_W { w: self }
    }
    #[doc = "Bit 12 - No overcurrent protection (read/write)"]
    #[inline(always)]
    pub fn nocp(&mut self) -> NOCP_W {
        NOCP_W { w: self }
    }
    #[doc = "Bits 24:31 - Power-on to power-good time (read/write)"]
    #[inline(always)]
    pub fn potpg(&mut self) -> POTPG_W {
        POTPG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HC Root Hub A Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptora](index.html) module"]
pub struct HCRHDESCRIPTORA_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptora::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptora::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORA to value 0x0a00_1203"]
impl crate::Resettable for HCRHDESCRIPTORA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a00_1203
    }
}
