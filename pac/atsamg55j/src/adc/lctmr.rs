#[doc = "Register `LCTMR` reader"]
pub struct R(crate::R<LCTMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCTMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCTMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCTMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCTMR` writer"]
pub struct W(crate::W<LCTMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCTMR_SPEC>;
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
impl From<crate::W<LCTMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCTMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUALTRIG` reader - Dual Trigger ON"]
pub struct DUALTRIG_R(crate::FieldReader<bool, bool>);
impl DUALTRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUALTRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUALTRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUALTRIG` writer - Dual Trigger ON"]
pub struct DUALTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DUALTRIG_W<'a> {
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
#[doc = "Last Channel Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMOD_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPMOD` reader - Last Channel Comparison Mode"]
pub struct CMPMOD_R(crate::FieldReader<u8, CMPMOD_A>);
impl CMPMOD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMOD_A {
        match self.bits {
            0 => CMPMOD_A::LOW,
            1 => CMPMOD_A::HIGH,
            2 => CMPMOD_A::IN,
            3 => CMPMOD_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CMPMOD_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CMPMOD_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == CMPMOD_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == CMPMOD_A::OUT
    }
}
impl core::ops::Deref for CMPMOD_R {
    type Target = crate::FieldReader<u8, CMPMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMOD` writer - Last Channel Comparison Mode"]
pub struct CMPMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMOD_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMOD_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMOD_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMOD_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Dual Trigger ON"]
    #[inline(always)]
    pub fn dualtrig(&self) -> DUALTRIG_R {
        DUALTRIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Last Channel Comparison Mode"]
    #[inline(always)]
    pub fn cmpmod(&self) -> CMPMOD_R {
        CMPMOD_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dual Trigger ON"]
    #[inline(always)]
    pub fn dualtrig(&mut self) -> DUALTRIG_W {
        DUALTRIG_W { w: self }
    }
    #[doc = "Bits 4:5 - Last Channel Comparison Mode"]
    #[inline(always)]
    pub fn cmpmod(&mut self) -> CMPMOD_W {
        CMPMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Last Channel Trigger Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lctmr](index.html) module"]
pub struct LCTMR_SPEC;
impl crate::RegisterSpec for LCTMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lctmr::R](R) reader structure"]
impl crate::Readable for LCTMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lctmr::W](W) writer structure"]
impl crate::Writable for LCTMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LCTMR to value 0"]
impl crate::Resettable for LCTMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
