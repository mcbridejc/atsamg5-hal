#[doc = "Register `PMC_PCSR1` reader"]
pub struct R(crate::R<PMC_PCSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID47` reader - Peripheral Clock 47 Status"]
pub struct PID47_R(crate::FieldReader<bool, bool>);
impl PID47_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID48` reader - Peripheral Clock 48 Status"]
pub struct PID48_R(crate::FieldReader<bool, bool>);
impl PID48_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PID49` reader - Peripheral Clock 49 Status"]
pub struct PID49_R(crate::FieldReader<bool, bool>);
impl PID49_R {
    pub(crate) fn new(bits: bool) -> Self {
        PID49_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PID49_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - Peripheral Clock 47 Status"]
    #[inline(always)]
    pub fn pid47(&self) -> PID47_R {
        PID47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Peripheral Clock 48 Status"]
    #[inline(always)]
    pub fn pid48(&self) -> PID48_R {
        PID48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral Clock 49 Status"]
    #[inline(always)]
    pub fn pid49(&self) -> PID49_R {
        PID49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Peripheral Clock Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pcsr1](index.html) module"]
pub struct PMC_PCSR1_SPEC;
impl crate::RegisterSpec for PMC_PCSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pcsr1::R](R) reader structure"]
impl crate::Readable for PMC_PCSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_PCSR1 to value 0"]
impl crate::Resettable for PMC_PCSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
