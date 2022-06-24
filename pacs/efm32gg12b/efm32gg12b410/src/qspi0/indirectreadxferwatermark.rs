#[doc = "Register `INDIRECTREADXFERWATERMARK` reader"]
pub struct R(crate::R<INDIRECTREADXFERWATERMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTREADXFERWATERMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTREADXFERWATERMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTREADXFERWATERMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTREADXFERWATERMARK` writer"]
pub struct W(crate::W<INDIRECTREADXFERWATERMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTREADXFERWATERMARK_SPEC>;
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
impl From<crate::W<INDIRECTREADXFERWATERMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTREADXFERWATERMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - Watermark Value"]
pub type LEVEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LEVEL` writer - Watermark Value"]
pub type LEVEL_W<'a> = crate::FieldWriter<'a, u32, INDIRECTREADXFERWATERMARK_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watermark Value"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Read Transfer Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectreadxferwatermark](index.html) module"]
pub struct INDIRECTREADXFERWATERMARK_SPEC;
impl crate::RegisterSpec for INDIRECTREADXFERWATERMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectreadxferwatermark::R](R) reader structure"]
impl crate::Readable for INDIRECTREADXFERWATERMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectreadxferwatermark::W](W) writer structure"]
impl crate::Writable for INDIRECTREADXFERWATERMARK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTREADXFERWATERMARK to value 0"]
impl crate::Resettable for INDIRECTREADXFERWATERMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
