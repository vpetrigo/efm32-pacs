#[doc = "Register `INDIRECTTRIGGERADDRRANGE` reader"]
pub struct R(crate::R<INDIRECTTRIGGERADDRRANGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDIRECTTRIGGERADDRRANGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDIRECTTRIGGERADDRRANGE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDIRECTTRIGGERADDRRANGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDIRECTTRIGGERADDRRANGE` writer"]
pub struct W(crate::W<INDIRECTTRIGGERADDRRANGE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDIRECTTRIGGERADDRRANGE_SPEC>;
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
impl From<crate::W<INDIRECTTRIGGERADDRRANGE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDIRECTTRIGGERADDRRANGE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDRANGEWIDTH` reader - Indirect Trigger Address Width"]
pub type INDRANGEWIDTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDRANGEWIDTH` writer - Indirect Trigger Address Width"]
pub type INDRANGEWIDTH_W<'a> =
    crate::FieldWriter<'a, u32, INDIRECTTRIGGERADDRRANGE_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&self) -> INDRANGEWIDTH_R {
        INDRANGEWIDTH_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indirect Trigger Address Width"]
    #[inline(always)]
    pub fn indrangewidth(&mut self) -> INDRANGEWIDTH_W {
        INDRANGEWIDTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Trigger Address Range Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indirecttriggeraddrrange](index.html) module"]
pub struct INDIRECTTRIGGERADDRRANGE_SPEC;
impl crate::RegisterSpec for INDIRECTTRIGGERADDRRANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indirecttriggeraddrrange::R](R) reader structure"]
impl crate::Readable for INDIRECTTRIGGERADDRRANGE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indirecttriggeraddrrange::W](W) writer structure"]
impl crate::Writable for INDIRECTTRIGGERADDRRANGE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDIRECTTRIGGERADDRRANGE to value 0x04"]
impl crate::Resettable for INDIRECTTRIGGERADDRRANGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
