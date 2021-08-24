#[doc = "Register `PWMR` reader"]
pub struct R(crate::R<PWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWMR` writer"]
pub struct W(crate::W<PWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWMR_SPEC>;
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
impl From<crate::W<PWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Low Power Value Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOWERS_A {
    #[doc = "0: The trimming value applied to the regulator when the device is in Wait mode. This value is factory-defined."]
    FACTORY = 0,
    #[doc = "1: The trimming value applied to the regulator is defined by the value programmed in the LPOWERx bits."]
    USER = 1,
}
impl From<LPOWERS_A> for bool {
    #[inline(always)]
    fn from(variant: LPOWERS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOWERS` reader - Low Power Value Selection"]
pub struct LPOWERS_R(crate::FieldReader<bool, LPOWERS_A>);
impl LPOWERS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPOWERS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOWERS_A {
        match self.bits {
            false => LPOWERS_A::FACTORY,
            true => LPOWERS_A::USER,
        }
    }
    #[doc = "Checks if the value of the field is `FACTORY`"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        **self == LPOWERS_A::FACTORY
    }
    #[doc = "Checks if the value of the field is `USER`"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        **self == LPOWERS_A::USER
    }
}
impl core::ops::Deref for LPOWERS_R {
    type Target = crate::FieldReader<bool, LPOWERS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOWERS` writer - Low Power Value Selection"]
pub struct LPOWERS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOWERS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOWERS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The trimming value applied to the regulator when the device is in Wait mode. This value is factory-defined."]
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(LPOWERS_A::FACTORY)
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in the LPOWERx bits."]
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(LPOWERS_A::USER)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `LPOWER0` reader - Low Power Value"]
pub struct LPOWER0_R(crate::FieldReader<bool, bool>);
impl LPOWER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPOWER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPOWER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOWER0` writer - Low Power Value"]
pub struct LPOWER0_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOWER0_W<'a> {
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
#[doc = "Field `LPOWER1` reader - Low Power Value"]
pub struct LPOWER1_R(crate::FieldReader<bool, bool>);
impl LPOWER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPOWER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPOWER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOWER1` writer - Low Power Value"]
pub struct LPOWER1_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOWER1_W<'a> {
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
#[doc = "Field `LPOWER2` reader - Low Power Value"]
pub struct LPOWER2_R(crate::FieldReader<bool, bool>);
impl LPOWER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPOWER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPOWER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOWER2` writer - Low Power Value"]
pub struct LPOWER2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOWER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `LPOWER3` reader - Low Power Value"]
pub struct LPOWER3_R(crate::FieldReader<bool, bool>);
impl LPOWER3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPOWER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPOWER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPOWER3` writer - Low Power Value"]
pub struct LPOWER3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOWER3_W<'a> {
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
#[doc = "Start-up Time when Resuming from Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STUPTIME_A {
    #[doc = "0: Fast start-up."]
    FAST = 0,
    #[doc = "1: Slow start-up."]
    SLOW = 1,
}
impl From<STUPTIME_A> for bool {
    #[inline(always)]
    fn from(variant: STUPTIME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STUPTIME` reader - Start-up Time when Resuming from Wait Mode"]
pub struct STUPTIME_R(crate::FieldReader<bool, STUPTIME_A>);
impl STUPTIME_R {
    pub(crate) fn new(bits: bool) -> Self {
        STUPTIME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STUPTIME_A {
        match self.bits {
            false => STUPTIME_A::FAST,
            true => STUPTIME_A::SLOW,
        }
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == STUPTIME_A::FAST
    }
    #[doc = "Checks if the value of the field is `SLOW`"]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        **self == STUPTIME_A::SLOW
    }
}
impl core::ops::Deref for STUPTIME_R {
    type Target = crate::FieldReader<bool, STUPTIME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STUPTIME` writer - Start-up Time when Resuming from Wait Mode"]
pub struct STUPTIME_W<'a> {
    w: &'a mut W,
}
impl<'a> STUPTIME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STUPTIME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fast start-up."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(STUPTIME_A::FAST)
    }
    #[doc = "Slow start-up."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut W {
        self.variant(STUPTIME_A::SLOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Enhanced Custom Power Value Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECPWRS_A {
    #[doc = "0: The trimming value applied to the regulator when the device is in Active mode. This value is factory-defined."]
    FACTORY = 0,
    #[doc = "1: The trimming value applied to the regulator is defined by the value programmed in ECPWRx bits."]
    USER = 1,
}
impl From<ECPWRS_A> for bool {
    #[inline(always)]
    fn from(variant: ECPWRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECPWRS` reader - Enhanced Custom Power Value Selection"]
pub struct ECPWRS_R(crate::FieldReader<bool, ECPWRS_A>);
impl ECPWRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECPWRS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECPWRS_A {
        match self.bits {
            false => ECPWRS_A::FACTORY,
            true => ECPWRS_A::USER,
        }
    }
    #[doc = "Checks if the value of the field is `FACTORY`"]
    #[inline(always)]
    pub fn is_factory(&self) -> bool {
        **self == ECPWRS_A::FACTORY
    }
    #[doc = "Checks if the value of the field is `USER`"]
    #[inline(always)]
    pub fn is_user(&self) -> bool {
        **self == ECPWRS_A::USER
    }
}
impl core::ops::Deref for ECPWRS_R {
    type Target = crate::FieldReader<bool, ECPWRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPWRS` writer - Enhanced Custom Power Value Selection"]
pub struct ECPWRS_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPWRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECPWRS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The trimming value applied to the regulator when the device is in Active mode. This value is factory-defined."]
    #[inline(always)]
    pub fn factory(self) -> &'a mut W {
        self.variant(ECPWRS_A::FACTORY)
    }
    #[doc = "The trimming value applied to the regulator is defined by the value programmed in ECPWRx bits."]
    #[inline(always)]
    pub fn user(self) -> &'a mut W {
        self.variant(ECPWRS_A::USER)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ECPWR0` reader - Enhanced Custom Power Value"]
pub struct ECPWR0_R(crate::FieldReader<bool, bool>);
impl ECPWR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECPWR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECPWR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPWR0` writer - Enhanced Custom Power Value"]
pub struct ECPWR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPWR0_W<'a> {
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
#[doc = "Field `ECPWR1` reader - Enhanced Custom Power Value"]
pub struct ECPWR1_R(crate::FieldReader<bool, bool>);
impl ECPWR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECPWR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECPWR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPWR1` writer - Enhanced Custom Power Value"]
pub struct ECPWR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPWR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ECPWR2` reader - Enhanced Custom Power Value"]
pub struct ECPWR2_R(crate::FieldReader<bool, bool>);
impl ECPWR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECPWR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECPWR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPWR2` writer - Enhanced Custom Power Value"]
pub struct ECPWR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPWR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ECPWR3` reader - Enhanced Custom Power Value"]
pub struct ECPWR3_R(crate::FieldReader<bool, bool>);
impl ECPWR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECPWR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECPWR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECPWR3` writer - Enhanced Custom Power Value"]
pub struct ECPWR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPWR3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM0ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM0ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM0ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM0ON` reader - SRAM Power Control"]
pub struct SRAM0ON_R(crate::FieldReader<bool, SRAM0ON_A>);
impl SRAM0ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM0ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM0ON_A {
        match self.bits {
            false => SRAM0ON_A::OFF,
            true => SRAM0ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM0ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM0ON_A::ON
    }
}
impl core::ops::Deref for SRAM0ON_R {
    type Target = crate::FieldReader<bool, SRAM0ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM0ON` writer - SRAM Power Control"]
pub struct SRAM0ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM0ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM0ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM0ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM0ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM1ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM1ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM1ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM1ON` reader - SRAM Power Control"]
pub struct SRAM1ON_R(crate::FieldReader<bool, SRAM1ON_A>);
impl SRAM1ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM1ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM1ON_A {
        match self.bits {
            false => SRAM1ON_A::OFF,
            true => SRAM1ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM1ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM1ON_A::ON
    }
}
impl core::ops::Deref for SRAM1ON_R {
    type Target = crate::FieldReader<bool, SRAM1ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM1ON` writer - SRAM Power Control"]
pub struct SRAM1ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM1ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM1ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM1ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM2ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM2ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM2ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM2ON` reader - SRAM Power Control"]
pub struct SRAM2ON_R(crate::FieldReader<bool, SRAM2ON_A>);
impl SRAM2ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM2ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM2ON_A {
        match self.bits {
            false => SRAM2ON_A::OFF,
            true => SRAM2ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM2ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM2ON_A::ON
    }
}
impl core::ops::Deref for SRAM2ON_R {
    type Target = crate::FieldReader<bool, SRAM2ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM2ON` writer - SRAM Power Control"]
pub struct SRAM2ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM2ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM2ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM2ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM3ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM3ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM3ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM3ON` reader - SRAM Power Control"]
pub struct SRAM3ON_R(crate::FieldReader<bool, SRAM3ON_A>);
impl SRAM3ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM3ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM3ON_A {
        match self.bits {
            false => SRAM3ON_A::OFF,
            true => SRAM3ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM3ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM3ON_A::ON
    }
}
impl core::ops::Deref for SRAM3ON_R {
    type Target = crate::FieldReader<bool, SRAM3ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM3ON` writer - SRAM Power Control"]
pub struct SRAM3ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM3ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM3ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM3ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM3ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM4ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM4ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM4ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM4ON` reader - SRAM Power Control"]
pub struct SRAM4ON_R(crate::FieldReader<bool, SRAM4ON_A>);
impl SRAM4ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM4ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM4ON_A {
        match self.bits {
            false => SRAM4ON_A::OFF,
            true => SRAM4ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM4ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM4ON_A::ON
    }
}
impl core::ops::Deref for SRAM4ON_R {
    type Target = crate::FieldReader<bool, SRAM4ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM4ON` writer - SRAM Power Control"]
pub struct SRAM4ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM4ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM4ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM4ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM4ON_A::ON)
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
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM5ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM5ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM5ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM5ON` reader - SRAM Power Control"]
pub struct SRAM5ON_R(crate::FieldReader<bool, SRAM5ON_A>);
impl SRAM5ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM5ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM5ON_A {
        match self.bits {
            false => SRAM5ON_A::OFF,
            true => SRAM5ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM5ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM5ON_A::ON
    }
}
impl core::ops::Deref for SRAM5ON_R {
    type Target = crate::FieldReader<bool, SRAM5ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM5ON` writer - SRAM Power Control"]
pub struct SRAM5ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM5ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM5ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM5ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM5ON_A::ON)
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
#[doc = "SRAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM6ON_A {
    #[doc = "0: SRAMx is not powered."]
    OFF = 0,
    #[doc = "1: SRAMx is powered."]
    ON = 1,
}
impl From<SRAM6ON_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM6ON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM6ON` reader - SRAM Power Control"]
pub struct SRAM6ON_R(crate::FieldReader<bool, SRAM6ON_A>);
impl SRAM6ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM6ON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM6ON_A {
        match self.bits {
            false => SRAM6ON_A::OFF,
            true => SRAM6ON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == SRAM6ON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == SRAM6ON_A::ON
    }
}
impl core::ops::Deref for SRAM6ON_R {
    type Target = crate::FieldReader<bool, SRAM6ON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM6ON` writer - SRAM Power Control"]
