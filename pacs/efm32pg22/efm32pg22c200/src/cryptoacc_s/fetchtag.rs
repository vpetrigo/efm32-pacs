#[doc = "Register `FETCHTAG` reader"]
pub struct R(crate::R<FETCHTAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHTAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHTAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHTAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHTAG` writer"]
pub struct W(crate::W<FETCHTAG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHTAG_SPEC>;
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
impl From<crate::W<FETCHTAG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHTAG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAG` reader - User tag"]
pub type TAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TAG` writer - User tag"]
pub type TAG_W<'a> = crate::FieldWriter<'a, u32, FETCHTAG_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - User tag"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User tag"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fetcher: User tag. In direct mode, this register is written by the software. In scatter-gather mode, this register is not used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchtag](index.html) module"]
pub struct FETCHTAG_SPEC;
impl crate::RegisterSpec for FETCHTAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchtag::R](R) reader structure"]
impl crate::Readable for FETCHTAG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchtag::W](W) writer structure"]
impl crate::Writable for FETCHTAG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FETCHTAG to value 0"]
impl crate::Resettable for FETCHTAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
