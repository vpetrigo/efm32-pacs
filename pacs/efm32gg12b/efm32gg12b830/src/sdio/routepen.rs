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
#[doc = "Field `CLKPEN` reader - CLK I/O Enable"]
pub type CLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPEN` writer - CLK I/O Enable"]
pub type CLKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `CMDPEN` reader - CMD I/O Enable"]
pub type CMDPEN_R = crate::BitReader<bool>;
#[doc = "Field `CMDPEN` writer - CMD I/O Enable"]
pub type CMDPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `D0PEN` reader - Dat0 I/O Enable"]
pub type D0PEN_R = crate::BitReader<bool>;
#[doc = "Field `D0PEN` writer - Dat0 I/O Enable"]
pub type D0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `D1PEN` reader - Dat1 I/O Enable"]
pub type D1PEN_R = crate::BitReader<bool>;
#[doc = "Field `D1PEN` writer - Dat1 I/O Enable"]
pub type D1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `D2PEN` reader - Dat2 I/O Enable"]
pub type D2PEN_R = crate::BitReader<bool>;
#[doc = "Field `D2PEN` writer - Dat2 I/O Enable"]
pub type D2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 4>;
#[doc = "Field `D3PEN` reader - Dat3 I/O Enable"]
pub type D3PEN_R = crate::BitReader<bool>;
#[doc = "Field `D3PEN` writer - Dat3 I/O Enable"]
pub type D3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 5>;
#[doc = "Field `D4PEN` reader - Dat4 I/O Enable"]
pub type D4PEN_R = crate::BitReader<bool>;
#[doc = "Field `D4PEN` writer - Dat4 I/O Enable"]
pub type D4PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 6>;
#[doc = "Field `D5PEN` reader - Dat5 Enable"]
pub type D5PEN_R = crate::BitReader<bool>;
#[doc = "Field `D5PEN` writer - Dat5 Enable"]
pub type D5PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 7>;
#[doc = "Field `D6PEN` reader - Dat6 Enable"]
pub type D6PEN_R = crate::BitReader<bool>;
#[doc = "Field `D6PEN` writer - Dat6 Enable"]
pub type D6PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 8>;
#[doc = "Field `D7PEN` reader - Data7 I/O Enable"]
pub type D7PEN_R = crate::BitReader<bool>;
#[doc = "Field `D7PEN` writer - Data7 I/O Enable"]
pub type D7PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 9>;
impl R {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&self) -> CMDPEN_R {
        CMDPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&self) -> D0PEN_R {
        D0PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&self) -> D1PEN_R {
        D1PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&self) -> D2PEN_R {
        D2PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&self) -> D3PEN_R {
        D3PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&self) -> D4PEN_R {
        D4PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&self) -> D5PEN_R {
        D5PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&self) -> D6PEN_R {
        D6PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&self) -> D7PEN_R {
        D7PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK I/O Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W::new(self)
    }
    #[doc = "Bit 1 - CMD I/O Enable"]
    #[inline(always)]
    pub fn cmdpen(&mut self) -> CMDPEN_W {
        CMDPEN_W::new(self)
    }
    #[doc = "Bit 2 - Dat0 I/O Enable"]
    #[inline(always)]
    pub fn d0pen(&mut self) -> D0PEN_W {
        D0PEN_W::new(self)
    }
    #[doc = "Bit 3 - Dat1 I/O Enable"]
    #[inline(always)]
    pub fn d1pen(&mut self) -> D1PEN_W {
        D1PEN_W::new(self)
    }
    #[doc = "Bit 4 - Dat2 I/O Enable"]
    #[inline(always)]
    pub fn d2pen(&mut self) -> D2PEN_W {
        D2PEN_W::new(self)
    }
    #[doc = "Bit 5 - Dat3 I/O Enable"]
    #[inline(always)]
    pub fn d3pen(&mut self) -> D3PEN_W {
        D3PEN_W::new(self)
    }
    #[doc = "Bit 6 - Dat4 I/O Enable"]
    #[inline(always)]
    pub fn d4pen(&mut self) -> D4PEN_W {
        D4PEN_W::new(self)
    }
    #[doc = "Bit 7 - Dat5 Enable"]
    #[inline(always)]
    pub fn d5pen(&mut self) -> D5PEN_W {
        D5PEN_W::new(self)
    }
    #[doc = "Bit 8 - Dat6 Enable"]
    #[inline(always)]
    pub fn d6pen(&mut self) -> D6PEN_W {
        D6PEN_W::new(self)
    }
    #[doc = "Bit 9 - Data7 I/O Enable"]
    #[inline(always)]
    pub fn d7pen(&mut self) -> D7PEN_W {
        D7PEN_W::new(self)
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
