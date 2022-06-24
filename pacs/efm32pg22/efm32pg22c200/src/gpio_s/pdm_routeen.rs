#[doc = "Register `PDM_ROUTEEN` reader"]
pub struct R(crate::R<PDM_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_ROUTEEN` writer"]
pub struct W(crate::W<PDM_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_ROUTEEN_SPEC>;
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
impl From<crate::W<PDM_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKPEN` reader - CLK pin enable control bit"]
pub type CLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPEN` writer - CLK pin enable control bit"]
pub type CLKPEN_W<'a> = crate::BitWriter<'a, u32, PDM_ROUTEEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - CLK pin enable control bit"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLK pin enable control bit"]
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
#[doc = "PDM pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_routeen](index.html) module"]
pub struct PDM_ROUTEEN_SPEC;
impl crate::RegisterSpec for PDM_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_routeen::R](R) reader structure"]
impl crate::Readable for PDM_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_routeen::W](W) writer structure"]
impl crate::Writable for PDM_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_ROUTEEN to value 0"]
impl crate::Resettable for PDM_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
