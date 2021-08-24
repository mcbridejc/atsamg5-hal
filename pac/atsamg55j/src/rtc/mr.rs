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
#[doc = "Field `HRMOD` reader - 12-/24-hour Mode"]
pub struct HRMOD_R(crate::FieldReader<bool, bool>);
impl HRMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRMOD` writer - 12-/24-hour Mode"]
pub struct HRMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> HRMOD_W<'a> {
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
#[doc = "Field `PERSIAN` reader - PERSIAN Calendar"]
pub struct PERSIAN_R(crate::FieldReader<bool, bool>);
impl PERSIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERSIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERSIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERSIAN` writer - PERSIAN Calendar"]
pub struct PERSIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `NEGPPM` reader - NEGative PPM Correction"]
pub struct NEGPPM_R(crate::FieldReader<bool, bool>);
impl NEGPPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEGPPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEGPPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEGPPM` writer - NEGative PPM Correction"]
pub struct NEGPPM_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGPPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CORRECTION` reader - Slow Clock Correction"]
pub struct CORRECTION_R(crate::FieldReader<u8, u8>);
impl CORRECTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        CORRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORRECTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CORRECTION` writer - Slow Clock Correction"]
pub struct CORRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> CORRECTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `HIGHPPM` reader - HIGH PPM Correction"]
pub struct HIGHPPM_R(crate::FieldReader<bool, bool>);
impl HIGHPPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIGHPPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIGHPPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIGHPPM` writer - HIGH PPM Correction"]
pub struct HIGHPPM_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGHPPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "All ADC Channel Trigger Event Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT0_A {
    #[doc = "0: No waveform, stuck at '0'"]
    NO_WAVE = 0,
    #[doc = "1: 1 Hz square wave"]
    FREQ1HZ = 1,
    #[doc = "2: 32 Hz square wave"]
    FREQ32HZ = 2,
    #[doc = "3: 64 Hz square wave"]
    FREQ64HZ = 3,
    #[doc = "4: 512 Hz square wave"]
    FREQ512HZ = 4,
    #[doc = "6: Output is a copy of the alarm flag"]
    ALARM_FLAG = 6,
}
impl From<OUT0_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUT0` reader - All ADC Channel Trigger Event Source Selection"]
pub struct OUT0_R(crate::FieldReader<u8, OUT0_A>);
impl OUT0_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT0_A> {
        match self.bits {
            0 => Some(OUT0_A::NO_WAVE),
            1 => Some(OUT0_A::FREQ1HZ),
            2 => Some(OUT0_A::FREQ32HZ),
            3 => Some(OUT0_A::FREQ64HZ),
            4 => Some(OUT0_A::FREQ512HZ),
            6 => Some(OUT0_A::ALARM_FLAG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        **self == OUT0_A::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        **self == OUT0_A::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        **self == OUT0_A::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        **self == OUT0_A::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        **self == OUT0_A::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        **self == OUT0_A::ALARM_FLAG
    }
}
impl core::ops::Deref for OUT0_R {
    type Target = crate::FieldReader<u8, OUT0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT0` writer - All ADC Channel Trigger Event Source Selection"]
