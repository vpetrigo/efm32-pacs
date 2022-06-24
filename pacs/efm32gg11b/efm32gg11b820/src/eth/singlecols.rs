#[doc = "Register `SINGLECOLS` reader"]
pub struct R(crate::R<SINGLECOLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGLECOLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGLECOLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGLECOLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SINGLECOLS` writer"]
pub struct W(crate::W<SINGLECOLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGLECOLS_SPEC>;
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
impl From<crate::W<SINGLECOLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGLECOLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Single collision frames"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Single collision frames"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, SINGLECOLS_SPEC, u32, u32, 18, 0>;
impl R {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Single Collision Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singlecols](index.html) module"]
pub struct SINGLECOLS_SPEC;
impl crate::RegisterSpec for SINGLECOLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singlecols::R](R) reader structure"]
impl crate::Readable for SINGLECOLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singlecols::W](W) writer structure"]
impl crate::Writable for SINGLECOLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SINGLECOLS to value 0"]
impl crate::Resettable for SINGLECOLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
