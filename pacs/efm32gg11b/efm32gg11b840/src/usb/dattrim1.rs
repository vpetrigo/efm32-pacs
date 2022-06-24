#[doc = "Register `DATTRIM1` reader"]
pub struct R(crate::R<DATTRIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATTRIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATTRIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATTRIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATTRIM1` writer"]
pub struct W(crate::W<DATTRIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATTRIM1_SPEC>;
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
impl From<crate::W<DATTRIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATTRIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ROUT` reader - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type ROUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ROUT` writer - Trim for DP and DM Output Impedance for Both FS and LS"]
pub type ROUT_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 6, 0>;
#[doc = "Field `ENDLYPULLUP` reader - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type ENDLYPULLUP_R = crate::BitReader<bool>;
#[doc = "Field `ENDLYPULLUP` writer - Enables Delay of Pull in TX Mode for Both FS and LS"]
pub type ENDLYPULLUP_W<'a> = crate::BitWriter<'a, u32, DATTRIM1_SPEC, bool, 7>;
#[doc = "Field `DLYPULLUPFS` reader - Trim for Rising Crossover Voltage in FS"]
pub type DLYPULLUPFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYPULLUPFS` writer - Trim for Rising Crossover Voltage in FS"]
pub type DLYPULLUPFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 8>;
#[doc = "Field `VCRSFS` reader - Trim for Falling Crossover Voltage in FS"]
pub type VCRSFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCRSFS` writer - Trim for Falling Crossover Voltage in FS"]
pub type VCRSFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 10>;
#[doc = "Field `TFDMFS` reader - Trim for DM Fall Time in FS"]
pub type TFDMFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFDMFS` writer - Trim for DM Fall Time in FS"]
pub type TFDMFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 12>;
#[doc = "Field `TRDMFS` reader - Trim for DM Rise Time in FS"]
pub type TRDMFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDMFS` writer - Trim for DM Rise Time in FS"]
pub type TRDMFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 14>;
#[doc = "Field `TFDPFS` reader - Trim for DP Fall Time in FS"]
pub type TFDPFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFDPFS` writer - Trim for DP Fall Time in FS"]
pub type TFDPFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 16>;
#[doc = "Field `TRDPFS` reader - Trim for DP Rise Time in FS"]
pub type TRDPFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRDPFS` writer - Trim for DP Rise Time in FS"]
pub type TRDPFS_W<'a> = crate::FieldWriter<'a, u32, DATTRIM1_SPEC, u8, u8, 2, 18>;
impl R {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&self) -> ROUT_R {
        ROUT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&self) -> ENDLYPULLUP_R {
        ENDLYPULLUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&self) -> DLYPULLUPFS_R {
        DLYPULLUPFS_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&self) -> VCRSFS_R {
        VCRSFS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&self) -> TFDMFS_R {
        TFDMFS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&self) -> TRDMFS_R {
        TRDMFS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&self) -> TFDPFS_R {
        TFDPFS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&self) -> TRDPFS_R {
        TRDPFS_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline(always)]
    pub fn rout(&mut self) -> ROUT_W {
        ROUT_W::new(self)
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline(always)]
    pub fn endlypullup(&mut self) -> ENDLYPULLUP_W {
        ENDLYPULLUP_W::new(self)
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline(always)]
    pub fn dlypullupfs(&mut self) -> DLYPULLUPFS_W {
        DLYPULLUPFS_W::new(self)
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline(always)]
    pub fn vcrsfs(&mut self) -> VCRSFS_W {
        VCRSFS_W::new(self)
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline(always)]
    pub fn tfdmfs(&mut self) -> TFDMFS_W {
        TFDMFS_W::new(self)
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline(always)]
    pub fn trdmfs(&mut self) -> TRDMFS_W {
        TRDMFS_W::new(self)
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline(always)]
    pub fn tfdpfs(&mut self) -> TFDPFS_W {
        TFDPFS_W::new(self)
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline(always)]
    pub fn trdpfs(&mut self) -> TRDPFS_W {
        TRDPFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data TRIM 1 Values for USB DP and DM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dattrim1](index.html) module"]
pub struct DATTRIM1_SPEC;
impl crate::RegisterSpec for DATTRIM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dattrim1::R](R) reader structure"]
impl crate::Readable for DATTRIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dattrim1::W](W) writer structure"]
impl crate::Writable for DATTRIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATTRIM1 to value 0x24"]
impl crate::Resettable for DATTRIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x24
    }
}
