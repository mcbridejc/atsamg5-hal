#[doc = "Register `MODR` reader"]
pub struct R(crate::R<MODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODR` writer"]
pub struct W(crate::W<MODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODR_SPEC>;
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
impl From<crate::W<MODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selection of the 32-bit Counter Modulo to generate RTTINC2 flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELINC2_A {
    #[doc = "0: The RTTINC2 flag never rises"]
    NO_RTTINC2 = 0,
    #[doc = "1: The RTTINC2 flag is set when CRTV modulo 64 equals 0"]
    MOD64 = 1,
    #[doc = "2: The RTTINC2 flag is set when CRTV modulo 128 equals 0"]
    MOD128 = 2,
    #[doc = "3: The RTTINC2 flag is set when CRTV modulo 256 equals 0"]
    MOD256 = 3,
    #[doc = "4: The RTTINC2 flag is set when CRTV modulo 512 equals 0"]
    MOD512 = 4,
    #[doc = "5: The RTTINC2 flag is set when CRTV modulo 1024 equals 0.Example: If RTPRES=32 then RTTINC2 flag rises once per second if the slow clock is 32.768 kHz."]
    MOD1024 = 5,
    #[doc = "6: The RTTINC2 flag is set when CRTV modulo 2048 equals 0"]
    MOD2048 = 6,
    #[doc = "7: The RTTINC2 flag is set when CRTV modulo 4096 equals 0"]
    MOD4096 = 7,
}
impl From<SELINC2_A> for u8 {
    #[inline(always)]
    fn from(variant: SELINC2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELINC2` reader - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
pub struct SELINC2_R(crate::FieldReader<u8, SELINC2_A>);
impl SELINC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELINC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELINC2_A {
        match self.bits {
            0 => SELINC2_A::NO_RTTINC2,
            1 => SELINC2_A::MOD64,
            2 => SELINC2_A::MOD128,
            3 => SELINC2_A::MOD256,
            4 => SELINC2_A::MOD512,
            5 => SELINC2_A::MOD1024,
            6 => SELINC2_A::MOD2048,
            7 => SELINC2_A::MOD4096,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_RTTINC2`"]
    #[inline(always)]
    pub fn is_no_rttinc2(&self) -> bool {
        **self == SELINC2_A::NO_RTTINC2
    }
    #[doc = "Checks if the value of the field is `MOD64`"]
    #[inline(always)]
    pub fn is_mod64(&self) -> bool {
        **self == SELINC2_A::MOD64
    }
    #[doc = "Checks if the value of the field is `MOD128`"]
    #[inline(always)]
    pub fn is_mod128(&self) -> bool {
        **self == SELINC2_A::MOD128
    }
    #[doc = "Checks if the value of the field is `MOD256`"]
    #[inline(always)]
    pub fn is_mod256(&self) -> bool {
        **self == SELINC2_A::MOD256
    }
    #[doc = "Checks if the value of the field is `MOD512`"]
    #[inline(always)]
    pub fn is_mod512(&self) -> bool {
        **self == SELINC2_A::MOD512
    }
    #[doc = "Checks if the value of the field is `MOD1024`"]
    #[inline(always)]
    pub fn is_mod1024(&self) -> bool {
        **self == SELINC2_A::MOD1024
    }
    #[doc = "Checks if the value of the field is `MOD2048`"]
    #[inline(always)]
    pub fn is_mod2048(&self) -> bool {
        **self == SELINC2_A::MOD2048
    }
    #[doc = "Checks if the value of the field is `MOD4096`"]
    #[inline(always)]
    pub fn is_mod4096(&self) -> bool {
        **self == SELINC2_A::MOD4096
    }
}
impl core::ops::Deref for SELINC2_R {
    type Target = crate::FieldReader<u8, SELINC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELINC2` writer - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
pub struct SELINC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SELINC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELINC2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The RTTINC2 flag never rises"]
    #[inline(always)]
    pub fn no_rttinc2(self) -> &'a mut W {
        self.variant(SELINC2_A::NO_RTTINC2)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn mod64(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD64)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn mod128(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD128)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 256 equals 0"]
    #[inline(always)]
    pub fn mod256(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD256)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 512 equals 0"]
    #[inline(always)]
    pub fn mod512(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD512)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 1024 equals 0.Example: If RTPRES=32 then RTTINC2 flag rises once per second if the slow clock is 32.768 kHz."]
    #[inline(always)]
    pub fn mod1024(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD1024)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 2048 equals 0"]
    #[inline(always)]
    pub fn mod2048(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD2048)
    }
    #[doc = "The RTTINC2 flag is set when CRTV modulo 4096 equals 0"]
    #[inline(always)]
    pub fn mod4096(self) -> &'a mut W {
        self.variant(SELINC2_A::MOD4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Selection of the 32-bit Counter Modulo to generate the trigger event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SELTRGEV_A {
    #[doc = "0: No event generated"]
    NO_EVENT = 0,
    #[doc = "1: Event occurs when CRTV modulo 2 equals 0"]
    MOD2 = 1,
    #[doc = "2: Event occurs when CRTV modulo 4 equals 0"]
    MOD4 = 2,
    #[doc = "3: Event occurs when CRTV modulo 8 equals 0"]
    MOD8 = 3,
    #[doc = "4: Event occurs when CRTV modulo 16 equals 0"]
    MOD16 = 4,
    #[doc = "5: Event occurs when CRTV modulo 32 equals 0"]
    MOD32 = 5,
    #[doc = "6: Event occurs when CRTV modulo 64 equals 0"]
    MOD64 = 6,
    #[doc = "7: Event occurs when CRTV modulo 128 equals 0"]
    MOD128 = 7,
}
impl From<SELTRGEV_A> for u8 {
    #[inline(always)]
    fn from(variant: SELTRGEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SELTRGEV` reader - Selection of the 32-bit Counter Modulo to generate the trigger event"]
pub struct SELTRGEV_R(crate::FieldReader<u8, SELTRGEV_A>);
impl SELTRGEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELTRGEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELTRGEV_A {
        match self.bits {
            0 => SELTRGEV_A::NO_EVENT,
            1 => SELTRGEV_A::MOD2,
            2 => SELTRGEV_A::MOD4,
            3 => SELTRGEV_A::MOD8,
            4 => SELTRGEV_A::MOD16,
            5 => SELTRGEV_A::MOD32,
            6 => SELTRGEV_A::MOD64,
            7 => SELTRGEV_A::MOD128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        **self == SELTRGEV_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `MOD2`"]
    #[inline(always)]
    pub fn is_mod2(&self) -> bool {
        **self == SELTRGEV_A::MOD2
    }
    #[doc = "Checks if the value of the field is `MOD4`"]
    #[inline(always)]
    pub fn is_mod4(&self) -> bool {
        **self == SELTRGEV_A::MOD4
    }
    #[doc = "Checks if the value of the field is `MOD8`"]
    #[inline(always)]
    pub fn is_mod8(&self) -> bool {
        **self == SELTRGEV_A::MOD8
    }
    #[doc = "Checks if the value of the field is `MOD16`"]
    #[inline(always)]
    pub fn is_mod16(&self) -> bool {
        **self == SELTRGEV_A::MOD16
    }
    #[doc = "Checks if the value of the field is `MOD32`"]
    #[inline(always)]
    pub fn is_mod32(&self) -> bool {
        **self == SELTRGEV_A::MOD32
    }
    #[doc = "Checks if the value of the field is `MOD64`"]
    #[inline(always)]
    pub fn is_mod64(&self) -> bool {
        **self == SELTRGEV_A::MOD64
    }
    #[doc = "Checks if the value of the field is `MOD128`"]
    #[inline(always)]
    pub fn is_mod128(&self) -> bool {
        **self == SELTRGEV_A::MOD128
    }
}
impl core::ops::Deref for SELTRGEV_R {
    type Target = crate::FieldReader<u8, SELTRGEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELTRGEV` writer - Selection of the 32-bit Counter Modulo to generate the trigger event"]
pub struct SELTRGEV_W<'a> {
    w: &'a mut W,
}
impl<'a> SELTRGEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELTRGEV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No event generated"]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(SELTRGEV_A::NO_EVENT)
    }
    #[doc = "Event occurs when CRTV modulo 2 equals 0"]
    #[inline(always)]
    pub fn mod2(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD2)
    }
    #[doc = "Event occurs when CRTV modulo 4 equals 0"]
    #[inline(always)]
    pub fn mod4(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD4)
    }
    #[doc = "Event occurs when CRTV modulo 8 equals 0"]
    #[inline(always)]
    pub fn mod8(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD8)
    }
    #[doc = "Event occurs when CRTV modulo 16 equals 0"]
    #[inline(always)]
    pub fn mod16(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD16)
    }
    #[doc = "Event occurs when CRTV modulo 32 equals 0"]
    #[inline(always)]
    pub fn mod32(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD32)
    }
    #[doc = "Event occurs when CRTV modulo 64 equals 0"]
    #[inline(always)]
    pub fn mod64(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD64)
    }
    #[doc = "Event occurs when CRTV modulo 128 equals 0"]
    #[inline(always)]
    pub fn mod128(self) -> &'a mut W {
        self.variant(SELTRGEV_A::MOD128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
    #[inline(always)]
    pub fn selinc2(&self) -> SELINC2_R {
        SELINC2_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Selection of the 32-bit Counter Modulo to generate the trigger event"]
    #[inline(always)]
    pub fn seltrgev(&self) -> SELTRGEV_R {
        SELTRGEV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection of the 32-bit Counter Modulo to generate RTTINC2 flag"]
    #[inline(always)]
    pub fn selinc2(&mut self) -> SELINC2_W {
        SELINC2_W { w: self }
    }
    #[doc = "Bits 8:10 - Selection of the 32-bit Counter Modulo to generate the trigger event"]
    #[inline(always)]
    pub fn seltrgev(&mut self) -> SELTRGEV_W {
        SELTRGEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modulo Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modr](index.html) module"]
pub struct MODR_SPEC;
impl crate::RegisterSpec for MODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [modr::R](R) reader structure"]
impl crate::Readable for MODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [modr::W](W) writer structure"]
impl crate::Writable for MODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODR to value 0"]
impl crate::Resettable for MODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
