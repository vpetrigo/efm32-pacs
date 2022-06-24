#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWCLKTCKPEN` reader - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWCLKTCKPEN` writer - Serial Wire Clock and JTAG Test Clock Pin Enable"]
pub type SWCLKTCKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `SWDIOTMSPEN` reader - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWDIOTMSPEN` writer - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
pub type SWDIOTMSPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `TDOPEN` reader - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_R = crate::BitReader<bool>;
#[doc = "Field `TDOPEN` writer - JTAG Test Debug Output Pin Enable"]
pub type TDOPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `TDIPEN` reader - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_R = crate::BitReader<bool>;
#[doc = "Field `TDIPEN` writer - JTAG Test Debug Input Pin Enable"]
pub type TDIPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `SWVPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWVPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWVPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 4>;
#[doc = "Field `ETMTCLKPEN` reader - ETM Trace Clock Pin Enable"]
pub type ETMTCLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMTCLKPEN` writer - ETM Trace Clock Pin Enable"]
pub type ETMTCLKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 16>;
#[doc = "Field `ETMTD0PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD0PEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMTD0PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 17>;
#[doc = "Field `ETMTD1PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD1PEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMTD1PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 18>;
#[doc = "Field `ETMTD2PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD2PEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMTD2PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 19>;
#[doc = "Field `ETMTD3PEN` reader - ETM Trace Data Pin Enable"]
pub type ETMTD3PEN_R = crate::BitReader<bool>;
#[doc = "Field `ETMTD3PEN` writer - ETM Trace Data Pin Enable"]
pub type ETMTD3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&self) -> SWCLKTCKPEN_R {
        SWCLKTCKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&self) -> SWDIOTMSPEN_R {
        SWDIOTMSPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&self) -> TDOPEN_R {
        TDOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&self) -> TDIPEN_R {
        TDIPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&self) -> SWVPEN_R {
        SWVPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn etmtclkpen(&self) -> ETMTCLKPEN_R {
        ETMTCLKPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd0pen(&self) -> ETMTD0PEN_R {
        ETMTD0PEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd1pen(&self) -> ETMTD1PEN_R {
        ETMTD1PEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd2pen(&self) -> ETMTD2PEN_R {
        ETMTD2PEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd3pen(&self) -> ETMTD3PEN_R {
        ETMTD3PEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock and JTAG Test Clock Pin Enable"]
    #[inline(always)]
    pub fn swclktckpen(&mut self) -> SWCLKTCKPEN_W {
        SWCLKTCKPEN_W::new(self)
    }
    #[doc = "Bit 1 - Serial Wire Data and JTAG Test Mode Select Pin Enable"]
    #[inline(always)]
    pub fn swdiotmspen(&mut self) -> SWDIOTMSPEN_W {
        SWDIOTMSPEN_W::new(self)
    }
    #[doc = "Bit 2 - JTAG Test Debug Output Pin Enable"]
    #[inline(always)]
    pub fn tdopen(&mut self) -> TDOPEN_W {
        TDOPEN_W::new(self)
    }
    #[doc = "Bit 3 - JTAG Test Debug Input Pin Enable"]
    #[inline(always)]
    pub fn tdipen(&mut self) -> TDIPEN_W {
        TDIPEN_W::new(self)
    }
    #[doc = "Bit 4 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swvpen(&mut self) -> SWVPEN_W {
        SWVPEN_W::new(self)
    }
    #[doc = "Bit 16 - ETM Trace Clock Pin Enable"]
    #[inline(always)]
    pub fn etmtclkpen(&mut self) -> ETMTCLKPEN_W {
        ETMTCLKPEN_W::new(self)
    }
    #[doc = "Bit 17 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd0pen(&mut self) -> ETMTD0PEN_W {
        ETMTD0PEN_W::new(self)
    }
    #[doc = "Bit 18 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd1pen(&mut self) -> ETMTD1PEN_W {
        ETMTD1PEN_W::new(self)
    }
    #[doc = "Bit 19 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd2pen(&mut self) -> ETMTD2PEN_W {
        ETMTD2PEN_W::new(self)
    }
    #[doc = "Bit 20 - ETM Trace Data Pin Enable"]
    #[inline(always)]
    pub fn etmtd3pen(&mut self) -> ETMTD3PEN_W {
        ETMTD3PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0x0f"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
