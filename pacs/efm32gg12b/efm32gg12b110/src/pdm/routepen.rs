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
#[doc = "Field `DAT0PEN` reader - DAT0 I/O Enable"]
pub type DAT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `DAT0PEN` writer - DAT0 I/O Enable"]
pub type DAT0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `DAT1PEN` reader - DAT1 I/O Enable"]
pub type DAT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `DAT1PEN` writer - DAT1 I/O Enable"]
pub type DAT1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `DAT2PEN` reader - DAT2 I/O Enable"]
pub type DAT2PEN_R = crate::BitReader<bool>;
#[doc = "Field `DAT2PEN` writer - DAT2 I/O Enable"]
pub type DAT2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `DAT3PEN` reader - DAT3 I/O Enable"]
pub type DAT3PEN_R = crate::BitReader<bool>;
#[doc = "Field `DAT3PEN` writer - DAT3 I/O Enable"]
pub type DAT3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `CLKPEN` reader - CLK I/O Enable"]
pub type CLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPEN` writer - CLK I/O Enable"]
pub type CLKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - DAT0 I/O Enable"]
    #[inline(always)]
    pub fn dat0pen(&self) -> DAT0PEN_R {
        DAT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAT1 I/O Enable"]
    #[inline(always)]
    pub fn dat1pen(&self) -> DAT1PEN_R {
        DAT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAT2 I/O Enable"]
    #[inline(always)]
    pub fn dat2pen(&self) -> DAT2PEN_R {
        DAT2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAT3 I/O Enable"]
    #[inline(always)]
    pub fn dat3pen(&self) -> DAT3PEN_R {
        DAT3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAT0 I/O Enable"]
    #[inline(always)]
    pub fn dat0pen(&mut self) -> DAT0PEN_W {
        DAT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - DAT1 I/O Enable"]
    #[inline(always)]
    pub fn dat1pen(&mut self) -> DAT1PEN_W {
        DAT1PEN_W::new(self)
    }
    #[doc = "Bit 2 - DAT2 I/O Enable"]
    #[inline(always)]
    pub fn dat2pen(&mut self) -> DAT2PEN_W {
        DAT2PEN_W::new(self)
    }
    #[doc = "Bit 3 - DAT3 I/O Enable"]
    #[inline(always)]
    pub fn dat3pen(&mut self) -> DAT3PEN_W {
        DAT3PEN_W::new(self)
    }
    #[doc = "Bit 8 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O LOCATION Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
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
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
