#[doc = "Register `HCDONEHEAD` reader"]
pub struct R(crate::R<HCDONEHEAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDONEHEAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDONEHEAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDONEHEAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DH` reader - Physical address of the last TD that has added to the done queue"]
pub struct DH_R(crate::FieldReader<u32, u32>);
impl DH_R {
    pub(crate) fn new(bits: u32) -> Self {
        DH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DH_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:31 - Physical address of the last TD that has added to the done queue"]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
#[doc = "HC Head Done Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdonehead](index.html) module"]
pub struct HCDONEHEAD_SPEC;
impl crate::RegisterSpec for HCDONEHEAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdonehead::R](R) reader structure"]
impl crate::Readable for HCDONEHEAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCDONEHEAD to value 0"]
impl crate::Resettable for HCDONEHEAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
