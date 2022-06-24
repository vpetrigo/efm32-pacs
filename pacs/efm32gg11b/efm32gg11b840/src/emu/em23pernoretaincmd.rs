#[doc = "Register `EM23PERNORETAINCMD` writer"]
pub struct W(crate::W<EM23PERNORETAINCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM23PERNORETAINCMD_SPEC>;
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
impl From<crate::W<EM23PERNORETAINCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM23PERNORETAINCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP0UNLOCK` writer - Clears Status Bit of ACMP0 and Unlocks Access to It"]
pub type ACMP0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 0>;
#[doc = "Field `ACMP1UNLOCK` writer - Clears Status Bit of ACMP1 and Unlocks Access to It"]
pub type ACMP1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 1>;
#[doc = "Field `PCNT0UNLOCK` writer - Clears Status Bit of PCNT0 and Unlocks Access to It"]
pub type PCNT0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 2>;
#[doc = "Field `PCNT1UNLOCK` writer - Clears Status Bit of PCNT1 and Unlocks Access to It"]
pub type PCNT1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 3>;
#[doc = "Field `PCNT2UNLOCK` writer - Clears Status Bit of PCNT2 and Unlocks Access to It"]
pub type PCNT2UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 4>;
#[doc = "Field `I2C0UNLOCK` writer - Clears Status Bit of I2C0 and Unlocks Access to It"]
pub type I2C0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 5>;
#[doc = "Field `I2C1UNLOCK` writer - Clears Status Bit of I2C1 and Unlocks Access to It"]
pub type I2C1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 6>;
#[doc = "Field `DAC0UNLOCK` writer - Clears Status Bit of DAC0 and Unlocks Access to It"]
pub type DAC0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 7>;
#[doc = "Field `IDAC0UNLOCK` writer - Clears Status Bit of IDAC0 and Unlocks Access to It"]
pub type IDAC0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 8>;
#[doc = "Field `ADC0UNLOCK` writer - Clears Status Bit of ADC0 and Unlocks Access to It"]
pub type ADC0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 9>;
#[doc = "Field `LETIMER0UNLOCK` writer - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
pub type LETIMER0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 10>;
#[doc = "Field `WDOG0UNLOCK` writer - Clears Status Bit of WDOG0 and Unlocks Access to It"]
pub type WDOG0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 11>;
#[doc = "Field `WDOG1UNLOCK` writer - Clears Status Bit of WDOG1 and Unlocks Access to It"]
pub type WDOG1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 12>;
#[doc = "Field `LESENSE0UNLOCK` writer - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
pub type LESENSE0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 13>;
#[doc = "Field `CSENUNLOCK` writer - Clears Status Bit of CSEN and Unlocks Access to It"]
pub type CSENUNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 14>;
#[doc = "Field `LEUART0UNLOCK` writer - Clears Status Bit of LEUART0 and Unlocks Access to It"]
pub type LEUART0UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 15>;
#[doc = "Field `LEUART1UNLOCK` writer - Clears Status Bit of LEUART1 and Unlocks Access to It"]
pub type LEUART1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 16>;
#[doc = "Field `LCDUNLOCK` writer - Clears Status Bit of LCD and Unlocks Access to It"]
pub type LCDUNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 17>;
#[doc = "Field `LETIMER1UNLOCK` writer - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
pub type LETIMER1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 18>;
#[doc = "Field `I2C2UNLOCK` writer - Clears Status Bit of I2C2 and Unlocks Access to It"]
pub type I2C2UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 19>;
#[doc = "Field `ADC1UNLOCK` writer - Clears Status Bit of ADC1 and Unlocks Access to It"]
pub type ADC1UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 20>;
#[doc = "Field `ACMP2UNLOCK` writer - Clears Status Bit of ACMP2 and Unlocks Access to It"]
pub type ACMP2UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 21>;
#[doc = "Field `ACMP3UNLOCK` writer - Clears Status Bit of ACMP3 and Unlocks Access to It"]
pub type ACMP3UNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 22>;
#[doc = "Field `RTCUNLOCK` writer - Clears Status Bit of RTC and Unlocks Access to It"]
pub type RTCUNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 23>;
#[doc = "Field `USBUNLOCK` writer - Clears Status Bit of USB and Unlocks Access to It"]
pub type USBUNLOCK_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, 24>;
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp0unlock(&mut self) -> ACMP0UNLOCK_W {
        ACMP0UNLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp1unlock(&mut self) -> ACMP1UNLOCK_W {
        ACMP1UNLOCK_W::new(self)
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt0unlock(&mut self) -> PCNT0UNLOCK_W {
        PCNT0UNLOCK_W::new(self)
    }
    #[doc = "Bit 3 - Clears Status Bit of PCNT1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt1unlock(&mut self) -> PCNT1UNLOCK_W {
        PCNT1UNLOCK_W::new(self)
    }
    #[doc = "Bit 4 - Clears Status Bit of PCNT2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn pcnt2unlock(&mut self) -> PCNT2UNLOCK_W {
        PCNT2UNLOCK_W::new(self)
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c0unlock(&mut self) -> I2C0UNLOCK_W {
        I2C0UNLOCK_W::new(self)
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c1unlock(&mut self) -> I2C1UNLOCK_W {
        I2C1UNLOCK_W::new(self)
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn dac0unlock(&mut self) -> DAC0UNLOCK_W {
        DAC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 8 - Clears Status Bit of IDAC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn idac0unlock(&mut self) -> IDAC0UNLOCK_W {
        IDAC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc0unlock(&mut self) -> ADC0UNLOCK_W {
        ADC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer0unlock(&mut self) -> LETIMER0UNLOCK_W {
        LETIMER0UNLOCK_W::new(self)
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog0unlock(&mut self) -> WDOG0UNLOCK_W {
        WDOG0UNLOCK_W::new(self)
    }
    #[doc = "Bit 12 - Clears Status Bit of WDOG1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn wdog1unlock(&mut self) -> WDOG1UNLOCK_W {
        WDOG1UNLOCK_W::new(self)
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn lesense0unlock(&mut self) -> LESENSE0UNLOCK_W {
        LESENSE0UNLOCK_W::new(self)
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    pub fn csenunlock(&mut self) -> CSENUNLOCK_W {
        CSENUNLOCK_W::new(self)
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart0unlock(&mut self) -> LEUART0UNLOCK_W {
        LEUART0UNLOCK_W::new(self)
    }
    #[doc = "Bit 16 - Clears Status Bit of LEUART1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn leuart1unlock(&mut self) -> LEUART1UNLOCK_W {
        LEUART1UNLOCK_W::new(self)
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    pub fn lcdunlock(&mut self) -> LCDUNLOCK_W {
        LCDUNLOCK_W::new(self)
    }
    #[doc = "Bit 18 - Clears Status Bit of LETIMER1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn letimer1unlock(&mut self) -> LETIMER1UNLOCK_W {
        LETIMER1UNLOCK_W::new(self)
    }
    #[doc = "Bit 19 - Clears Status Bit of I2C2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn i2c2unlock(&mut self) -> I2C2UNLOCK_W {
        I2C2UNLOCK_W::new(self)
    }
    #[doc = "Bit 20 - Clears Status Bit of ADC1 and Unlocks Access to It"]
    #[inline(always)]
    pub fn adc1unlock(&mut self) -> ADC1UNLOCK_W {
        ADC1UNLOCK_W::new(self)
    }
    #[doc = "Bit 21 - Clears Status Bit of ACMP2 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp2unlock(&mut self) -> ACMP2UNLOCK_W {
        ACMP2UNLOCK_W::new(self)
    }
    #[doc = "Bit 22 - Clears Status Bit of ACMP3 and Unlocks Access to It"]
    #[inline(always)]
    pub fn acmp3unlock(&mut self) -> ACMP3UNLOCK_W {
        ACMP3UNLOCK_W::new(self)
    }
    #[doc = "Bit 23 - Clears Status Bit of RTC and Unlocks Access to It"]
    #[inline(always)]
    pub fn rtcunlock(&mut self) -> RTCUNLOCK_W {
        RTCUNLOCK_W::new(self)
    }
    #[doc = "Bit 24 - Clears Status Bit of USB and Unlocks Access to It"]
    #[inline(always)]
    pub fn usbunlock(&mut self) -> USBUNLOCK_W {
        USBUNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em23pernoretaincmd](index.html) module"]
pub struct EM23PERNORETAINCMD_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [em23pernoretaincmd::W](W) writer structure"]
impl crate::Writable for EM23PERNORETAINCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM23PERNORETAINCMD to value 0"]
impl crate::Resettable for EM23PERNORETAINCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
