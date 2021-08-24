#[doc = "Register `HCRHPORTSTATUS[%s]` reader"]
pub struct R(crate::R<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCRHPORTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCRHPORTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCRHPORTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCRHPORTSTATUS[%s]` writer"]
pub struct W(crate::W<HCRHPORTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCRHPORTSTATUS_SPEC>;
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
impl From<crate::W<HCRHPORTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCRHPORTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCS_CPE` reader - "]
pub struct CCS_CPE_R(crate::FieldReader<bool, bool>);
impl CCS_CPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCS_CPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCS_CPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCS_CPE` writer - "]
pub struct CCS_CPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCS_CPE_W<'a> {
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
#[doc = "Field `PES_SPE` reader - "]
pub struct PES_SPE_R(crate::FieldReader<bool, bool>);
impl PES_SPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PES_SPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PES_SPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PES_SPE` writer - "]
pub struct PES_SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PES_SPE_W<'a> {
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
#[doc = "Field `PSS_SPS` reader - "]
pub struct PSS_SPS_R(crate::FieldReader<bool, bool>);
impl PSS_SPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS_SPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS_SPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS_SPS` writer - "]
pub struct PSS_SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_SPS_W<'a> {
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
#[doc = "Field `POCI_CSS` reader - "]
pub struct POCI_CSS_R(crate::FieldReader<bool, bool>);
impl POCI_CSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        POCI_CSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POCI_CSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POCI_CSS` writer - "]
pub struct POCI_CSS_W<'a> {
    w: &'a mut W,
}
impl<'a> POCI_CSS_W<'a> {
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
#[doc = "Field `PRS_SPR` reader - "]
pub struct PRS_SPR_R(crate::FieldReader<bool, bool>);
impl PRS_SPR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRS_SPR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRS_SPR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRS_SPR` writer - "]
pub struct PRS_SPR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_SPR_W<'a> {
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
#[doc = "Field `PPS_SPP` reader - "]
pub struct PPS_SPP_R(crate::FieldReader<bool, bool>);
impl PPS_SPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PPS_SPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PPS_SPP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPS_SPP` writer - "]
pub struct PPS_SPP_W<'a> {
    w: &'a mut W,
}
impl<'a> PPS_SPP_W<'a> {
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
#[doc = "Field `LSDA_CPP` reader - "]
pub struct LSDA_CPP_R(crate::FieldReader<bool, bool>);
impl LSDA_CPP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSDA_CPP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSDA_CPP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSDA_CPP` writer - "]
pub struct LSDA_CPP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSDA_CPP_W<'a> {
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
#[doc = "Field `CSC` reader - Port 1 connect status change (read/write, write '1' to clear)"]
pub struct CSC_R(crate::FieldReader<bool, bool>);
impl CSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSC` writer - Port 1 connect status change (read/write, write '1' to clear)"]
pub struct CSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSC_W<'a> {
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
#[doc = "Field `PESC` reader - Port 1 enable status change (read/write, write '1' to clear)"]
pub struct PESC_R(crate::FieldReader<bool, bool>);
impl PESC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PESC` writer - Port 1 enable status change (read/write, write '1' to clear)"]
pub struct PESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PESC_W<'a> {
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
#[doc = "Field `PSSC` reader - Port 1 suspend status change (read/write, write '1' to clear)"]
pub struct PSSC_R(crate::FieldReader<bool, bool>);
impl PSSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSSC` writer - Port 1 suspend status change (read/write, write '1' to clear)"]
pub struct PSSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSSC_W<'a> {
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
#[doc = "Field `OCIC` reader - Port 1 overcurrent indicator change (read/write)"]
pub struct OCIC_R(crate::FieldReader<bool, bool>);
impl OCIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCIC` writer - Port 1 overcurrent indicator change (read/write)"]
pub struct OCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OCIC_W<'a> {
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
#[doc = "Field `PRSC` reader - Port 1 reset status change (read/write, write '1' to clear)"]
pub struct PRSC_R(crate::FieldReader<bool, bool>);
impl PRSC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRSC` writer - Port 1 reset status change (read/write, write '1' to clear)"]
pub struct PRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSC_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccs_cpe(&self) -> CCS_CPE_R {
        CCS_CPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_spe(&self) -> PES_SPE_R {
        PES_SPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pss_sps(&self) -> PSS_SPS_R {
        PSS_SPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn poci_css(&self) -> POCI_CSS_R {
        POCI_CSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prs_spr(&self) -> PRS_SPR_R {
        PRS_SPR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pps_spp(&self) -> PPS_SPP_R {
        PPS_SPP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lsda_cpp(&self) -> LSDA_CPP_R {
        LSDA_CPP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Port 1 connect status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn csc(&self) -> CSC_R {
        CSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Port 1 enable status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pesc(&self) -> PESC_R {
        PESC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Port 1 suspend status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Port 1 overcurrent indicator change (read/write)"]
    #[inline(always)]
    pub fn ocic(&self) -> OCIC_R {
        OCIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Port 1 reset status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn prsc(&self) -> PRSC_R {
        PRSC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccs_cpe(&mut self) -> CCS_CPE_W {
        CCS_CPE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pes_spe(&mut self) -> PES_SPE_W {
        PES_SPE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pss_sps(&mut self) -> PSS_SPS_W {
        PSS_SPS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn poci_css(&mut self) -> POCI_CSS_W {
        POCI_CSS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prs_spr(&mut self) -> PRS_SPR_W {
        PRS_SPR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pps_spp(&mut self) -> PPS_SPP_W {
        PPS_SPP_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lsda_cpp(&mut self) -> LSDA_CPP_W {
        LSDA_CPP_W { w: self }
    }
    #[doc = "Bit 16 - Port 1 connect status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn csc(&mut self) -> CSC_W {
        CSC_W { w: self }
    }
    #[doc = "Bit 17 - Port 1 enable status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pesc(&mut self) -> PESC_W {
        PESC_W { w: self }
    }
    #[doc = "Bit 18 - Port 1 suspend status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn pssc(&mut self) -> PSSC_W {
        PSSC_W { w: self }
    }
    #[doc = "Bit 19 - Port 1 overcurrent indicator change (read/write)"]
    #[inline(always)]
    pub fn ocic(&mut self) -> OCIC_W {
        OCIC_W { w: self }
    }
    #[doc = "Bit 20 - Port 1 reset status change (read/write, write '1' to clear)"]
    #[inline(always)]
    pub fn prsc(&mut self) -> PRSC_W {
        PRSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HC Port 1 Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcrhportstatus](index.html) module"]
pub struct HCRHPORTSTATUS_SPEC;
impl crate::RegisterSpec for HCRHPORTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcrhportstatus::R](R) reader structure"]
impl crate::Readable for HCRHPORTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcrhportstatus::W](W) writer structure"]
impl crate::Writable for HCRHPORTSTATUS_SPEC {
    type Writer = W;
}
