#[doc = "Register `BMR` reader"]
pub struct R(crate::R<BMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BMR` writer"]
pub struct W(crate::W<BMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMR_SPEC>;
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
impl From<crate::W<BMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Clock Signal 0 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC0XC0S_A {
    #[doc = "0: Signal connected to XC0: TCLK0"]
    TCLK0 = 0,
    #[doc = "2: Signal connected to XC0: TIOA1"]
    TIOA1 = 2,
    #[doc = "3: Signal connected to XC0: TIOA2"]
    TIOA2 = 3,
}
impl From<TC0XC0S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC0XC0S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC0XC0S` reader - External Clock Signal 0 Selection"]
pub struct TC0XC0S_R(crate::FieldReader<u8, TC0XC0S_A>);
impl TC0XC0S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC0XC0S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC0XC0S_A> {
        match self.bits {
            0 => Some(TC0XC0S_A::TCLK0),
            2 => Some(TC0XC0S_A::TIOA1),
            3 => Some(TC0XC0S_A::TIOA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK0`"]
    #[inline(always)]
    pub fn is_tclk0(&self) -> bool {
        **self == TC0XC0S_A::TCLK0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        **self == TC0XC0S_A::TIOA1
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        **self == TC0XC0S_A::TIOA2
    }
}
impl core::ops::Deref for TC0XC0S_R {
    type Target = crate::FieldReader<u8, TC0XC0S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC0XC0S` writer - External Clock Signal 0 Selection"]
pub struct TC0XC0S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC0XC0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC0XC0S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC0: TCLK0"]
    #[inline(always)]
    pub fn tclk0(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TCLK0)
    }
    #[doc = "Signal connected to XC0: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA1)
    }
    #[doc = "Signal connected to XC0: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC0XC0S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "External Clock Signal 1 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC1XC1S_A {
    #[doc = "0: Signal connected to XC1: TCLK1"]
    TCLK1 = 0,
    #[doc = "2: Signal connected to XC1: TIOA0"]
    TIOA0 = 2,
    #[doc = "3: Signal connected to XC1: TIOA2"]
    TIOA2 = 3,
}
impl From<TC1XC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC1XC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC1XC1S` reader - External Clock Signal 1 Selection"]
pub struct TC1XC1S_R(crate::FieldReader<u8, TC1XC1S_A>);
impl TC1XC1S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC1XC1S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC1XC1S_A> {
        match self.bits {
            0 => Some(TC1XC1S_A::TCLK1),
            2 => Some(TC1XC1S_A::TIOA0),
            3 => Some(TC1XC1S_A::TIOA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK1`"]
    #[inline(always)]
    pub fn is_tclk1(&self) -> bool {
        **self == TC1XC1S_A::TCLK1
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        **self == TC1XC1S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA2`"]
    #[inline(always)]
    pub fn is_tioa2(&self) -> bool {
        **self == TC1XC1S_A::TIOA2
    }
}
impl core::ops::Deref for TC1XC1S_R {
    type Target = crate::FieldReader<u8, TC1XC1S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC1XC1S` writer - External Clock Signal 1 Selection"]
pub struct TC1XC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC1XC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC1XC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC1: TCLK1"]
    #[inline(always)]
    pub fn tclk1(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TCLK1)
    }
    #[doc = "Signal connected to XC1: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA0)
    }
    #[doc = "Signal connected to XC1: TIOA2"]
    #[inline(always)]
    pub fn tioa2(self) -> &'a mut W {
        self.variant(TC1XC1S_A::TIOA2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "External Clock Signal 2 Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TC2XC2S_A {
    #[doc = "0: Signal connected to XC2: TCLK2"]
    TCLK2 = 0,
    #[doc = "2: Signal connected to XC2: TIOA0"]
    TIOA0 = 2,
    #[doc = "3: Signal connected to XC2: TIOA1"]
    TIOA1 = 3,
}
impl From<TC2XC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: TC2XC2S_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TC2XC2S` reader - External Clock Signal 2 Selection"]
pub struct TC2XC2S_R(crate::FieldReader<u8, TC2XC2S_A>);
impl TC2XC2S_R {
    pub(crate) fn new(bits: u8) -> Self {
        TC2XC2S_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TC2XC2S_A> {
        match self.bits {
            0 => Some(TC2XC2S_A::TCLK2),
            2 => Some(TC2XC2S_A::TIOA0),
            3 => Some(TC2XC2S_A::TIOA1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCLK2`"]
    #[inline(always)]
    pub fn is_tclk2(&self) -> bool {
        **self == TC2XC2S_A::TCLK2
    }
    #[doc = "Checks if the value of the field is `TIOA0`"]
    #[inline(always)]
    pub fn is_tioa0(&self) -> bool {
        **self == TC2XC2S_A::TIOA0
    }
    #[doc = "Checks if the value of the field is `TIOA1`"]
    #[inline(always)]
    pub fn is_tioa1(&self) -> bool {
        **self == TC2XC2S_A::TIOA1
    }
}
impl core::ops::Deref for TC2XC2S_R {
    type Target = crate::FieldReader<u8, TC2XC2S_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC2XC2S` writer - External Clock Signal 2 Selection"]
pub struct TC2XC2S_W<'a> {
    w: &'a mut W,
}
impl<'a> TC2XC2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TC2XC2S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Signal connected to XC2: TCLK2"]
    #[inline(always)]
    pub fn tclk2(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TCLK2)
    }
    #[doc = "Signal connected to XC2: TIOA0"]
    #[inline(always)]
    pub fn tioa0(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA0)
    }
    #[doc = "Signal connected to XC2: TIOA1"]
    #[inline(always)]
    pub fn tioa1(self) -> &'a mut W {
        self.variant(TC2XC2S_A::TIOA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&self) -> TC0XC0S_R {
        TC0XC0S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&self) -> TC1XC1S_R {
        TC1XC1S_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&self) -> TC2XC2S_R {
        TC2XC2S_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Clock Signal 0 Selection"]
    #[inline(always)]
    pub fn tc0xc0s(&mut self) -> TC0XC0S_W {
        TC0XC0S_W { w: self }
    }
    #[doc = "Bits 2:3 - External Clock Signal 1 Selection"]
    #[inline(always)]
    pub fn tc1xc1s(&mut self) -> TC1XC1S_W {
        TC1XC1S_W { w: self }
    }
    #[doc = "Bits 4:5 - External Clock Signal 2 Selection"]
    #[inline(always)]
    pub fn tc2xc2s(&mut self) -> TC2XC2S_W {
        TC2XC2S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmr](index.html) module"]
pub struct BMR_SPEC;
impl crate::RegisterSpec for BMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmr::R](R) reader structure"]
impl crate::Readable for BMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmr::W](W) writer structure"]
impl crate::Writable for BMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BMR to value 0"]
impl crate::Resettable for BMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
