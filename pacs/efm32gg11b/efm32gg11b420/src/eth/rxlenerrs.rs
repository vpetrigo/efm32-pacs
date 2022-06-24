#[doc = "Register `RXLENERRS` reader"]
pub struct R(crate::R<RXLENERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLENERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLENERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLENERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXLENERRS` writer"]
pub struct W(crate::W<RXLENERRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXLENERRS_SPEC>;
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
impl From<crate::W<RXLENERRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXLENERRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Length field frame errors"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Length field frame errors"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, RXLENERRS_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Length field frame errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Length field frame errors"]
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
#[doc = "Length Field Frame Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlenerrs](index.html) module"]
pub struct RXLENERRS_SPEC;
impl crate::RegisterSpec for RXLENERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlenerrs::R](R) reader structure"]
impl crate::Readable for RXLENERRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxlenerrs::W](W) writer structure"]
impl crate::Writable for RXLENERRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXLENERRS to value 0"]
impl crate::Resettable for RXLENERRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
