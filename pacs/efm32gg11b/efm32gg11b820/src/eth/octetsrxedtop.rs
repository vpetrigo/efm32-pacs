#[doc = "Register `OCTETSRXEDTOP` reader"]
pub struct R(crate::R<OCTETSRXEDTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OCTETSRXEDTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OCTETSRXEDTOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OCTETSRXEDTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OCTETSRXEDTOP` writer"]
pub struct W(crate::W<OCTETSRXEDTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OCTETSRXEDTOP_SPEC>;
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
impl From<crate::W<OCTETSRXEDTOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OCTETSRXEDTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Received octets in frame without errors"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Received octets in frame without errors"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, OCTETSRXEDTOP_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Received octets in frame without errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Received octets in frame without errors"]
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
#[doc = "Octets Received 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [octetsrxedtop](index.html) module"]
pub struct OCTETSRXEDTOP_SPEC;
impl crate::RegisterSpec for OCTETSRXEDTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [octetsrxedtop::R](R) reader structure"]
impl crate::Readable for OCTETSRXEDTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [octetsrxedtop::W](W) writer structure"]
impl crate::Writable for OCTETSRXEDTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OCTETSRXEDTOP to value 0"]
impl crate::Resettable for OCTETSRXEDTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