pub struct SRAM6ON_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM6ON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAM6ON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SRAMx is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SRAM6ON_A::OFF)
    }
    #[doc = "SRAMx is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(SRAM6ON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Dual-port RAM Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPRAMON_A {
    #[doc = "0: USB dual-port RAM is not powered."]
    OFF = 0,
    #[doc = "1: USB dual-port RAM is powered."]
    ON = 1,
}
impl From<DPRAMON_A> for bool {
    #[inline(always)]
    fn from(variant: DPRAMON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPRAMON` reader - Dual-port RAM Power Control"]
pub struct DPRAMON_R(crate::FieldReader<bool, DPRAMON_A>);
impl DPRAMON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPRAMON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPRAMON_A {
        match self.bits {
            false => DPRAMON_A::OFF,
            true => DPRAMON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DPRAMON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == DPRAMON_A::ON
    }
}
impl core::ops::Deref for DPRAMON_R {
    type Target = crate::FieldReader<bool, DPRAMON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPRAMON` writer - Dual-port RAM Power Control"]
pub struct DPRAMON_W<'a> {
    w: &'a mut W,
}
impl<'a> DPRAMON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPRAMON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB dual-port RAM is not powered."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DPRAMON_A::OFF)
    }
    #[doc = "USB dual-port RAM is powered."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(DPRAMON_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Password Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEY_A {
    #[doc = "90: Writing any other value in this field aborts the write operation.Always reads as 0."]
    PASSWD = 90,
}
impl From<KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: KEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` reader - Password Key"]
pub struct KEY_R(crate::FieldReader<u8, KEY_A>);
impl KEY_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEY_A> {
        match self.bits {
            90 => Some(KEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        **self == KEY_A::PASSWD
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u8, KEY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - Password Key"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(KEY_A::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Value Selection"]
    #[inline(always)]
    pub fn lpowers(&self) -> LPOWERS_R {
        LPOWERS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Low Power Value"]
    #[inline(always)]
    pub fn lpower0(&self) -> LPOWER0_R {
        LPOWER0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Power Value"]
    #[inline(always)]
    pub fn lpower1(&self) -> LPOWER1_R {
        LPOWER1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Low Power Value"]
    #[inline(always)]
    pub fn lpower2(&self) -> LPOWER2_R {
        LPOWER2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Power Value"]
    #[inline(always)]
    pub fn lpower3(&self) -> LPOWER3_R {
        LPOWER3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Start-up Time when Resuming from Wait Mode"]
    #[inline(always)]
    pub fn stuptime(&self) -> STUPTIME_R {
        STUPTIME_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enhanced Custom Power Value Selection"]
    #[inline(always)]
    pub fn ecpwrs(&self) -> ECPWRS_R {
        ECPWRS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr0(&self) -> ECPWR0_R {
        ECPWR0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr1(&self) -> ECPWR1_R {
        ECPWR1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr2(&self) -> ECPWR2_R {
        ECPWR2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr3(&self) -> ECPWR3_R {
        ECPWR3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram0on(&self) -> SRAM0ON_R {
        SRAM0ON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram1on(&self) -> SRAM1ON_R {
        SRAM1ON_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram2on(&self) -> SRAM2ON_R {
        SRAM2ON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram3on(&self) -> SRAM3ON_R {
        SRAM3ON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram4on(&self) -> SRAM4ON_R {
        SRAM4ON_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram5on(&self) -> SRAM5ON_R {
        SRAM5ON_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram6on(&self) -> SRAM6ON_R {
        SRAM6ON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Dual-port RAM Power Control"]
    #[inline(always)]
    pub fn dpramon(&self) -> DPRAMON_R {
        DPRAMON_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Value Selection"]
    #[inline(always)]
    pub fn lpowers(&mut self) -> LPOWERS_W {
        LPOWERS_W { w: self }
    }
    #[doc = "Bit 1 - Low Power Value"]
    #[inline(always)]
    pub fn lpower0(&mut self) -> LPOWER0_W {
        LPOWER0_W { w: self }
    }
    #[doc = "Bit 2 - Low Power Value"]
    #[inline(always)]
    pub fn lpower1(&mut self) -> LPOWER1_W {
        LPOWER1_W { w: self }
    }
    #[doc = "Bit 3 - Low Power Value"]
    #[inline(always)]
    pub fn lpower2(&mut self) -> LPOWER2_W {
        LPOWER2_W { w: self }
    }
    #[doc = "Bit 4 - Low Power Value"]
    #[inline(always)]
    pub fn lpower3(&mut self) -> LPOWER3_W {
        LPOWER3_W { w: self }
    }
    #[doc = "Bit 7 - Start-up Time when Resuming from Wait Mode"]
    #[inline(always)]
    pub fn stuptime(&mut self) -> STUPTIME_W {
        STUPTIME_W { w: self }
    }
    #[doc = "Bit 8 - Enhanced Custom Power Value Selection"]
    #[inline(always)]
    pub fn ecpwrs(&mut self) -> ECPWRS_W {
        ECPWRS_W { w: self }
    }
    #[doc = "Bit 9 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr0(&mut self) -> ECPWR0_W {
        ECPWR0_W { w: self }
    }
    #[doc = "Bit 10 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr1(&mut self) -> ECPWR1_W {
        ECPWR1_W { w: self }
    }
    #[doc = "Bit 11 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr2(&mut self) -> ECPWR2_W {
        ECPWR2_W { w: self }
    }
    #[doc = "Bit 12 - Enhanced Custom Power Value"]
    #[inline(always)]
    pub fn ecpwr3(&mut self) -> ECPWR3_W {
        ECPWR3_W { w: self }
    }
    #[doc = "Bit 16 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram0on(&mut self) -> SRAM0ON_W {
        SRAM0ON_W { w: self }
    }
    #[doc = "Bit 17 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram1on(&mut self) -> SRAM1ON_W {
        SRAM1ON_W { w: self }
    }
    #[doc = "Bit 18 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram2on(&mut self) -> SRAM2ON_W {
        SRAM2ON_W { w: self }
    }
    #[doc = "Bit 19 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram3on(&mut self) -> SRAM3ON_W {
        SRAM3ON_W { w: self }
    }
    #[doc = "Bit 20 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram4on(&mut self) -> SRAM4ON_W {
        SRAM4ON_W { w: self }
    }
    #[doc = "Bit 21 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram5on(&mut self) -> SRAM5ON_W {
        SRAM5ON_W { w: self }
    }
    #[doc = "Bit 22 - SRAM Power Control"]
    #[inline(always)]
    pub fn sram6on(&mut self) -> SRAM6ON_W {
        SRAM6ON_W { w: self }
    }
    #[doc = "Bit 23 - Dual-port RAM Power Control"]
    #[inline(always)]
    pub fn dpramon(&mut self) -> DPRAMON_W {
        DPRAMON_W { w: self }
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Power Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwmr](index.html) module"]
pub struct PWMR_SPEC;
impl crate::RegisterSpec for PWMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwmr::R](R) reader structure"]
impl crate::Readable for PWMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwmr::W](W) writer structure"]
impl crate::Writable for PWMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWMR to value 0"]
impl crate::Resettable for PWMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
