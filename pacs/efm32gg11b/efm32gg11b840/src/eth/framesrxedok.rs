#[doc = "Register `FRAMESRXEDOK` reader"]
pub struct R(crate::R<FRAMESRXEDOK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMESRXEDOK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMESRXEDOK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMESRXEDOK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMESRXEDOK` writer"]
pub struct W(crate::W<FRAMESRXEDOK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMESRXEDOK_SPEC>;
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
impl From<crate::W<FRAMESRXEDOK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMESRXEDOK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Frames received without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Frames received without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, FRAMESRXEDOK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Frames received without error"]
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
#[doc = "Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framesrxedok](index.html) module"]
pub struct FRAMESRXEDOK_SPEC;
impl crate::RegisterSpec for FRAMESRXEDOK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framesrxedok::R](R) reader structure"]
impl crate::Readable for FRAMESRXEDOK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framesrxedok::W](W) writer structure"]
impl crate::Writable for FRAMESRXEDOK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMESRXEDOK to value 0"]
impl crate::Resettable for FRAMESRXEDOK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
