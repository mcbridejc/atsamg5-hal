#[doc = "Register `PTCR` writer"]
pub struct W(crate::W<PTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTCR_SPEC>;
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
impl From<crate::W<PTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTEN` writer - Receiver Transfer Enable"]
pub struct RXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTEN_W<'a> {
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
#[doc = "Field `RXTDIS` writer - Receiver Transfer Disable"]
pub struct RXTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTDIS_W<'a> {
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
#[doc = "Field `TXTEN` writer - Transmitter Transfer Enable"]
pub struct TXTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTEN_W<'a> {
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
#[doc = "Field `TXTDIS` writer - Transmitter Transfer Disable"]
pub struct TXTDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTDIS_W<'a> {
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
#[doc = "Field `RXCBEN` writer - Receiver Circular Buffer Enable"]
pub struct RXCBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCBEN_W<'a> {
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
#[doc = "Field `RXCBDIS` writer - Receiver Circular Buffer Disable"]
pub struct RXCBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCBDIS_W<'a> {
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
#[doc = "Field `TXCBEN` writer - Transmitter Circular Buffer Enable"]
pub struct TXCBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCBEN_W<'a> {
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
#[doc = "Field `TXCBDIS` writer - Transmitter Circular Buffer Disable"]
pub struct TXCBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCBDIS_W<'a> {
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
#[doc = "Field `ERRCLR` writer - Transfer Bus Error Clear"]
pub struct ERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCLR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RXTEN_W {
        RXTEN_W { w: self }
    }
    #[doc = "Bit 1 - Receiver Transfer Disable"]
    #[inline(always)]
    pub fn rxtdis(&mut self) -> RXTDIS_W {
        RXTDIS_W { w: self }
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TXTEN_W {
        TXTEN_W { w: self }
    }
    #[doc = "Bit 9 - Transmitter Transfer Disable"]
    #[inline(always)]
    pub fn txtdis(&mut self) -> TXTDIS_W {
        TXTDIS_W { w: self }
    }
    #[doc = "Bit 16 - Receiver Circular Buffer Enable"]
    #[inline(always)]
    pub fn rxcben(&mut self) -> RXCBEN_W {
        RXCBEN_W { w: self }
    }
    #[doc = "Bit 17 - Receiver Circular Buffer Disable"]
    #[inline(always)]
    pub fn rxcbdis(&mut self) -> RXCBDIS_W {
        RXCBDIS_W { w: self }
    }
    #[doc = "Bit 18 - Transmitter Circular Buffer Enable"]
    #[inline(always)]
    pub fn txcben(&mut self) -> TXCBEN_W {
        TXCBEN_W { w: self }
    }
    #[doc = "Bit 19 - Transmitter Circular Buffer Disable"]
    #[inline(always)]
    pub fn txcbdis(&mut self) -> TXCBDIS_W {
        TXCBDIS_W { w: self }
    }
    #[doc = "Bit 24 - Transfer Bus Error Clear"]
    #[inline(always)]
    pub fn errclr(&mut self) -> ERRCLR_W {
        ERRCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptcr](index.html) module"]
pub struct PTCR_SPEC;
impl crate::RegisterSpec for PTCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ptcr::W](W) writer structure"]
impl crate::Writable for PTCR_SPEC {
    type Writer = W;
}
