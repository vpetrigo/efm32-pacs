#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` reader"]
pub struct R(crate::R<INDIRECTWRITEXFERNUMBYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTWRITEXFERNUMBYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTWRITEXFERNUMBYTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTWRITEXFERNUMBYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTWRITEXFERNUMBYTES` writer"]
pub struct W(crate::W<INDIRECTWRITEXFERNUMBYTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTWRITEXFERNUMBYTES_SPEC>;
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
impl From<crate::W<INDIRECTWRITEXFERNUMBYTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTWRITEXFERNUMBYTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - Indirect Number of Bytes"]
pub type VALUE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VALUE` writer - Indirect Number of Bytes"]
pub type VALUE_W<'a> = crate::FieldWriter<'a, u32, INDIRECTWRITEXFERNUMBYTES_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Number of Bytes"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Write Transfer Number Bytes Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirectwritexfernumbytes](index.html) module"]
pub struct INDIRECTWRITEXFERNUMBYTES_SPEC;
impl crate::RegisterSpec for INDIRECTWRITEXFERNUMBYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirectwritexfernumbytes::R](R) reader structure"]
impl crate::Readable for INDIRECTWRITEXFERNUMBYTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirectwritexfernumbytes::W](W) writer structure"]
impl crate::Writable for INDIRECTWRITEXFERNUMBYTES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERNUMBYTES to value 0"]
impl crate::Resettable for INDIRECTWRITEXFERNUMBYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
