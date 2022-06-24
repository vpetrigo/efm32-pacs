#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GAIN` reader - Selects Gain factor of DCF"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Selects Gain factor of DCF"]
pub type GAIN_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, 0>;
#[doc = "Field `DSR` reader - Down sampling rate of Decimation filter"]
pub type DSR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DSR` writer - Down sampling rate of Decimation filter"]
pub type DSR_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u16, u16, 12, 8>;
#[doc = "Field `OUTCLKEN` reader - PDM Clock enable"]
pub type OUTCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTCLKEN` writer - PDM Clock enable"]
pub type OUTCLKEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - PDM Clock enable"]
    #[inline(always)]
    pub fn outclken(&self) -> OUTCLKEN_R {
        OUTCLKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects Gain factor of DCF"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W {
        GAIN_W::new(self)
    }
    #[doc = "Bits 8:19 - Down sampling rate of Decimation filter"]
    #[inline(always)]
    pub fn dsr(&mut self) -> DSR_W {
        DSR_W::new(self)
    }
    #[doc = "Bit 31 - PDM Clock enable"]
    #[inline(always)]
    pub fn outclken(&mut self) -> OUTCLKEN_W {
        OUTCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM Core Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
