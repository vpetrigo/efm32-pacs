#[doc = "Register `INDIRECTWRITEXFERWATERMARK` reader"]
pub struct R(crate::R<INDIRECTWRITEXFERWATERMARK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTWRITEXFERWATERMARK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTWRITEXFERWATERMARK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTWRITEXFERWATERMARK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTWRITEXFERWATERMARK` writer"]
pub struct W(crate::W<INDIRECTWRITEXFERWATERMARK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTWRITEXFERWATERMARK_SPEC>;
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
impl From<crate::W<INDIRECTWRITEXFERWATERMARK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTWRITEXFERWATERMARK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - Watermark Value"]
pub type LEVEL_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LEVEL` writer - Watermark Value"]
pub type LEVEL_W<'a> =
    crate::FieldWriter<'a, u32, INDIRECTWRITEXFERWATERMARK_SPEC, u32, u32, 32, 0>;
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
#[doc = "Indirect Write Transfer Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexferwatermark](index.html) module"]
pub struct INDIRECTWRITEXFERWATERMARK_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERWATERMARK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectwritexferwatermark::R](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERWATERMARK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectwritexferwatermark::W](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERWATERMARK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERWATERMARK to value 0xffff_ffff"]
impl crate::Resettable for INDIRECTWRITEXFERWATERMARK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
