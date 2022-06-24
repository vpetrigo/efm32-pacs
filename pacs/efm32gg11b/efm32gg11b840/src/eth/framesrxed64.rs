#[doc = "Register `FRAMESRXED64` reader"]
pub struct R(crate::R<FRAMESRXED64_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRAMESRXED64_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRAMESRXED64_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRAMESRXED64_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRAMESRXED64` writer"]
pub struct W(crate::W<FRAMESRXED64_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRAMESRXED64_SPEC>;
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
impl From<crate::W<FRAMESRXED64_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRAMESRXED64_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - 64 byte frames received without error"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - 64 byte frames received without error"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, FRAMESRXED64_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - 64 byte frames received without error"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 64 byte frames received without error"]
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
#[doc = "64 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framesrxed64](index.html) module"]
pub struct FRAMESRXED64_SPEC;
impl crate::RegisterSpec for FRAMESRXED64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [framesrxed64::R](R) reader structure"]
impl crate::Readable for FRAMESRXED64_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [framesrxed64::W](W) writer structure"]
impl crate::Writable for FRAMESRXED64_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRAMESRXED64 to value 0"]
impl crate::Resettable for FRAMESRXED64_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
