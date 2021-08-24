#[doc = "Register `PMC_PCER1` writer"]
pub struct W(crate::W<PMC_PCER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCER1_SPEC>;
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
impl From<crate::W<PMC_PCER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID47` writer - Peripheral Clock 47 Enable"]
pub struct PID47_W<'a> {
    w: &'a mut W,
}
impl<'a> PID47_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PID48` writer - Peripheral Clock 48 Enable"]
pub struct PID48_W<'a> {
    w: &'a mut W,
}
impl<'a> PID48_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PID49` writer - Peripheral Clock 49 Enable"]
pub struct PID49_W<'a> {
    w: &'a mut W,
}
impl<'a> PID49_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl W {
    #[doc = "Bit 15 - Peripheral Clock 47 Enable"]
    #[inline(always)]
    pub fn pid47(&mut self) -> PID47_W {
        PID47_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Enable"]
    #[inline(always)]
    pub fn pid48(&mut self) -> PID48_W {
        PID48_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Enable"]
    #[inline(always)]
    pub fn pid49(&mut self) -> PID49_W {
        PID49_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcer1](index.html) module"]
pub struct PMC_PCER1_SPEC;
impl crate::RegisterSpec for PMC_PCER1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcer1::W](W) writer structure"]
impl crate::Writable for PMC_PCER1_SPEC {
    type Writer = W;
}
