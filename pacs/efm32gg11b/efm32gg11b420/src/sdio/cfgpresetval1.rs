#[doc = "Register `CFGPRESETVAL1` reader"]
pub struct R(crate::R<CFGPRESETVAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGPRESETVAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGPRESETVAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGPRESETVAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGPRESETVAL1` writer"]
pub struct W(crate::W<CFGPRESETVAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGPRESETVAL1_SPEC>;
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
impl From<crate::W<CFGPRESETVAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGPRESETVAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSPSDCLKFREQ` reader - High Speed SD_CLK Frequency"]
pub type HSPSDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HSPSDCLKFREQ` writer - High Speed SD_CLK Frequency"]
pub type HSPSDCLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `HSPCLKGENEN` reader - High Speed SD_CLK Gen Enable"]
pub type HSPCLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `HSPCLKGENEN` writer - High Speed SD_CLK Gen Enable"]
pub type HSPCLKGENEN_W<'a> = crate::BitWriter<'a, u32, CFGPRESETVAL1_SPEC, bool, 10>;
#[doc = "Field `HSPDRVST` reader - High Speed SD Drive Strength"]
pub type HSPDRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSPDRVST` writer - High Speed SD Drive Strength"]
pub type HSPDRVST_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL1_SPEC, u8, u8, 2, 11>;
#[doc = "Field `SDR12SDCLKFREQ` reader - Preset Value for SDR12 Speed of SD_CLK"]
pub type SDR12SDCLKFREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SDR12SDCLKFREQ` writer - Preset Value for SDR12 Speed of SD_CLK"]
pub type SDR12SDCLKFREQ_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL1_SPEC, u16, u16, 10, 16>;
#[doc = "Field `SDR12CLKGENEN` reader - SDR12 Speed Clock Gen Enable"]
pub type SDR12CLKGENEN_R = crate::BitReader<bool>;
#[doc = "Field `SDR12CLKGENEN` writer - SDR12 Speed Clock Gen Enable"]
pub type SDR12CLKGENEN_W<'a> = crate::BitWriter<'a, u32, CFGPRESETVAL1_SPEC, bool, 26>;
#[doc = "Field `SDR12DRVST` reader - SDR12 Speed Drive Strength"]
pub type SDR12DRVST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDR12DRVST` writer - SDR12 Speed Drive Strength"]
pub type SDR12DRVST_W<'a> = crate::FieldWriter<'a, u32, CFGPRESETVAL1_SPEC, u8, u8, 2, 27>;
impl R {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&self) -> HSPSDCLKFREQ_R {
        HSPSDCLKFREQ_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&self) -> HSPCLKGENEN_R {
        HSPCLKGENEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&self) -> HSPDRVST_R {
        HSPDRVST_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&self) -> SDR12SDCLKFREQ_R {
        SDR12SDCLKFREQ_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&self) -> SDR12CLKGENEN_R {
        SDR12CLKGENEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&self) -> SDR12DRVST_R {
        SDR12DRVST_R::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - High Speed SD_CLK Frequency"]
    #[inline(always)]
    pub fn hspsdclkfreq(&mut self) -> HSPSDCLKFREQ_W {
        HSPSDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 10 - High Speed SD_CLK Gen Enable"]
    #[inline(always)]
    pub fn hspclkgenen(&mut self) -> HSPCLKGENEN_W {
        HSPCLKGENEN_W::new(self)
    }
    #[doc = "Bits 11:12 - High Speed SD Drive Strength"]
    #[inline(always)]
    pub fn hspdrvst(&mut self) -> HSPDRVST_W {
        HSPDRVST_W::new(self)
    }
    #[doc = "Bits 16:25 - Preset Value for SDR12 Speed of SD_CLK"]
    #[inline(always)]
    pub fn sdr12sdclkfreq(&mut self) -> SDR12SDCLKFREQ_W {
        SDR12SDCLKFREQ_W::new(self)
    }
    #[doc = "Bit 26 - SDR12 Speed Clock Gen Enable"]
    #[inline(always)]
    pub fn sdr12clkgenen(&mut self) -> SDR12CLKGENEN_W {
        SDR12CLKGENEN_W::new(self)
    }
    #[doc = "Bits 27:28 - SDR12 Speed Drive Strength"]
    #[inline(always)]
    pub fn sdr12drvst(&mut self) -> SDR12DRVST_W {
        SDR12DRVST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Configuration Preset Value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgpresetval1](index.html) module"]
pub struct CFGPRESETVAL1_SPEC;
impl crate::RegisterSpec for CFGPRESETVAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgpresetval1::R](R) reader structure"]
impl crate::Readable for CFGPRESETVAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgpresetval1::W](W) writer structure"]
impl crate::Writable for CFGPRESETVAL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGPRESETVAL1 to value 0"]
impl crate::Resettable for CFGPRESETVAL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
