#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transfer Size\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSIZE_A {
    #[doc = "0: The buffer size is defined in byte."]
    T_8BIT = 0,
    #[doc = "1: The buffer size is defined in half-word (16-bit)."]
    T_16BIT = 1,
    #[doc = "2: The buffer size is defined in word (32-bit). Default value."]
    T_32BIT = 2,
}
impl From<TSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: TSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSIZE` reader - Transfer Size"]
pub struct TSIZE_R(crate::FieldReader<u8, TSIZE_A>);
impl TSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSIZE_A> {
        match self.bits {
            0 => Some(TSIZE_A::T_8BIT),
            1 => Some(TSIZE_A::T_16BIT),
            2 => Some(TSIZE_A::T_32BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `T_8BIT`"]
    #[inline(always)]
    pub fn is_t_8bit(&self) -> bool {
        **self == TSIZE_A::T_8BIT
    }
    #[doc = "Checks if the value of the field is `T_16BIT`"]
    #[inline(always)]
    pub fn is_t_16bit(&self) -> bool {
        **self == TSIZE_A::T_16BIT
    }
    #[doc = "Checks if the value of the field is `T_32BIT`"]
    #[inline(always)]
    pub fn is_t_32bit(&self) -> bool {
        **self == TSIZE_A::T_32BIT
    }
}
impl core::ops::Deref for TSIZE_R {
    type Target = crate::FieldReader<u8, TSIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIZE` writer - Transfer Size"]
pub struct TSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The buffer size is defined in byte."]
    #[inline(always)]
    pub fn t_8bit(self) -> &'a mut W {
        self.variant(TSIZE_A::T_8BIT)
    }
    #[doc = "The buffer size is defined in half-word (16-bit)."]
    #[inline(always)]
    pub fn t_16bit(self) -> &'a mut W {
        self.variant(TSIZE_A::T_16BIT)
    }
    #[doc = "The buffer size is defined in word (32-bit). Default value."]
    #[inline(always)]
    pub fn t_32bit(self) -> &'a mut W {
        self.variant(TSIZE_A::T_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Transfer Size"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transfer Size"]
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W {
        TSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory to Memory Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0x02"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
