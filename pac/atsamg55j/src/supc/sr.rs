#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "WKUP Wake-up Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPS_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP pins has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<WKUPS_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPS` reader - WKUP Wake-up Status (cleared on read)"]
pub struct WKUPS_R(crate::FieldReader<bool, WKUPS_A>);
impl WKUPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPS_A {
        match self.bits {
            false => WKUPS_A::NO,
            true => WKUPS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == WKUPS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == WKUPS_A::PRESENT
    }
}
impl core::ops::Deref for WKUPS_R {
    type Target = crate::FieldReader<bool, WKUPS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Brownout Detector Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTS_A {
    #[doc = "0: No core brownout rising edge event has been detected since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one brownout output rising edge event has been detected since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<BODRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BODRSTS` reader - Brownout Detector Reset Status (cleared on read)"]
pub struct BODRSTS_R(crate::FieldReader<bool, BODRSTS_A>);
impl BODRSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTS_A {
        match self.bits {
            false => BODRSTS_A::NO,
            true => BODRSTS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == BODRSTS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == BODRSTS_A::PRESENT
    }
}
impl core::ops::Deref for BODRSTS_R {
    type Target = crate::FieldReader<bool, BODRSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Supply Monitor Reset Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMRSTS_A {
    #[doc = "0: No supply monitor detection has generated a core reset since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection has generated a core reset since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMRSTS_A> for bool {
    #[inline(always)]
    fn from(variant: SMRSTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMRSTS` reader - Supply Monitor Reset Status (cleared on read)"]
pub struct SMRSTS_R(crate::FieldReader<bool, SMRSTS_A>);
impl SMRSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMRSTS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMRSTS_A {
        match self.bits {
            false => SMRSTS_A::NO,
            true => SMRSTS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SMRSTS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == SMRSTS_A::PRESENT
    }
}
impl core::ops::Deref for SMRSTS_R {
    type Target = crate::FieldReader<bool, SMRSTS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Supply Monitor Status (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMS_A {
    #[doc = "0: No supply monitor detection since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one supply monitor detection since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<SMS_A> for bool {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMS` reader - Supply Monitor Status (cleared on read)"]
pub struct SMS_R(crate::FieldReader<bool, SMS_A>);
impl SMS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            false => SMS_A::NO,
            true => SMS_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == SMS_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == SMS_A::PRESENT
    }
}
impl core::ops::Deref for SMS_R {
    type Target = crate::FieldReader<bool, SMS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Supply Monitor Output Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMOS_A {
    #[doc = "0: The supply monitor detected VDDIO higher than its threshold at its last measurement."]
    HIGH = 0,
    #[doc = "1: The supply monitor detected VDDIO lower than its threshold at its last measurement."]
    LOW = 1,
}
impl From<SMOS_A> for bool {
    #[inline(always)]
    fn from(variant: SMOS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMOS` reader - Supply Monitor Output Status"]
pub struct SMOS_R(crate::FieldReader<bool, SMOS_A>);
impl SMOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMOS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMOS_A {
        match self.bits {
            false => SMOS_A::HIGH,
            true => SMOS_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SMOS_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SMOS_A::LOW
    }
}
impl core::ops::Deref for SMOS_R {
    type Target = crate::FieldReader<bool, SMOS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "32-kHz Oscillator Selection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSEL_A {
    #[doc = "0: The slow clock SLCK is generated by the embedded 32 kHz RC oscillator."]
    RC = 0,
    #[doc = "1: The slow clock SLCK is generated by the 32 kHz crystal oscillator."]
    CRYST = 1,
}
impl From<OSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSEL` reader - 32-kHz Oscillator Selection Status"]
pub struct OSCSEL_R(crate::FieldReader<bool, OSCSEL_A>);
impl OSCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OSCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSEL_A {
        match self.bits {
            false => OSCSEL_A::RC,
            true => OSCSEL_A::CRYST,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        **self == OSCSEL_A::RC
    }
    #[doc = "Checks if the value of the field is `CRYST`"]
    #[inline(always)]
    pub fn is_cryst(&self) -> bool {
        **self == OSCSEL_A::CRYST
    }
}
impl core::ops::Deref for OSCSEL_R {
    type Target = crate::FieldReader<bool, OSCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS0_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP0 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS0_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS0` reader - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
pub struct LPDBCS0_R(crate::FieldReader<bool, LPDBCS0_A>);
impl LPDBCS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDBCS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS0_A {
        match self.bits {
            false => LPDBCS0_A::NO,
            true => LPDBCS0_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == LPDBCS0_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == LPDBCS0_A::PRESENT
    }
}
impl core::ops::Deref for LPDBCS0_R {
    type Target = crate::FieldReader<bool, LPDBCS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPDBCS1_A {
    #[doc = "0: No wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    NO = 0,
    #[doc = "1: At least one wake-up due to the assertion of the WKUP1 pin has occurred since the last read of SUPC_SR."]
    PRESENT = 1,
}
impl From<LPDBCS1_A> for bool {
    #[inline(always)]
    fn from(variant: LPDBCS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPDBCS1` reader - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
pub struct LPDBCS1_R(crate::FieldReader<bool, LPDBCS1_A>);
impl LPDBCS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPDBCS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPDBCS1_A {
        match self.bits {
            false => LPDBCS1_A::NO,
            true => LPDBCS1_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == LPDBCS1_A::NO
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        **self == LPDBCS1_A::PRESENT
    }
}
impl core::ops::Deref for LPDBCS1_R {
    type Target = crate::FieldReader<bool, LPDBCS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 0 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS0_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS0` reader - WKUP Input Status 0 (cleared on read)"]
pub struct WKUPIS0_R(crate::FieldReader<bool, WKUPIS0_A>);
impl WKUPIS0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS0_A {
        match self.bits {
            false => WKUPIS0_A::DISABLED,
            true => WKUPIS0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS0_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS0_R {
    type Target = crate::FieldReader<bool, WKUPIS0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 1 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS1_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS1` reader - WKUP Input Status 1 (cleared on read)"]
pub struct WKUPIS1_R(crate::FieldReader<bool, WKUPIS1_A>);
impl WKUPIS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS1_A {
        match self.bits {
            false => WKUPIS1_A::DISABLED,
            true => WKUPIS1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS1_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS1_R {
    type Target = crate::FieldReader<bool, WKUPIS1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 2 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS2_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS2` reader - WKUP Input Status 2 (cleared on read)"]
pub struct WKUPIS2_R(crate::FieldReader<bool, WKUPIS2_A>);
impl WKUPIS2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS2_A {
        match self.bits {
            false => WKUPIS2_A::DISABLED,
            true => WKUPIS2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS2_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS2_R {
    type Target = crate::FieldReader<bool, WKUPIS2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 3 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS3_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS3` reader - WKUP Input Status 3 (cleared on read)"]
pub struct WKUPIS3_R(crate::FieldReader<bool, WKUPIS3_A>);
impl WKUPIS3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS3_A {
        match self.bits {
            false => WKUPIS3_A::DISABLED,
            true => WKUPIS3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS3_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS3_R {
    type Target = crate::FieldReader<bool, WKUPIS3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 4 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS4_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS4` reader - WKUP Input Status 4 (cleared on read)"]
pub struct WKUPIS4_R(crate::FieldReader<bool, WKUPIS4_A>);
impl WKUPIS4_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS4_A {
        match self.bits {
            false => WKUPIS4_A::DISABLED,
            true => WKUPIS4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS4_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS4_R {
    type Target = crate::FieldReader<bool, WKUPIS4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 5 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS5_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS5` reader - WKUP Input Status 5 (cleared on read)"]
pub struct WKUPIS5_R(crate::FieldReader<bool, WKUPIS5_A>);
impl WKUPIS5_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS5_A {
        match self.bits {
            false => WKUPIS5_A::DISABLED,
            true => WKUPIS5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS5_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS5_R {
    type Target = crate::FieldReader<bool, WKUPIS5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 6 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS6_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS6` reader - WKUP Input Status 6 (cleared on read)"]
pub struct WKUPIS6_R(crate::FieldReader<bool, WKUPIS6_A>);
impl WKUPIS6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS6_A {
        match self.bits {
            false => WKUPIS6_A::DISABLED,
            true => WKUPIS6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS6_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS6_R {
    type Target = crate::FieldReader<bool, WKUPIS6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 7 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS7_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS7` reader - WKUP Input Status 7 (cleared on read)"]
pub struct WKUPIS7_R(crate::FieldReader<bool, WKUPIS7_A>);
impl WKUPIS7_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS7_A {
        match self.bits {
            false => WKUPIS7_A::DISABLED,
            true => WKUPIS7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS7_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS7_R {
    type Target = crate::FieldReader<bool, WKUPIS7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 8 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS8_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS8` reader - WKUP Input Status 8 (cleared on read)"]
pub struct WKUPIS8_R(crate::FieldReader<bool, WKUPIS8_A>);
impl WKUPIS8_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS8_A {
        match self.bits {
            false => WKUPIS8_A::DISABLED,
            true => WKUPIS8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS8_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS8_R {
    type Target = crate::FieldReader<bool, WKUPIS8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 9 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS9_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS9` reader - WKUP Input Status 9 (cleared on read)"]
pub struct WKUPIS9_R(crate::FieldReader<bool, WKUPIS9_A>);
impl WKUPIS9_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS9_A {
        match self.bits {
            false => WKUPIS9_A::DISABLED,
            true => WKUPIS9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS9_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS9_R {
    type Target = crate::FieldReader<bool, WKUPIS9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 10 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS10_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS10` reader - WKUP Input Status 10 (cleared on read)"]
pub struct WKUPIS10_R(crate::FieldReader<bool, WKUPIS10_A>);
impl WKUPIS10_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS10_A {
        match self.bits {
            false => WKUPIS10_A::DISABLED,
            true => WKUPIS10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS10_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS10_R {
    type Target = crate::FieldReader<bool, WKUPIS10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 11 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS11_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS11` reader - WKUP Input Status 11 (cleared on read)"]
pub struct WKUPIS11_R(crate::FieldReader<bool, WKUPIS11_A>);
impl WKUPIS11_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS11_A {
        match self.bits {
            false => WKUPIS11_A::DISABLED,
            true => WKUPIS11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS11_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS11_R {
    type Target = crate::FieldReader<bool, WKUPIS11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 12 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS12_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS12` reader - WKUP Input Status 12 (cleared on read)"]
pub struct WKUPIS12_R(crate::FieldReader<bool, WKUPIS12_A>);
impl WKUPIS12_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS12_A {
        match self.bits {
            false => WKUPIS12_A::DISABLED,
            true => WKUPIS12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS12_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS12_R {
    type Target = crate::FieldReader<bool, WKUPIS12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 13 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS13_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS13` reader - WKUP Input Status 13 (cleared on read)"]
pub struct WKUPIS13_R(crate::FieldReader<bool, WKUPIS13_A>);
impl WKUPIS13_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS13_A {
        match self.bits {
            false => WKUPIS13_A::DISABLED,
            true => WKUPIS13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS13_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS13_R {
    type Target = crate::FieldReader<bool, WKUPIS13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 14 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS14_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS14_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS14` reader - WKUP Input Status 14 (cleared on read)"]
pub struct WKUPIS14_R(crate::FieldReader<bool, WKUPIS14_A>);
impl WKUPIS14_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS14_A {
        match self.bits {
            false => WKUPIS14_A::DISABLED,
            true => WKUPIS14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS14_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS14_R {
    type Target = crate::FieldReader<bool, WKUPIS14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "WKUP Input Status 15 (cleared on read)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPIS15_A {
    #[doc = "0: The corresponding wake-up input is disabled, or was inactive at the time the debouncer triggered a wake-up event."]
    DISABLED = 0,
    #[doc = "1: The corresponding wake-up input was active at the time the debouncer triggered a wake-up event since the last read of SUPC_SR."]
    ENABLED = 1,
}
impl From<WKUPIS15_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPIS15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPIS15` reader - WKUP Input Status 15 (cleared on read)"]
pub struct WKUPIS15_R(crate::FieldReader<bool, WKUPIS15_A>);
impl WKUPIS15_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKUPIS15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPIS15_A {
        match self.bits {
            false => WKUPIS15_A::DISABLED,
            true => WKUPIS15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WKUPIS15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WKUPIS15_A::ENABLED
    }
}
impl core::ops::Deref for WKUPIS15_R {
    type Target = crate::FieldReader<bool, WKUPIS15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - WKUP Wake-up Status (cleared on read)"]
    #[inline(always)]
    pub fn wkups(&self) -> WKUPS_R {
        WKUPS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Brownout Detector Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn bodrsts(&self) -> BODRSTS_R {
        BODRSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Supply Monitor Reset Status (cleared on read)"]
    #[inline(always)]
    pub fn smrsts(&self) -> SMRSTS_R {
        SMRSTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Supply Monitor Status (cleared on read)"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Supply Monitor Output Status"]
    #[inline(always)]
    pub fn smos(&self) -> SMOS_R {
        SMOS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 32-kHz Oscillator Selection Status"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Low-power Debouncer Wake-up Status on WKUP0 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs0(&self) -> LPDBCS0_R {
        LPDBCS0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Low-power Debouncer Wake-up Status on WKUP1 (cleared on read)"]
    #[inline(always)]
    pub fn lpdbcs1(&self) -> LPDBCS1_R {
        LPDBCS1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WKUP Input Status 0 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis0(&self) -> WKUPIS0_R {
        WKUPIS0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - WKUP Input Status 1 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis1(&self) -> WKUPIS1_R {
        WKUPIS1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - WKUP Input Status 2 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis2(&self) -> WKUPIS2_R {
        WKUPIS2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - WKUP Input Status 3 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis3(&self) -> WKUPIS3_R {
        WKUPIS3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - WKUP Input Status 4 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis4(&self) -> WKUPIS4_R {
        WKUPIS4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - WKUP Input Status 5 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis5(&self) -> WKUPIS5_R {
        WKUPIS5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - WKUP Input Status 6 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis6(&self) -> WKUPIS6_R {
        WKUPIS6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - WKUP Input Status 7 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis7(&self) -> WKUPIS7_R {
        WKUPIS7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - WKUP Input Status 8 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis8(&self) -> WKUPIS8_R {
        WKUPIS8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - WKUP Input Status 9 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis9(&self) -> WKUPIS9_R {
        WKUPIS9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - WKUP Input Status 10 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis10(&self) -> WKUPIS10_R {
        WKUPIS10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - WKUP Input Status 11 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis11(&self) -> WKUPIS11_R {
        WKUPIS11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - WKUP Input Status 12 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis12(&self) -> WKUPIS12_R {
        WKUPIS12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - WKUP Input Status 13 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis13(&self) -> WKUPIS13_R {
        WKUPIS13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - WKUP Input Status 14 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis14(&self) -> WKUPIS14_R {
        WKUPIS14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - WKUP Input Status 15 (cleared on read)"]
    #[inline(always)]
    pub fn wkupis15(&self) -> WKUPIS15_R {
        WKUPIS15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Supply Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
