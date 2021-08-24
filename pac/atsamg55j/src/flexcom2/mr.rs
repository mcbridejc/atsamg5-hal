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
#[doc = "FLEXCOM Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMODE_A {
    #[doc = "0: No communication"]
    NO_COM = 0,
    #[doc = "1: All related USART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN,)All SPI/TWI related registers are not accessible and have no impact on IOs."]
    USART = 1,
    #[doc = "2: SPI operating mode is selected.All USART/TWI related registers are not accessible and have no impact on IOs."]
    SPI = 2,
    #[doc = "3: All related TWI protocols are selected (TWI, SMBUS). All USART/SPI related registers are not accessible and have no impact on IOs."]
    TWI = 3,
}
impl From<OPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMODE` reader - FLEXCOM Operating Mode"]
pub struct OPMODE_R(crate::FieldReader<u8, OPMODE_A>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPMODE_A {
        match self.bits {
            0 => OPMODE_A::NO_COM,
            1 => OPMODE_A::USART,
            2 => OPMODE_A::SPI,
            3 => OPMODE_A::TWI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_COM`"]
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        **self == OPMODE_A::NO_COM
    }
    #[doc = "Checks if the value of the field is `USART`"]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        **self == OPMODE_A::USART
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == OPMODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `TWI`"]
    #[inline(always)]
    pub fn is_twi(&self) -> bool {
        **self == OPMODE_A::TWI
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, OPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` writer - FLEXCOM Operating Mode"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No communication"]
    #[inline(always)]
    pub fn no_com(self) -> &'a mut W {
        self.variant(OPMODE_A::NO_COM)
    }
    #[doc = "All related USART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN,)All SPI/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut W {
        self.variant(OPMODE_A::USART)
    }
    #[doc = "SPI operating mode is selected.All USART/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(OPMODE_A::SPI)
    }
    #[doc = "All related TWI protocols are selected (TWI, SMBUS). All USART/SPI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn twi(self) -> &'a mut W {
        self.variant(OPMODE_A::TWI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FLEXCOM Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
