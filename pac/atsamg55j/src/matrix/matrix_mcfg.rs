#[doc = "Register `MATRIX_MCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_MCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_MCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_MCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_MCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_MCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_MCFG_SPEC>;
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
impl From<crate::W<MATRIX_MCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_MCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Undefined Length Burst Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ULBT_A {
    #[doc = "0: No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    UNLIMITED = 0,
    #[doc = "1: The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    SINGLE = 1,
    #[doc = "2: The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    _4_BEAT = 2,
    #[doc = "3: The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    _8_BEAT = 3,
    #[doc = "4: The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    _16_BEAT = 4,
}
impl From<ULBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ULBT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub struct ULBT_R(crate::FieldReader<u8, ULBT_A>);
impl ULBT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ULBT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ULBT_A> {
        match self.bits {
            0 => Some(ULBT_A::UNLIMITED),
            1 => Some(ULBT_A::SINGLE),
            2 => Some(ULBT_A::_4_BEAT),
            3 => Some(ULBT_A::_8_BEAT),
            4 => Some(ULBT_A::_16_BEAT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLIMITED`"]
    #[inline(always)]
    pub fn is_unlimited(&self) -> bool {
        **self == ULBT_A::UNLIMITED
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == ULBT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `_4_BEAT`"]
    #[inline(always)]
    pub fn is_4_beat(&self) -> bool {
        **self == ULBT_A::_4_BEAT
    }
    #[doc = "Checks if the value of the field is `_8_BEAT`"]
    #[inline(always)]
    pub fn is_8_beat(&self) -> bool {
        **self == ULBT_A::_8_BEAT
    }
    #[doc = "Checks if the value of the field is `_16_BEAT`"]
    #[inline(always)]
    pub fn is_16_beat(&self) -> bool {
        **self == ULBT_A::_16_BEAT
    }
}
impl core::ops::Deref for ULBT_R {
    type Target = crate::FieldReader<u8, ULBT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub struct ULBT_W<'a> {
    w: &'a mut W,
}
impl<'a> ULBT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULBT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No predicted end of burst is generated and therefore INCR bursts coming from this master cannot be broken."]
    #[inline(always)]
    pub fn unlimited(self) -> &'a mut W {
        self.variant(ULBT_A::UNLIMITED)
    }
    #[doc = "The undefined length burst is treated as a succession of single access allowing rearbitration at each beat of the INCR burst."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(ULBT_A::SINGLE)
    }
    #[doc = "The undefined length burst is split into a 4-beat bursts allowing rearbitration at each 4-beat burst end."]
    #[inline(always)]
    pub fn _4_beat(self) -> &'a mut W {
        self.variant(ULBT_A::_4_BEAT)
    }
    #[doc = "The undefined length burst is split into 8-beat bursts allowing rearbitration at each 8-beat burst end."]
    #[inline(always)]
    pub fn _8_beat(self) -> &'a mut W {
        self.variant(ULBT_A::_8_BEAT)
    }
    #[doc = "The undefined length burst is split into 16-beat bursts allowing rearbitration at each 16-beat burst end."]
    #[inline(always)]
    pub fn _16_beat(self) -> &'a mut W {
        self.variant(ULBT_A::_16_BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> ULBT_W {
        ULBT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_mcfg](index.html) module"]
pub struct MATRIX_MCFG_SPEC;
impl crate::RegisterSpec for MATRIX_MCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_mcfg::R](R) reader structure"]
impl crate::Readable for MATRIX_MCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_mcfg::W](W) writer structure"]
impl crate::Writable for MATRIX_MCFG_SPEC {
    type Writer = W;
}
