#[doc = "Register `FRAMESTXEDOK` reader"]
pub struct R(crate::R<FRAMESTXEDOK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMESTXEDOK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMESTXEDOK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMESTXEDOK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMESTXEDOK` writer"]
pub struct W(crate::W<FRAMESTXEDOK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMESTXEDOK_SPEC>;
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
impl From<crate::W<FRAMESTXEDOK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMESTXEDOK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Frames transmitted without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Frames transmitted without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, FRAMESTXEDOK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames transmitted without error"]
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
#[doc = "Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framestxedok](index.html) module"]
pub struct FRAMESTXEDOK_SPEC;
impl crate::RegisterSpec for FRAMESTXEDOK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framestxedok::R](R) reader structure"]
impl crate::Readable for FRAMESTXEDOK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framestxedok::W](W) writer structure"]
impl crate::Writable for FRAMESTXEDOK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMESTXEDOK to value 0"]
impl crate::Resettable for FRAMESTXEDOK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
