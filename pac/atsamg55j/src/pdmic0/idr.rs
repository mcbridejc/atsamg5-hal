#[doc = "Register `IDR` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRDY` writer - Data Ready Interrupt Disable"]
pub struct DRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DRDY_W<'a> {
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
#[doc = "Field `OVRE` writer - General Overrun Error Interrupt Disable"]
pub struct OVRE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRE_W<'a> {
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
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
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
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub struct RXBUFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFF_W<'a> {
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
impl W {
    #[doc = "Bit 24 - Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DRDY_W {
        DRDY_W { w: self }
    }
    #[doc = "Bit 25 - General Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OVRE_W {
        OVRE_W { w: self }
    }
    #[doc = "Bit 27 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 28 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RXBUFF_W {
        RXBUFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
}
