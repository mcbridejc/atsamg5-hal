#[doc = "Register `CCFG_I2SCLKSEL` reader"]
pub struct R(crate::R<CCFG_I2SCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_I2SCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_I2SCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_I2SCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_I2SCLKSEL` writer"]
pub struct W(crate::W<CCFG_I2SCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_I2SCLKSEL_SPEC>;
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
impl From<crate::W<CCFG_I2SCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_I2SCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL0` reader - I2S0 Clock Source"]
pub struct CLKSEL0_R(crate::FieldReader<bool, bool>);
impl CLKSEL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSEL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL0` writer - I2S0 Clock Source"]
pub struct CLKSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL0_W<'a> {
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
#[doc = "Field `CLKSEL1` reader - I2S1 Clock Source"]
pub struct CLKSEL1_R(crate::FieldReader<bool, bool>);
impl CLKSEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSEL1` writer - I2S1 Clock Source"]
pub struct CLKSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2S0 Clock Source"]
    #[inline(always)]
    pub fn clksel0(&self) -> CLKSEL0_R {
        CLKSEL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2S1 Clock Source"]
    #[inline(always)]
    pub fn clksel1(&self) -> CLKSEL1_R {
        CLKSEL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S0 Clock Source"]
    #[inline(always)]
    pub fn clksel0(&mut self) -> CLKSEL0_W {
        CLKSEL0_W { w: self }
    }
    #[doc = "Bit 1 - I2S1 Clock Source"]
    #[inline(always)]
    pub fn clksel1(&mut self) -> CLKSEL1_W {
        CLKSEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Source Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_i2sclksel](index.html) module"]
pub struct CCFG_I2SCLKSEL_SPEC;
impl crate::RegisterSpec for CCFG_I2SCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_i2sclksel::R](R) reader structure"]
impl crate::Readable for CCFG_I2SCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_i2sclksel::W](W) writer structure"]
impl crate::Writable for CCFG_I2SCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCFG_I2SCLKSEL to value 0"]
impl crate::Resettable for CCFG_I2SCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
