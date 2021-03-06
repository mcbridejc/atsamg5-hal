#[doc = "Register `CCFG_SYSIO` reader"]
pub struct R(crate::R<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_SYSIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_SYSIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_SYSIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_SYSIO` writer"]
pub struct W(crate::W<CCFG_SYSIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_SYSIO_SPEC>;
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
impl From<crate::W<CCFG_SYSIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_SYSIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSIO4` reader - PB4 or TDI Assignment"]
pub struct SYSIO4_R(crate::FieldReader<bool, bool>);
impl SYSIO4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO4` writer - PB4 or TDI Assignment"]
pub struct SYSIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SYSIO5` reader - PB5 or TDO/TRACESWO Assignment"]
pub struct SYSIO5_R(crate::FieldReader<bool, bool>);
impl SYSIO5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO5` writer - PB5 or TDO/TRACESWO Assignment"]
pub struct SYSIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SYSIO6` reader - PB6 or TMS/SWDIO Assignment"]
pub struct SYSIO6_R(crate::FieldReader<bool, bool>);
impl SYSIO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO6` writer - PB6 or TMS/SWDIO Assignment"]
pub struct SYSIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `SYSIO7` reader - PB7 or TCK/SWCLK Assignment"]
pub struct SYSIO7_R(crate::FieldReader<bool, bool>);
impl SYSIO7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO7` writer - PB7 or TCK/SWCLK Assignment"]
pub struct SYSIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO7_W<'a> {
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
#[doc = "Field `SYSIO10` reader - PA21 or DM Assignment"]
pub struct SYSIO10_R(crate::FieldReader<bool, bool>);
impl SYSIO10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO10` writer - PA21 or DM Assignment"]
pub struct SYSIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO10_W<'a> {
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
#[doc = "Field `SYSIO11` reader - PA22 or DP Assignment"]
pub struct SYSIO11_R(crate::FieldReader<bool, bool>);
impl SYSIO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO11` writer - PA22 or DP Assignment"]
pub struct SYSIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO11_W<'a> {
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
#[doc = "Field `SYSIO12` reader - PB12 or ERASE Assignment"]
pub struct SYSIO12_R(crate::FieldReader<bool, bool>);
impl SYSIO12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSIO12` writer - PB12 or ERASE Assignment"]
pub struct SYSIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO12_W<'a> {
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
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> SYSIO4_R {
        SYSIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> SYSIO5_R {
        SYSIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> SYSIO6_R {
        SYSIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> SYSIO7_R {
        SYSIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PA21 or DM Assignment"]
    #[inline(always)]
    pub fn sysio10(&self) -> SYSIO10_R {
        SYSIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PA22 or DP Assignment"]
    #[inline(always)]
    pub fn sysio11(&self) -> SYSIO11_R {
        SYSIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&mut self) -> SYSIO4_W {
        SYSIO4_W { w: self }
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&mut self) -> SYSIO5_W {
        SYSIO5_W { w: self }
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&mut self) -> SYSIO6_W {
        SYSIO6_W { w: self }
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&mut self) -> SYSIO7_W {
        SYSIO7_W { w: self }
    }
    #[doc = "Bit 10 - PA21 or DM Assignment"]
    #[inline(always)]
    pub fn sysio10(&mut self) -> SYSIO10_W {
        SYSIO10_W { w: self }
    }
    #[doc = "Bit 11 - PA22 or DP Assignment"]
    #[inline(always)]
    pub fn sysio11(&mut self) -> SYSIO11_W {
        SYSIO11_W { w: self }
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> SYSIO12_W {
        SYSIO12_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System I/O Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_sysio](index.html) module"]
pub struct CCFG_SYSIO_SPEC;
impl crate::RegisterSpec for CCFG_SYSIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_sysio::R](R) reader structure"]
impl crate::Readable for CCFG_SYSIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_sysio::W](W) writer structure"]
impl crate::Writable for CCFG_SYSIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_SYSIO to value 0x0c00"]
impl crate::Resettable for CCFG_SYSIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c00
    }
}
