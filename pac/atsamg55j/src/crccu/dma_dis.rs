#[doc = "Register `DMA_DIS` writer"]
pub struct W(crate::W<DMA_DIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_DIS_SPEC>;
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
impl From<crate::W<DMA_DIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_DIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMADIS` writer - DMA Disable"]
pub struct DMADIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMADIS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DMA Disable"]
    #[inline(always)]
    pub fn dmadis(&mut self) -> DMADIS_W {
        DMADIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRCCU DMA Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_dis](index.html) module"]
pub struct DMA_DIS_SPEC;
impl crate::RegisterSpec for DMA_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_dis::W](W) writer structure"]
impl crate::Writable for DMA_DIS_SPEC {
    type Writer = W;
}
