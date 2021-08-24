#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC Auto-test modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUTOTEST_A {
    #[doc = "0: No auto test, normal mode of operation"]
    NO_AUTOTEST = 0,
    #[doc = "1: Offset Error test (refer to ADC cell datasheet)"]
    OFFSET_ERROR = 1,
    #[doc = "2: Gain Error (high code) test (refer to ADC cell datasheet)"]
    GAIN_ERROR_HIGH = 2,
    #[doc = "3: Gain Error (low code) test (refer to ADC cell datasheet)"]
    GAIN_ERROR_LOW = 3,
}
impl From<AUTOTEST_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTOTEST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AUTOTEST` reader - ADC Auto-test modes"]
pub struct AUTOTEST_R(crate::FieldReader<u8, AUTOTEST_A>);
impl AUTOTEST_R {
    pub(crate) fn new(bits: u8) -> Self {
        AUTOTEST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOTEST_A {
        match self.bits {
            0 => AUTOTEST_A::NO_AUTOTEST,
            1 => AUTOTEST_A::OFFSET_ERROR,
            2 => AUTOTEST_A::GAIN_ERROR_HIGH,
            3 => AUTOTEST_A::GAIN_ERROR_LOW,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_AUTOTEST`"]
    #[inline(always)]
    pub fn is_no_autotest(&self) -> bool {
        **self == AUTOTEST_A::NO_AUTOTEST
    }
    #[doc = "Checks if the value of the field is `OFFSET_ERROR`"]
    #[inline(always)]
    pub fn is_offset_error(&self) -> bool {
        **self == AUTOTEST_A::OFFSET_ERROR
    }
    #[doc = "Checks if the value of the field is `GAIN_ERROR_HIGH`"]
    #[inline(always)]
    pub fn is_gain_error_high(&self) -> bool {
        **self == AUTOTEST_A::GAIN_ERROR_HIGH
    }
    #[doc = "Checks if the value of the field is `GAIN_ERROR_LOW`"]
    #[inline(always)]
    pub fn is_gain_error_low(&self) -> bool {
        **self == AUTOTEST_A::GAIN_ERROR_LOW
    }
}
impl core::ops::Deref for AUTOTEST_R {
    type Target = crate::FieldReader<u8, AUTOTEST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOTEST` writer - ADC Auto-test modes"]
pub struct AUTOTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOTEST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOTEST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No auto test, normal mode of operation"]
    #[inline(always)]
    pub fn no_autotest(self) -> &'a mut W {
        self.variant(AUTOTEST_A::NO_AUTOTEST)
    }
    #[doc = "Offset Error test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn offset_error(self) -> &'a mut W {
        self.variant(AUTOTEST_A::OFFSET_ERROR)
    }
    #[doc = "Gain Error (high code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn gain_error_high(self) -> &'a mut W {
        self.variant(AUTOTEST_A::GAIN_ERROR_HIGH)
    }
    #[doc = "Gain Error (low code) test (refer to ADC cell datasheet)"]
    #[inline(always)]
    pub fn gain_error_low(self) -> &'a mut W {
        self.variant(AUTOTEST_A::GAIN_ERROR_LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC Auto-test modes"]
    #[inline(always)]
    pub fn autotest(&self) -> AUTOTEST_R {
        AUTOTEST_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC Auto-test modes"]
    #[inline(always)]
    pub fn autotest(&mut self) -> AUTOTEST_W {
        AUTOTEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
