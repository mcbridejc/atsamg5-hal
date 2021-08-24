#[doc = "Register `HCPERIODCURRENTED` reader"]
pub struct R(crate::R<HCPERIODCURRENTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCPERIODCURRENTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCPERIODCURRENTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCPERIODCURRENTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCED` reader - Physical address of the current ED on the periodic ED list"]
pub struct PCED_R(crate::FieldReader<u32, u32>);
impl PCED_R {
    pub(crate) fn new(bits: u32) -> Self {
        PCED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:31 - Physical address of the current ED on the periodic ED list"]
    #[inline(always)]
    pub fn pced(&self) -> PCED_R {
        PCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
#[doc = "HC Current Periodic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcperiodcurrented](index.html) module"]
pub struct HCPERIODCURRENTED_SPEC;
impl crate::RegisterSpec for HCPERIODCURRENTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcperiodcurrented::R](R) reader structure"]
impl crate::Readable for HCPERIODCURRENTED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCPERIODCURRENTED to value 0"]
impl crate::Resettable for HCPERIODCURRENTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
