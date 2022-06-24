#[doc = "Register `DEFERREDFRAMES` reader"]
pub struct R(crate::R<DEFERREDFRAMES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEFERREDFRAMES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEFERREDFRAMES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEFERREDFRAMES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEFERREDFRAMES` writer"]
pub struct W(crate::W<DEFERREDFRAMES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEFERREDFRAMES_SPEC>;
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
impl From<crate::W<DEFERREDFRAMES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEFERREDFRAMES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Deferred transmission frames"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Deferred transmission frames"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, DEFERREDFRAMES_SPEC, u32, u32, 18, 0>;
impl R {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Deferred transmission frames"]
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
#[doc = "Deferred Transmission Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deferredframes](index.html) module"]
pub struct DEFERREDFRAMES_SPEC;
impl crate::RegisterSpec for DEFERREDFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [deferredframes::R](R) reader structure"]
impl crate::Readable for DEFERREDFRAMES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [deferredframes::W](W) writer structure"]
impl crate::Writable for DEFERREDFRAMES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEFERREDFRAMES to value 0"]
impl crate::Resettable for DEFERREDFRAMES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
