#[doc = "Register `CFGPRESETVAL2` reader"]
pub struct R(crate::R<CFGPRESETVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGPRESETVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGPRESETVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGPRESETVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGPRESETVAL2` writer"]
pub struct W(crate::W<CFGPRESETVAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGPRESETVAL2_SPEC>;
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
impl From<crate::W<CFGPRESETVAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGPRESETVAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDR25SDCLKFREQ` reader - SDR25 SD_CLK Frequency"]
pub type SDR25SDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDR25SDCLKFREQ` writer - SDR25 SD_CLK Frequency"]
pub type SDR25SDCLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL2_SPEC, u16, u16, 10, 0>;
#[doc = "Field `SDR25CLKGENEN` reader - SDR25 SD_CLK Gen Enable"]
pub type SDR25CLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `SDR25CLKGENEN` writer - SDR25 SD_CLK Gen Enable"]
pub type SDR25CLKGENEN_W<'a> = crate::BitWriter<'a, u32, CFGPRESETVAL2_SPEC, bool, 10>;
#[doc = "Field `SDR25DRVST` reader - SDR25 SD Drive Strength"]
pub type SDR25DRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDR25DRVST` writer - SDR25 SD Drive Strength"]
pub type SDR25DRVST_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL2_SPEC, u8, u8, 2, 11>;
#[doc = "Field `SDR50SDCLKFREQ` reader - Preset Value for SDR50 Speed of SD_CLK"]
pub type SDR50SDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDR50SDCLKFREQ` writer - Preset Value for SDR50 Speed of SD_CLK"]
pub type SDR50SDCLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL2_SPEC, u16, u16, 10, 16>;
#[doc = "Field `SDR50CLKGENEN` reader - SDR50 Speed Clock Gen Enable"]
pub type SDR50CLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `SDR50CLKGENEN` writer - SDR50 Speed Clock Gen Enable"]
pub type SDR50CLKGENEN_W<'a> = crate::BitWriter<'a, u32, CFGPRESETVAL2_SPEC, bool, 26>;
#[doc = "Field `SDR50DRVST` reader - SDR50 Speed Drive Strength"]
pub type SDR50DRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDR50DRVST` writer - SDR50 Speed Drive Strength"]
pub type SDR50DRVST_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL2_SPEC, u8, u8, 2, 27>;
impl R {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&self) -> SDR25SDCLKFREQ_R {
        SDR25SDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&self) -> SDR25CLKGENEN_R {
        SDR25CLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&self) -> SDR25DRVST_R {
        SDR25DRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&self) -> SDR50SDCLKFREQ_R {
        SDR50SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&self) -> SDR50CLKGENEN_R {
        SDR50CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&self) -> SDR50DRVST_R {
        SDR50DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - SDR25 SD_CLK Frequency"]
    #[inline(always)]
    pub fn sdr25sdclkfreq(&mut self) -> SDR25SDCLKFREQ_W {
        SDR25SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - SDR25 SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn sdr25clkgenen(&mut self) -> SDR25CLKGENEN_W {
        SDR25CLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - SDR25 SD Drive Strength"]
    #[inline(always)]
    pub fn sdr25drvst(&mut self) -> SDR25DRVST_W {
        SDR25DRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR50 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr50sdclkfreq(&mut self) -> SDR50SDCLKFREQ_W {
        SDR50SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - SDR50 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr50clkgenen(&mut self) -> SDR50CLKGENEN_W {
        SDR50CLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - SDR50 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr50drvst(&mut self) -> SDR50DRVST_W {
        SDR50DRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration Preset Value 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgpresetval2](index.html) module"]
pub struct CFGPRESETVAL2_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgpresetval2::R](R) reader structure"]
impl crate::Readable for CFGPRESETVAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgpresetval2::W](W) writer structure"]
impl crate::Writable for CFGPRESETVAL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGPRESETVAL2 to value 0"]
impl crate::Resettable for CFGPRESETVAL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
