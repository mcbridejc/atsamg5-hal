#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl From<crate::W<EMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMODE_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub struct CMPMODE_R(crate::FieldReader<u8, CMPMODE_A>);
impl CMPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            0 => CMPMODE_A::LOW,
            1 => CMPMODE_A::HIGH,
            2 => CMPMODE_A::IN,
            3 => CMPMODE_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CMPMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CMPMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == CMPMODE_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == CMPMODE_A::OUT
    }
}
impl core::ops::Deref for CMPMODE_R {
    type Target = crate::FieldReader<u8, CMPMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODE_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODE_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODE_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODE_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Comparison Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMPTYPE_A {
    #[doc = "0: Any conversion is performed and comparison function drives the COMPE flag."]
    FLAG_ONLY = 0,
    #[doc = "1: Comparison conditions must be met to start the storage of all conversions until the CMPRST bit is set."]
    START_CONDITION = 1,
}
impl From<CMPTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPTYPE` reader - Comparison Type"]
pub struct CMPTYPE_R(crate::FieldReader<bool, CMPTYPE_A>);
impl CMPTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPTYPE_A {
        match self.bits {
            false => CMPTYPE_A::FLAG_ONLY,
            true => CMPTYPE_A::START_CONDITION,
        }
    }
    #[doc = "Checks if the value of the field is `FLAG_ONLY`"]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        **self == CMPTYPE_A::FLAG_ONLY
    }
    #[doc = "Checks if the value of the field is `START_CONDITION`"]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        **self == CMPTYPE_A::START_CONDITION
    }
}
impl core::ops::Deref for CMPTYPE_R {
    type Target = crate::FieldReader<bool, CMPTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPTYPE` writer - Comparison Type"]
pub struct CMPTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Any conversion is performed and comparison function drives the COMPE flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut W {
        self.variant(CMPTYPE_A::FLAG_ONLY)
    }
    #[doc = "Comparison conditions must be met to start the storage of all conversions until the CMPRST bit is set."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut W {
        self.variant(CMPTYPE_A::START_CONDITION)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CMPSEL` reader - Comparison Selected Channel"]
pub struct CMPSEL_R(crate::FieldReader<u8, u8>);
impl CMPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPSEL` writer - Comparison Selected Channel"]
pub struct CMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CMPALL` reader - Compare All Channels"]
pub struct CMPALL_R(crate::FieldReader<bool, bool>);
impl CMPALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMPALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPALL` writer - Compare All Channels"]
pub struct CMPALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CMPFILTER` reader - Compare Event Filtering"]
pub struct CMPFILTER_R(crate::FieldReader<u8, u8>);
impl CMPFILTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMPFILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPFILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMPFILTER` writer - Compare Event Filtering"]
pub struct CMPFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Over Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: No averaging. ADC sample rate is maximum."]
    NO_AVERAGE = 0,
    #[doc = "1: 1-bit enhanced resolution by averaging. ADC sample rate divided by 4."]
    OSR4 = 1,
    #[doc = "2: 2-bit enhanced resolution by averaging. ADC sample rate divided by 16."]
    OSR16 = 2,
    #[doc = "3: 3-bit enhanced resolution by averaging. ADC sample rate divided by 64."]
    OSR64 = 3,
    #[doc = "4: 4-bit enhanced resolution by averaging. ADC sample rate divided by 256."]
    OSR256 = 4,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OSR` reader - Over Sampling Rate"]
pub struct OSR_R(crate::FieldReader<u8, OSR_A>);
impl OSR_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSR_A> {
        match self.bits {
            0 => Some(OSR_A::NO_AVERAGE),
            1 => Some(OSR_A::OSR4),
            2 => Some(OSR_A::OSR16),
            3 => Some(OSR_A::OSR64),
            4 => Some(OSR_A::OSR256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        **self == OSR_A::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        **self == OSR_A::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        **self == OSR_A::OSR16
    }
    #[doc = "Checks if the value of the field is `OSR64`"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        **self == OSR_A::OSR64
    }
    #[doc = "Checks if the value of the field is `OSR256`"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        **self == OSR_A::OSR256
    }
}
impl core::ops::Deref for OSR_R {
    type Target = crate::FieldReader<u8, OSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSR` writer - Over Sampling Rate"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No averaging. ADC sample rate is maximum."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(OSR_A::NO_AVERAGE)
    }
    #[doc = "1-bit enhanced resolution by averaging. ADC sample rate divided by 4."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut W {
        self.variant(OSR_A::OSR4)
    }
    #[doc = "2-bit enhanced resolution by averaging. ADC sample rate divided by 16."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut W {
        self.variant(OSR_A::OSR16)
    }
    #[doc = "3-bit enhanced resolution by averaging. ADC sample rate divided by 64."]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut W {
        self.variant(OSR_A::OSR64)
    }
    #[doc = "4-bit enhanced resolution by averaging. ADC sample rate divided by 256."]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut W {
        self.variant(OSR_A::OSR256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Averaging on Single Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASTE_A {
    #[doc = "0: The average requests several trigger events."]
    MULTI_TRIG_AVERAGE = 0,
    #[doc = "1: The average requests only one trigger event."]
    SINGLE_TRIG_AVERAGE = 1,
}
impl From<ASTE_A> for bool {
    #[inline(always)]
    fn from(variant: ASTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASTE` reader - Averaging on Single Trigger Event"]
pub struct ASTE_R(crate::FieldReader<bool, ASTE_A>);
impl ASTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ASTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASTE_A {
        match self.bits {
            false => ASTE_A::MULTI_TRIG_AVERAGE,
            true => ASTE_A::SINGLE_TRIG_AVERAGE,
        }
    }
    #[doc = "Checks if the value of the field is `MULTI_TRIG_AVERAGE`"]
    #[inline(always)]
    pub fn is_multi_trig_average(&self) -> bool {
        **self == ASTE_A::MULTI_TRIG_AVERAGE
    }
    #[doc = "Checks if the value of the field is `SINGLE_TRIG_AVERAGE`"]
    #[inline(always)]
    pub fn is_single_trig_average(&self) -> bool {
        **self == ASTE_A::SINGLE_TRIG_AVERAGE
    }
}
impl core::ops::Deref for ASTE_R {
    type Target = crate::FieldReader<bool, ASTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASTE` writer - Averaging on Single Trigger Event"]
pub struct ASTE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The average requests several trigger events."]
    #[inline(always)]
    pub fn multi_trig_average(self) -> &'a mut W {
        self.variant(ASTE_A::MULTI_TRIG_AVERAGE)
    }
    #[doc = "The average requests only one trigger event."]
    #[inline(always)]
    pub fn single_trig_average(self) -> &'a mut W {
        self.variant(ASTE_A::SINGLE_TRIG_AVERAGE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "External Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCCLK_A {
    #[doc = "0: The peripheral clock is the source for the ADC prescaler."]
    PERIPH_CLK = 0,
    #[doc = "1: PMC PCKx is the source clock for the ADC prescaler, thus the ADC clock can be independent of the core/peripheral clock."]
    PMC_PCK = 1,
}
impl From<SRCCLK_A> for bool {
    #[inline(always)]
    fn from(variant: SRCCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCCLK` reader - External Clock Selection"]
pub struct SRCCLK_R(crate::FieldReader<bool, SRCCLK_A>);
impl SRCCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRCCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCCLK_A {
        match self.bits {
            false => SRCCLK_A::PERIPH_CLK,
            true => SRCCLK_A::PMC_PCK,
        }
    }
    #[doc = "Checks if the value of the field is `PERIPH_CLK`"]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        **self == SRCCLK_A::PERIPH_CLK
    }
    #[doc = "Checks if the value of the field is `PMC_PCK`"]
    #[inline(always)]
    pub fn is_pmc_pck(&self) -> bool {
        **self == SRCCLK_A::PMC_PCK
    }
}
impl core::ops::Deref for SRCCLK_R {
    type Target = crate::FieldReader<bool, SRCCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCCLK` writer - External Clock Selection"]
pub struct SRCCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The peripheral clock is the source for the ADC prescaler."]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut W {
        self.variant(SRCCLK_A::PERIPH_CLK)
    }
    #[doc = "PMC PCKx is the source clock for the ADC prescaler, thus the ADC clock can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn pmc_pck(self) -> &'a mut W {
        self.variant(SRCCLK_A::PMC_PCK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `TAG` reader - Tag of the ADC_LCDR"]
pub struct TAG_R(crate::FieldReader<bool, bool>);
impl TAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAG` writer - Tag of the ADC_LCDR"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Comparison Type"]
    #[inline(always)]
    pub fn cmptype(&self) -> CMPTYPE_R {
        CMPTYPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Over Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    pub fn aste(&self) -> ASTE_R {
        ASTE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - External Clock Selection"]
    #[inline(always)]
    pub fn srcclk(&self) -> SRCCLK_R {
        SRCCLK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Tag of the ADC_LCDR"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bit 2 - Comparison Type"]
    #[inline(always)]
    pub fn cmptype(&mut self) -> CMPTYPE_W {
        CMPTYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W {
        CMPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CMPALL_W {
        CMPALL_W { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W {
        CMPFILTER_W { w: self }
    }
    #[doc = "Bits 16:18 - Over Sampling Rate"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    pub fn aste(&mut self) -> ASTE_W {
        ASTE_W { w: self }
    }
    #[doc = "Bit 21 - External Clock Selection"]
    #[inline(always)]
    pub fn srcclk(&mut self) -> SRCCLK_W {
        SRCCLK_W { w: self }
    }
    #[doc = "Bit 24 - Tag of the ADC_LCDR"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
