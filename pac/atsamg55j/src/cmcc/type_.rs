#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AP` reader - Access Port Access Allowed"]
pub struct AP_R(crate::FieldReader<bool, bool>);
impl AP_R {
    pub(crate) fn new(bits: bool) -> Self {
        AP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLK` reader - Dynamic Clock Gating Supported"]
pub struct GCLK_R(crate::FieldReader<bool, bool>);
impl GCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANDP` reader - Random Selection Policy Supported"]
pub struct RANDP_R(crate::FieldReader<bool, bool>);
impl RANDP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANDP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANDP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRUP` reader - Least Recently Used Policy Supported"]
pub struct LRUP_R(crate::FieldReader<bool, bool>);
impl LRUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LRUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRP` reader - Random Selection Policy Supported"]
pub struct RRP_R(crate::FieldReader<bool, bool>);
impl RRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of Ways\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAYNUM_A {
    #[doc = "0: Direct Mapped Cache"]
    DMAPPED = 0,
    #[doc = "1: 2-way set associative"]
    ARCH2WAY = 1,
    #[doc = "2: 4-way set associative"]
    ARCH4WAY = 2,
    #[doc = "3: 8-way set associative"]
    ARCH8WAY = 3,
}
impl From<WAYNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: WAYNUM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAYNUM` reader - Number of Ways"]
pub struct WAYNUM_R(crate::FieldReader<u8, WAYNUM_A>);
impl WAYNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAYNUM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAYNUM_A {
        match self.bits {
            0 => WAYNUM_A::DMAPPED,
            1 => WAYNUM_A::ARCH2WAY,
            2 => WAYNUM_A::ARCH4WAY,
            3 => WAYNUM_A::ARCH8WAY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        **self == WAYNUM_A::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        **self == WAYNUM_A::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        **self == WAYNUM_A::ARCH4WAY
    }
    #[doc = "Checks if the value of the field is `ARCH8WAY`"]
    #[inline(always)]
    pub fn is_arch8way(&self) -> bool {
        **self == WAYNUM_A::ARCH8WAY
    }
}
impl core::ops::Deref for WAYNUM_R {
    type Target = crate::FieldReader<u8, WAYNUM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCKDOWN` reader - Lockdown Supported"]
pub struct LCKDOWN_R(crate::FieldReader<bool, bool>);
impl LCKDOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCKDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCKDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Data Cache Size\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSIZE_A {
    #[doc = "0: Data cache size is 1 Kbyte"]
    CSIZE_1KB = 0,
    #[doc = "1: Data cache size is 2 Kbytes"]
    CSIZE_2KB = 1,
    #[doc = "2: Data cache size is 4 Kbytes"]
    CSIZE_4KB = 2,
    #[doc = "3: Data cache size is 8 Kbytes"]
    CSIZE_8KB = 3,
}
impl From<CSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSIZE` reader - Data Cache Size"]
pub struct CSIZE_R(crate::FieldReader<u8, CSIZE_A>);
impl CSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZE_A> {
        match self.bits {
            0 => Some(CSIZE_A::CSIZE_1KB),
            1 => Some(CSIZE_A::CSIZE_2KB),
            2 => Some(CSIZE_A::CSIZE_4KB),
            3 => Some(CSIZE_A::CSIZE_8KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        **self == CSIZE_A::CSIZE_8KB
    }
}
impl core::ops::Deref for CSIZE_R {
    type Target = crate::FieldReader<u8, CSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Cache LIne Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLSIZE_A {
    #[doc = "0: Cache line size is 4 bytes"]
    CLSIZE_1KB = 0,
    #[doc = "1: Cache line size is 8 bytes"]
    CLSIZE_2KB = 1,
    #[doc = "2: Cache line size is 16 bytes"]
    CLSIZE_4KB = 2,
    #[doc = "3: Cache line size is 32 bytes"]
    CLSIZE_8KB = 3,
}
impl From<CLSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLSIZE` reader - Cache LIne Size"]
pub struct CLSIZE_R(crate::FieldReader<u8, CLSIZE_A>);
impl CLSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLSIZE_A> {
        match self.bits {
            0 => Some(CLSIZE_A::CLSIZE_1KB),
            1 => Some(CLSIZE_A::CLSIZE_2KB),
            2 => Some(CLSIZE_A::CLSIZE_4KB),
            3 => Some(CLSIZE_A::CLSIZE_8KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_1KB`"]
    #[inline(always)]
    pub fn is_clsize_1kb(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_2KB`"]
    #[inline(always)]
    pub fn is_clsize_2kb(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4KB`"]
    #[inline(always)]
    pub fn is_clsize_4kb(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8KB`"]
    #[inline(always)]
    pub fn is_clsize_8kb(&self) -> bool {
        **self == CLSIZE_A::CLSIZE_8KB
    }
}
impl core::ops::Deref for CLSIZE_R {
    type Target = crate::FieldReader<u8, CLSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Access Port Access Allowed"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dynamic Clock Gating Supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GCLK_R {
        GCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn randp(&self) -> RANDP_R {
        RANDP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Least Recently Used Policy Supported"]
    #[inline(always)]
    pub fn lrup(&self) -> LRUP_R {
        LRUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RRP_R {
        RRP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Number of Ways"]
    #[inline(always)]
    pub fn waynum(&self) -> WAYNUM_R {
        WAYNUM_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Lockdown Supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LCKDOWN_R {
        LCKDOWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Data Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:13 - Cache LIne Size"]
    #[inline(always)]
    pub fn clsize(&self) -> CLSIZE_R {
        CLSIZE_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
#[doc = "Cache Controller Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TYPE to value 0x13d7"]
impl crate::Resettable for TYPE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x13d7
    }
}
