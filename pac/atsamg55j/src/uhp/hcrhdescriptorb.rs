#[doc = "Register `HCRHDESCRIPTORB` reader"]
pub struct R(crate::R<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHDESCRIPTORB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHDESCRIPTORB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHDESCRIPTORB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHDESCRIPTORB` writer"]
pub struct W(crate::W<HCRHDESCRIPTORB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHDESCRIPTORB_SPEC>;
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
impl From<crate::W<HCRHDESCRIPTORB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHDESCRIPTORB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR0` reader - "]
pub struct DR0_R(crate::FieldReader<bool, bool>);
impl DR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR0` writer - "]
pub struct DR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DR0_W<'a> {
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
#[doc = "Field `DR1` reader - "]
pub struct DR1_R(crate::FieldReader<bool, bool>);
impl DR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR1` writer - "]
pub struct DR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DR1_W<'a> {
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
#[doc = "Field `DR2` reader - "]
pub struct DR2_R(crate::FieldReader<bool, bool>);
impl DR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR2` writer - "]
pub struct DR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DR2_W<'a> {
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
#[doc = "Field `DR3` reader - "]
pub struct DR3_R(crate::FieldReader<bool, bool>);
impl DR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR3` writer - "]
pub struct DR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DR3_W<'a> {
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
#[doc = "Field `DR4` reader - "]
pub struct DR4_R(crate::FieldReader<bool, bool>);
impl DR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR4` writer - "]
pub struct DR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DR4_W<'a> {
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
#[doc = "Field `DR5` reader - "]
pub struct DR5_R(crate::FieldReader<bool, bool>);
impl DR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR5` writer - "]
pub struct DR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DR6` reader - "]
pub struct DR6_R(crate::FieldReader<bool, bool>);
impl DR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR6` writer - "]
pub struct DR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DR6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `DR7` reader - "]
pub struct DR7_R(crate::FieldReader<bool, bool>);
impl DR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR7` writer - "]
pub struct DR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DR7_W<'a> {
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
#[doc = "Field `DR8` reader - "]
pub struct DR8_R(crate::FieldReader<bool, bool>);
impl DR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR8` writer - "]
pub struct DR8_W<'a> {
    w: &'a mut W,
}
impl<'a> DR8_W<'a> {
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
#[doc = "Field `DR9` reader - "]
pub struct DR9_R(crate::FieldReader<bool, bool>);
impl DR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR9` writer - "]
pub struct DR9_W<'a> {
    w: &'a mut W,
}
impl<'a> DR9_W<'a> {
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
#[doc = "Field `DR10` reader - "]
pub struct DR10_R(crate::FieldReader<bool, bool>);
impl DR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR10` writer - "]
pub struct DR10_W<'a> {
    w: &'a mut W,
}
impl<'a> DR10_W<'a> {
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
#[doc = "Field `DR11` reader - "]
pub struct DR11_R(crate::FieldReader<bool, bool>);
impl DR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR11` writer - "]
pub struct DR11_W<'a> {
    w: &'a mut W,
}
impl<'a> DR11_W<'a> {
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
#[doc = "Field `DR12` reader - "]
pub struct DR12_R(crate::FieldReader<bool, bool>);
impl DR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR12` writer - "]
pub struct DR12_W<'a> {
    w: &'a mut W,
}
impl<'a> DR12_W<'a> {
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
#[doc = "Field `DR13` reader - "]
pub struct DR13_R(crate::FieldReader<bool, bool>);
impl DR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR13` writer - "]
pub struct DR13_W<'a> {
    w: &'a mut W,
}
impl<'a> DR13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DR14` reader - "]
pub struct DR14_R(crate::FieldReader<bool, bool>);
impl DR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR14` writer - "]
pub struct DR14_W<'a> {
    w: &'a mut W,
}
impl<'a> DR14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `DR15` reader - "]
pub struct DR15_R(crate::FieldReader<bool, bool>);
impl DR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        DR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DR15` writer - "]
pub struct DR15_W<'a> {
    w: &'a mut W,
}
impl<'a> DR15_W<'a> {
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
#[doc = "Field `PPCM0` reader - "]
pub struct PPCM0_R(crate::FieldReader<bool, bool>);
impl PPCM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM0` writer - "]
pub struct PPCM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM0_W<'a> {
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
#[doc = "Field `PPCM1` reader - "]
pub struct PPCM1_R(crate::FieldReader<bool, bool>);
impl PPCM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM1` writer - "]
pub struct PPCM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM1_W<'a> {
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
#[doc = "Field `PPCM2` reader - "]
pub struct PPCM2_R(crate::FieldReader<bool, bool>);
impl PPCM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM2` writer - "]
pub struct PPCM2_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM2_W<'a> {
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
#[doc = "Field `PPCM3` reader - "]
pub struct PPCM3_R(crate::FieldReader<bool, bool>);
impl PPCM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM3` writer - "]
pub struct PPCM3_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM3_W<'a> {
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
#[doc = "Field `PPCM4` reader - "]
pub struct PPCM4_R(crate::FieldReader<bool, bool>);
impl PPCM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM4` writer - "]
pub struct PPCM4_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM4_W<'a> {
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
#[doc = "Field `PPCM5` reader - "]
pub struct PPCM5_R(crate::FieldReader<bool, bool>);
impl PPCM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM5` writer - "]
pub struct PPCM5_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM5_W<'a> {
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
#[doc = "Field `PPCM6` reader - "]
pub struct PPCM6_R(crate::FieldReader<bool, bool>);
impl PPCM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM6` writer - "]
pub struct PPCM6_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM6_W<'a> {
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
#[doc = "Field `PPCM7` reader - "]
pub struct PPCM7_R(crate::FieldReader<bool, bool>);
impl PPCM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM7` writer - "]
pub struct PPCM7_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM7_W<'a> {
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
#[doc = "Field `PPCM8` reader - "]
pub struct PPCM8_R(crate::FieldReader<bool, bool>);
impl PPCM8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM8` writer - "]
pub struct PPCM8_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM8_W<'a> {
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
#[doc = "Field `PPCM9` reader - "]
pub struct PPCM9_R(crate::FieldReader<bool, bool>);
impl PPCM9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM9` writer - "]
pub struct PPCM9_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PPCM10` reader - "]
pub struct PPCM10_R(crate::FieldReader<bool, bool>);
impl PPCM10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM10` writer - "]
pub struct PPCM10_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PPCM11` reader - "]
pub struct PPCM11_R(crate::FieldReader<bool, bool>);
impl PPCM11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM11` writer - "]
pub struct PPCM11_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PPCM12` reader - "]
pub struct PPCM12_R(crate::FieldReader<bool, bool>);
impl PPCM12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM12` writer - "]
pub struct PPCM12_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PPCM13` reader - "]
pub struct PPCM13_R(crate::FieldReader<bool, bool>);
impl PPCM13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM13` writer - "]
pub struct PPCM13_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PPCM14` reader - "]
pub struct PPCM14_R(crate::FieldReader<bool, bool>);
impl PPCM14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM14` writer - "]
pub struct PPCM14_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PPCM15` reader - "]
pub struct PPCM15_R(crate::FieldReader<bool, bool>);
impl PPCM15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPCM15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPCM15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPCM15` writer - "]
pub struct PPCM15_W<'a> {
    w: &'a mut W,
}
impl<'a> PPCM15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dr0(&self) -> DR0_R {
        DR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dr1(&self) -> DR1_R {
        DR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dr2(&self) -> DR2_R {
        DR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dr3(&self) -> DR3_R {
        DR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dr4(&self) -> DR4_R {
        DR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dr5(&self) -> DR5_R {
        DR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dr6(&self) -> DR6_R {
        DR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dr7(&self) -> DR7_R {
        DR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dr8(&self) -> DR8_R {
        DR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dr9(&self) -> DR9_R {
        DR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dr10(&self) -> DR10_R {
        DR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dr11(&self) -> DR11_R {
        DR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dr12(&self) -> DR12_R {
        DR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dr13(&self) -> DR13_R {
        DR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dr14(&self) -> DR14_R {
        DR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dr15(&self) -> DR15_R {
        DR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ppcm0(&self) -> PPCM0_R {
        PPCM0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ppcm1(&self) -> PPCM1_R {
        PPCM1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ppcm2(&self) -> PPCM2_R {
        PPCM2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ppcm3(&self) -> PPCM3_R {
        PPCM3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppcm4(&self) -> PPCM4_R {
        PPCM4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppcm5(&self) -> PPCM5_R {
        PPCM5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppcm6(&self) -> PPCM6_R {
        PPCM6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppcm7(&self) -> PPCM7_R {
        PPCM7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppcm8(&self) -> PPCM8_R {
        PPCM8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppcm9(&self) -> PPCM9_R {
        PPCM9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ppcm10(&self) -> PPCM10_R {
        PPCM10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ppcm11(&self) -> PPCM11_R {
        PPCM11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ppcm12(&self) -> PPCM12_R {
        PPCM12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ppcm13(&self) -> PPCM13_R {
        PPCM13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppcm14(&self) -> PPCM14_R {
        PPCM14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ppcm15(&self) -> PPCM15_R {
        PPCM15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dr0(&mut self) -> DR0_W {
        DR0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn dr1(&mut self) -> DR1_W {
        DR1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dr2(&mut self) -> DR2_W {
        DR2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dr3(&mut self) -> DR3_W {
        DR3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dr4(&mut self) -> DR4_W {
        DR4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dr5(&mut self) -> DR5_W {
        DR5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dr6(&mut self) -> DR6_W {
        DR6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dr7(&mut self) -> DR7_W {
        DR7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dr8(&mut self) -> DR8_W {
        DR8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dr9(&mut self) -> DR9_W {
        DR9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dr10(&mut self) -> DR10_W {
        DR10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dr11(&mut self) -> DR11_W {
        DR11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dr12(&mut self) -> DR12_W {
        DR12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dr13(&mut self) -> DR13_W {
        DR13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dr14(&mut self) -> DR14_W {
        DR14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dr15(&mut self) -> DR15_W {
        DR15_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ppcm0(&mut self) -> PPCM0_W {
        PPCM0_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ppcm1(&mut self) -> PPCM1_W {
        PPCM1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ppcm2(&mut self) -> PPCM2_W {
        PPCM2_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ppcm3(&mut self) -> PPCM3_W {
        PPCM3_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ppcm4(&mut self) -> PPCM4_W {
        PPCM4_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ppcm5(&mut self) -> PPCM5_W {
        PPCM5_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ppcm6(&mut self) -> PPCM6_W {
        PPCM6_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn ppcm7(&mut self) -> PPCM7_W {
        PPCM7_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ppcm8(&mut self) -> PPCM8_W {
        PPCM8_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ppcm9(&mut self) -> PPCM9_W {
        PPCM9_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ppcm10(&mut self) -> PPCM10_W {
        PPCM10_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ppcm11(&mut self) -> PPCM11_W {
        PPCM11_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ppcm12(&mut self) -> PPCM12_W {
        PPCM12_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ppcm13(&mut self) -> PPCM13_W {
        PPCM13_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ppcm14(&mut self) -> PPCM14_W {
        PPCM14_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn ppcm15(&mut self) -> PPCM15_W {
        PPCM15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HC Root Hub B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhdescriptorb](index.html) module"]
pub struct HCRHDESCRIPTORB_SPEC;
impl crate::RegisterSpec for HCRHDESCRIPTORB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhdescriptorb::R](R) reader structure"]
impl crate::Readable for HCRHDESCRIPTORB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhdescriptorb::W](W) writer structure"]
impl crate::Writable for HCRHDESCRIPTORB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCRHDESCRIPTORB to value 0"]
impl crate::Resettable for HCRHDESCRIPTORB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
