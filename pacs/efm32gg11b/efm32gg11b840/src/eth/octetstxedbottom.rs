#[doc = "Register `OCTETSTXEDBOTTOM` reader"]
pub struct R(crate::R<OCTETSTXEDBOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTETSTXEDBOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTETSTXEDBOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTETSTXEDBOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTETSTXEDBOTTOM` writer"]
pub struct W(crate::W<OCTETSTXEDBOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTETSTXEDBOTTOM_SPEC>;
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
impl From<crate::W<OCTETSTXEDBOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTETSTXEDBOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Transmitted octets in frame without errors \\[31:0\\]"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Transmitted octets in frame without errors \\[31:0\\]"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, OCTETSTXEDBOTTOM_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmitted octets in frame without errors \\[31:0\\]"]
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
#[doc = "Octets transmitted 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octetstxedbottom](index.html) module"]
pub struct OCTETSTXEDBOTTOM_SPEC;
impl crate::RegisterSpec for OCTETSTXEDBOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octetstxedbottom::R](R) reader structure"]
impl crate::Readable for OCTETSTXEDBOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octetstxedbottom::W](W) writer structure"]
impl crate::Writable for OCTETSTXEDBOTTOM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCTETSTXEDBOTTOM to value 0"]
impl crate::Resettable for OCTETSTXEDBOTTOM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
