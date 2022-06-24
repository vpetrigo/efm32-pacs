#[doc = "Register `PPUSATD1` reader"]
pub struct R(crate::R<PPUSATD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUSATD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUSATD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUSATD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUSATD1` writer"]
pub struct W(crate::W<PPUSATD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUSATD1_SPEC>;
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
impl From<crate::W<PPUSATD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUSATD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDC` reader - DCDC Secure Access"]
pub type DCDC_R = crate::BitReader<bool>;
#[doc = "Field `DCDC` writer - DCDC Secure Access"]
pub type DCDC_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 1>;
#[doc = "Field `PDM` reader - PDM Secure Access"]
pub type PDM_R = crate::BitReader<bool>;
#[doc = "Field `PDM` writer - PDM Secure Access"]
pub type PDM_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 2>;
#[doc = "Field `SMU` reader - SMU Secure Access"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - SMU Secure Access"]
pub type SMU_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 5>;
#[doc = "Field `SMUCFGNS` reader - SMUCFGNS Secure Access"]
pub type SMUCFGNS_R = crate::BitReader<bool>;
#[doc = "Field `SMUCFGNS` writer - SMUCFGNS Secure Access"]
pub type SMUCFGNS_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 6>;
#[doc = "Field `RTCC` reader - RTCC Secure Access"]
pub type RTCC_R = crate::BitReader<bool>;
#[doc = "Field `RTCC` writer - RTCC Secure Access"]
pub type RTCC_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 7>;
#[doc = "Field `LETIMER0` reader - LETIMER0 Secure Access"]
pub type LETIMER0_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0` writer - LETIMER0 Secure Access"]
pub type LETIMER0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 8>;
#[doc = "Field `IADC0` reader - IADC0 Secure Access"]
pub type IADC0_R = crate::BitReader<bool>;
#[doc = "Field `IADC0` writer - IADC0 Secure Access"]
pub type IADC0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 9>;
#[doc = "Field `I2C0` reader - I2C0 Secure Access"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - I2C0 Secure Access"]
pub type I2C0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 10>;
#[doc = "Field `WDOG0` reader - WDOG0 Secure Access"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - WDOG0 Secure Access"]
pub type WDOG0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 11>;
#[doc = "Field `AMUXCP0` reader - AMUXCP0 Secure Access"]
pub type AMUXCP0_R = crate::BitReader<bool>;
#[doc = "Field `AMUXCP0` writer - AMUXCP0 Secure Access"]
pub type AMUXCP0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 12>;
#[doc = "Field `EUART0` reader - EUART0 Secure Access"]
pub type EUART0_R = crate::BitReader<bool>;
#[doc = "Field `EUART0` writer - EUART0 Secure Access"]
pub type EUART0_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 13>;
#[doc = "Field `CRYPTOACC` reader - CRYPTOACC Secure Access"]
pub type CRYPTOACC_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTOACC` writer - CRYPTOACC Secure Access"]
pub type CRYPTOACC_W<'a> = crate::BitWriter<'a, u32, PPUSATD1_SPEC, bool, 14>;
impl R {
    #[doc = "Bit 1 - DCDC Secure Access"]
    #[inline(always)]
    pub fn dcdc(&self) -> DCDC_R {
        DCDC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDM Secure Access"]
    #[inline(always)]
    pub fn pdm(&self) -> PDM_R {
        PDM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - SMU Secure Access"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMUCFGNS Secure Access"]
    #[inline(always)]
    pub fn smucfgns(&self) -> SMUCFGNS_R {
        SMUCFGNS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RTCC Secure Access"]
    #[inline(always)]
    pub fn rtcc(&self) -> RTCC_R {
        RTCC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LETIMER0 Secure Access"]
    #[inline(always)]
    pub fn letimer0(&self) -> LETIMER0_R {
        LETIMER0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IADC0 Secure Access"]
    #[inline(always)]
    pub fn iadc0(&self) -> IADC0_R {
        IADC0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C0 Secure Access"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - WDOG0 Secure Access"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AMUXCP0 Secure Access"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> AMUXCP0_R {
        AMUXCP0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EUART0 Secure Access"]
    #[inline(always)]
    pub fn euart0(&self) -> EUART0_R {
        EUART0_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRYPTOACC Secure Access"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CRYPTOACC_R {
        CRYPTOACC_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DCDC Secure Access"]
    #[inline(always)]
    pub fn dcdc(&mut self) -> DCDC_W {
        DCDC_W::new(self)
    }
    #[doc = "Bit 2 - PDM Secure Access"]
    #[inline(always)]
    pub fn pdm(&mut self) -> PDM_W {
        PDM_W::new(self)
    }
    #[doc = "Bit 5 - SMU Secure Access"]
    #[inline(always)]
    pub fn smu(&mut self) -> SMU_W {
        SMU_W::new(self)
    }
    #[doc = "Bit 6 - SMUCFGNS Secure Access"]
    #[inline(always)]
    pub fn smucfgns(&mut self) -> SMUCFGNS_W {
        SMUCFGNS_W::new(self)
    }
    #[doc = "Bit 7 - RTCC Secure Access"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RTCC_W {
        RTCC_W::new(self)
    }
    #[doc = "Bit 8 - LETIMER0 Secure Access"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> LETIMER0_W {
        LETIMER0_W::new(self)
    }
    #[doc = "Bit 9 - IADC0 Secure Access"]
    #[inline(always)]
    pub fn iadc0(&mut self) -> IADC0_W {
        IADC0_W::new(self)
    }
    #[doc = "Bit 10 - I2C0 Secure Access"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W::new(self)
    }
    #[doc = "Bit 11 - WDOG0 Secure Access"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 12 - AMUXCP0 Secure Access"]
    #[inline(always)]
    pub fn amuxcp0(&mut self) -> AMUXCP0_W {
        AMUXCP0_W::new(self)
    }
    #[doc = "Bit 13 - EUART0 Secure Access"]
    #[inline(always)]
    pub fn euart0(&mut self) -> EUART0_W {
        EUART0_W::new(self)
    }
    #[doc = "Bit 14 - CRYPTOACC Secure Access"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CRYPTOACC_W {
        CRYPTOACC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set peripheral bits to 1 to mark as secure access only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppusatd1](index.html) module"]
pub struct PPUSATD1_SPEC;
impl crate::RegisterSpec for PPUSATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppusatd1::R](R) reader structure"]
impl crate::Readable for PPUSATD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppusatd1::W](W) writer structure"]
impl crate::Writable for PPUSATD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPUSATD1 to value 0xffff"]
impl crate::Resettable for PPUSATD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
