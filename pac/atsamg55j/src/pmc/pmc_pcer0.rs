#[doc = "Register `PMC_PCER0` writer"]
pub struct W(crate::W<PMC_PCER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCER0_SPEC>;
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
impl From<crate::W<PMC_PCER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID8` writer - Peripheral Clock 8 Enable"]
pub struct PID8_W<'a> {
    w: &'a mut W,
}
impl<'a> PID8_W<'a> {
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
#[doc = "Field `PID9` writer - Peripheral Clock 9 Enable"]
pub struct PID9_W<'a> {
    w: &'a mut W,
}
impl<'a> PID9_W<'a> {
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
#[doc = "Field `PID10` writer - Peripheral Clock 10 Enable"]
pub struct PID10_W<'a> {
    w: &'a mut W,
}
impl<'a> PID10_W<'a> {
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
#[doc = "Field `PID11` writer - Peripheral Clock 11 Enable"]
pub struct PID11_W<'a> {
    w: &'a mut W,
}
impl<'a> PID11_W<'a> {
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
#[doc = "Field `PID12` writer - Peripheral Clock 12 Enable"]
pub struct PID12_W<'a> {
    w: &'a mut W,
}
impl<'a> PID12_W<'a> {
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
#[doc = "Field `PID13` writer - Peripheral Clock 13 Enable"]
pub struct PID13_W<'a> {
    w: &'a mut W,
}
impl<'a> PID13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PID14` writer - Peripheral Clock 14 Enable"]
pub struct PID14_W<'a> {
    w: &'a mut W,
}
impl<'a> PID14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PID15` writer - Peripheral Clock 15 Enable"]
pub struct PID15_W<'a> {
    w: &'a mut W,
}
impl<'a> PID15_W<'a> {
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
#[doc = "Field `PID16` writer - Peripheral Clock 16 Enable"]
pub struct PID16_W<'a> {
    w: &'a mut W,
}
impl<'a> PID16_W<'a> {
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
#[doc = "Field `PID17` writer - Peripheral Clock 17 Enable"]
pub struct PID17_W<'a> {
    w: &'a mut W,
}
impl<'a> PID17_W<'a> {
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
#[doc = "Field `PID18` writer - Peripheral Clock 18 Enable"]
pub struct PID18_W<'a> {
    w: &'a mut W,
}
impl<'a> PID18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PID19` writer - Peripheral Clock 19 Enable"]
pub struct PID19_W<'a> {
    w: &'a mut W,
}
impl<'a> PID19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PID20` writer - Peripheral Clock 20 Enable"]
pub struct PID20_W<'a> {
    w: &'a mut W,
}
impl<'a> PID20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PID21` writer - Peripheral Clock 21 Enable"]
pub struct PID21_W<'a> {
    w: &'a mut W,
}
impl<'a> PID21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PID22` writer - Peripheral Clock 22 Enable"]
pub struct PID22_W<'a> {
    w: &'a mut W,
}
impl<'a> PID22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PID23` writer - Peripheral Clock 23 Enable"]
pub struct PID23_W<'a> {
    w: &'a mut W,
}
impl<'a> PID23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PID24` writer - Peripheral Clock 24 Enable"]
pub struct PID24_W<'a> {
    w: &'a mut W,
}
impl<'a> PID24_W<'a> {
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
#[doc = "Field `PID25` writer - Peripheral Clock 25 Enable"]
pub struct PID25_W<'a> {
    w: &'a mut W,
}
impl<'a> PID25_W<'a> {
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
#[doc = "Field `PID26` writer - Peripheral Clock 26 Enable"]
pub struct PID26_W<'a> {
    w: &'a mut W,
}
impl<'a> PID26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PID27` writer - Peripheral Clock 27 Enable"]
pub struct PID27_W<'a> {
    w: &'a mut W,
}
impl<'a> PID27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PID28` writer - Peripheral Clock 28 Enable"]
pub struct PID28_W<'a> {
    w: &'a mut W,
}
impl<'a> PID28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PID29` writer - Peripheral Clock 29 Enable"]
pub struct PID29_W<'a> {
    w: &'a mut W,
}
impl<'a> PID29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl W {
    #[doc = "Bit 8 - Peripheral Clock 8 Enable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W {
        PID8_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral Clock 9 Enable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PID9_W {
        PID9_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Enable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W {
        PID10_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Enable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W {
        PID11_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral Clock 12 Enable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PID12_W {
        PID12_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Enable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W {
        PID13_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Enable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W {
        PID14_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Enable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W {
        PID15_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Enable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PID16_W {
        PID16_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral Clock 17 Enable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> PID17_W {
        PID17_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Enable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PID18_W {
        PID18_W { w: self }
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Enable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PID19_W {
        PID19_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Enable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PID20_W {
        PID20_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Enable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PID21_W {
        PID21_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Enable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PID22_W {
        PID22_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Enable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PID23_W {
        PID23_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Enable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PID24_W {
        PID24_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Enable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PID25_W {
        PID25_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Enable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PID26_W {
        PID26_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Enable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PID27_W {
        PID27_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Enable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PID28_W {
        PID28_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Enable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PID29_W {
        PID29_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcer0](index.html) module"]
pub struct PMC_PCER0_SPEC;
impl crate::RegisterSpec for PMC_PCER0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pcer0::W](W) writer structure"]
impl crate::Writable for PMC_PCER0_SPEC {
    type Writer = W;
}
