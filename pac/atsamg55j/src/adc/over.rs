#[doc = "Register `OVER` reader"]
pub struct R(crate::R<OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVRE0` reader - Overrun Error 0"]
pub struct OVRE0_R(crate::FieldReader<bool, bool>);
impl OVRE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE1` reader - Overrun Error 1"]
pub struct OVRE1_R(crate::FieldReader<bool, bool>);
impl OVRE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE2` reader - Overrun Error 2"]
pub struct OVRE2_R(crate::FieldReader<bool, bool>);
impl OVRE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE3` reader - Overrun Error 3"]
pub struct OVRE3_R(crate::FieldReader<bool, bool>);
impl OVRE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE4` reader - Overrun Error 4"]
pub struct OVRE4_R(crate::FieldReader<bool, bool>);
impl OVRE4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE5` reader - Overrun Error 5"]
pub struct OVRE5_R(crate::FieldReader<bool, bool>);
impl OVRE5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE6` reader - Overrun Error 6"]
pub struct OVRE6_R(crate::FieldReader<bool, bool>);
impl OVRE6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE7` reader - Overrun Error 7"]
pub struct OVRE7_R(crate::FieldReader<bool, bool>);
impl OVRE7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Overrun Error 0"]
    #[inline(always)]
    pub fn ovre0(&self) -> OVRE0_R {
        OVRE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overrun Error 1"]
    #[inline(always)]
    pub fn ovre1(&self) -> OVRE1_R {
        OVRE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun Error 2"]
    #[inline(always)]
    pub fn ovre2(&self) -> OVRE2_R {
        OVRE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error 3"]
    #[inline(always)]
    pub fn ovre3(&self) -> OVRE3_R {
        OVRE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Overrun Error 4"]
    #[inline(always)]
    pub fn ovre4(&self) -> OVRE4_R {
        OVRE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error 5"]
    #[inline(always)]
    pub fn ovre5(&self) -> OVRE5_R {
        OVRE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Overrun Error 6"]
    #[inline(always)]
    pub fn ovre6(&self) -> OVRE6_R {
        OVRE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Overrun Error 7"]
    #[inline(always)]
    pub fn ovre7(&self) -> OVRE7_R {
        OVRE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
#[doc = "Overrun Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [over](index.html) module"]
pub struct OVER_SPEC;
impl crate::RegisterSpec for OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [over::R](R) reader structure"]
impl crate::Readable for OVER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OVER to value 0"]
impl crate::Resettable for OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