pub struct OUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT0_A::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT0_A::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT0_A::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT0_A::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT0_A::FREQ512HZ)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT0_A::ALARM_FLAG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "ADC Last Channel Trigger Event Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OUT1_A {
    #[doc = "0: No waveform, stuck at '0'"]
    NO_WAVE = 0,
    #[doc = "1: 1 Hz square wave"]
    FREQ1HZ = 1,
    #[doc = "2: 32 Hz square wave"]
    FREQ32HZ = 2,
    #[doc = "3: 64 Hz square wave"]
    FREQ64HZ = 3,
    #[doc = "4: 512 Hz square wave"]
    FREQ512HZ = 4,
    #[doc = "6: Output is a copy of the alarm flag"]
    ALARM_FLAG = 6,
}
impl From<OUT1_A> for u8 {
    #[inline(always)]
    fn from(variant: OUT1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OUT1` reader - ADC Last Channel Trigger Event Source Selection"]
pub struct OUT1_R(crate::FieldReader<u8, OUT1_A>);
impl OUT1_R {
    pub(crate) fn new(bits: u8) -> Self {
        OUT1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUT1_A> {
        match self.bits {
            0 => Some(OUT1_A::NO_WAVE),
            1 => Some(OUT1_A::FREQ1HZ),
            2 => Some(OUT1_A::FREQ32HZ),
            3 => Some(OUT1_A::FREQ64HZ),
            4 => Some(OUT1_A::FREQ512HZ),
            6 => Some(OUT1_A::ALARM_FLAG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_WAVE`"]
    #[inline(always)]
    pub fn is_no_wave(&self) -> bool {
        **self == OUT1_A::NO_WAVE
    }
    #[doc = "Checks if the value of the field is `FREQ1HZ`"]
    #[inline(always)]
    pub fn is_freq1hz(&self) -> bool {
        **self == OUT1_A::FREQ1HZ
    }
    #[doc = "Checks if the value of the field is `FREQ32HZ`"]
    #[inline(always)]
    pub fn is_freq32hz(&self) -> bool {
        **self == OUT1_A::FREQ32HZ
    }
    #[doc = "Checks if the value of the field is `FREQ64HZ`"]
    #[inline(always)]
    pub fn is_freq64hz(&self) -> bool {
        **self == OUT1_A::FREQ64HZ
    }
    #[doc = "Checks if the value of the field is `FREQ512HZ`"]
    #[inline(always)]
    pub fn is_freq512hz(&self) -> bool {
        **self == OUT1_A::FREQ512HZ
    }
    #[doc = "Checks if the value of the field is `ALARM_FLAG`"]
    #[inline(always)]
    pub fn is_alarm_flag(&self) -> bool {
        **self == OUT1_A::ALARM_FLAG
    }
}
impl core::ops::Deref for OUT1_R {
    type Target = crate::FieldReader<u8, OUT1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT1` writer - ADC Last Channel Trigger Event Source Selection"]
pub struct OUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUT1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No waveform, stuck at '0'"]
    #[inline(always)]
    pub fn no_wave(self) -> &'a mut W {
        self.variant(OUT1_A::NO_WAVE)
    }
    #[doc = "1 Hz square wave"]
    #[inline(always)]
    pub fn freq1hz(self) -> &'a mut W {
        self.variant(OUT1_A::FREQ1HZ)
    }
    #[doc = "32 Hz square wave"]
    #[inline(always)]
    pub fn freq32hz(self) -> &'a mut W {
        self.variant(OUT1_A::FREQ32HZ)
    }
    #[doc = "64 Hz square wave"]
    #[inline(always)]
    pub fn freq64hz(self) -> &'a mut W {
        self.variant(OUT1_A::FREQ64HZ)
    }
    #[doc = "512 Hz square wave"]
    #[inline(always)]
    pub fn freq512hz(self) -> &'a mut W {
        self.variant(OUT1_A::FREQ512HZ)
    }
    #[doc = "Output is a copy of the alarm flag"]
    #[inline(always)]
    pub fn alarm_flag(self) -> &'a mut W {
        self.variant(OUT1_A::ALARM_FLAG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&self) -> HRMOD_R {
        HRMOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&self) -> PERSIAN_R {
        PERSIAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&self) -> NEGPPM_R {
        NEGPPM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&self) -> CORRECTION_R {
        CORRECTION_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&self) -> HIGHPPM_R {
        HIGHPPM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - All ADC Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out0(&self) -> OUT0_R {
        OUT0_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - ADC Last Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out1(&self) -> OUT1_R {
        OUT1_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 12-/24-hour Mode"]
    #[inline(always)]
    pub fn hrmod(&mut self) -> HRMOD_W {
        HRMOD_W { w: self }
    }
    #[doc = "Bit 1 - PERSIAN Calendar"]
    #[inline(always)]
    pub fn persian(&mut self) -> PERSIAN_W {
        PERSIAN_W { w: self }
    }
    #[doc = "Bit 4 - NEGative PPM Correction"]
    #[inline(always)]
    pub fn negppm(&mut self) -> NEGPPM_W {
        NEGPPM_W { w: self }
    }
    #[doc = "Bits 8:14 - Slow Clock Correction"]
    #[inline(always)]
    pub fn correction(&mut self) -> CORRECTION_W {
        CORRECTION_W { w: self }
    }
    #[doc = "Bit 15 - HIGH PPM Correction"]
    #[inline(always)]
    pub fn highppm(&mut self) -> HIGHPPM_W {
        HIGHPPM_W { w: self }
    }
    #[doc = "Bits 16:18 - All ADC Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out0(&mut self) -> OUT0_W {
        OUT0_W { w: self }
    }
    #[doc = "Bits 20:22 - ADC Last Channel Trigger Event Source Selection"]
    #[inline(always)]
    pub fn out1(&mut self) -> OUT1_W {
        OUT1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
