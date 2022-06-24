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
#[doc = "Field `ITAPDLYEN` reader - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ITAPDLYEN_R = crate::BitReader<bool>;
#[doc = "Field `ITAPDLYEN` writer - Selective Tap Delay Line Enable on Rxclk_in"]
pub type ITAPDLYEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `ITAPDLYSEL` reader - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ITAPDLYSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITAPDLYSEL` writer - Selects One of 32 Taps on the Rxclk_in Line"]
pub type ITAPDLYSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 5, 1>;
#[doc = "Field `ITAPCHGWIN` reader - Gating Signal for Tap Delay Change"]
pub type ITAPCHGWIN_R = crate::BitReader<bool>;
#[doc = "Field `ITAPCHGWIN` writer - Gating Signal for Tap Delay Change"]
pub type ITAPCHGWIN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `OTAPDLYEN` reader - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OTAPDLYEN_R = crate::BitReader<bool>;
#[doc = "Field `OTAPDLYEN` writer - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
pub type OTAPDLYEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
#[doc = "Field `OTAPDLYSEL` reader - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OTAPDLYSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTAPDLYSEL` writer - Selects One of 32 Taps on the SDIO_CLK Pin"]
pub type OTAPDLYSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 8>;
#[doc = "Field `TXDLYMUXSEL` reader - TX Delay Mux Selection"]
pub type TXDLYMUXSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXDLYMUXSEL` writer - TX Delay Mux Selection"]
pub type TXDLYMUXSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&self) -> ITAPDLYEN_R {
        ITAPDLYEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&self) -> ITAPDLYSEL_R {
        ITAPDLYSEL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&self) -> ITAPCHGWIN_R {
        ITAPCHGWIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&self) -> OTAPDLYEN_R {
        OTAPDLYEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&self) -> OTAPDLYSEL_R {
        OTAPDLYSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&self) -> TXDLYMUXSEL_R {
        TXDLYMUXSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selective Tap Delay Line Enable on Rxclk_in"]
    #[inline(always)]
    pub fn itapdlyen(&mut self) -> ITAPDLYEN_W {
        ITAPDLYEN_W::new(self)
    }
    #[doc = "Bits 1:5 - Selects One of 32 Taps on the Rxclk_in Line"]
    #[inline(always)]
    pub fn itapdlysel(&mut self) -> ITAPDLYSEL_W {
        ITAPDLYSEL_W::new(self)
    }
    #[doc = "Bit 6 - Gating Signal for Tap Delay Change"]
    #[inline(always)]
    pub fn itapchgwin(&mut self) -> ITAPCHGWIN_W {
        ITAPCHGWIN_W::new(self)
    }
    #[doc = "Bit 7 - Selective Tap Delay Line Enable on SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlyen(&mut self) -> OTAPDLYEN_W {
        OTAPDLYEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Selects One of 32 Taps on the SDIO_CLK Pin"]
    #[inline(always)]
    pub fn otapdlysel(&mut self) -> OTAPDLYSEL_W {
        OTAPDLYSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - TX Delay Mux Selection"]
    #[inline(always)]
    pub fn txdlymuxsel(&mut self) -> TXDLYMUXSEL_W {
        TXDLYMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core Control Signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
