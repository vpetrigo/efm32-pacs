#[doc = "Register `FCSERRS` reader"]
pub struct R(crate::R<FCSERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCSERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCSERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCSERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCSERRS` writer"]
pub struct W(crate::W<FCSERRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCSERRS_SPEC>;
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
impl From<crate::W<FCSERRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCSERRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Frame check sequence errors"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - Frame check sequence errors"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, FCSERRS_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - Frame check sequence errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Frame check sequence errors"]
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
#[doc = "Frame Check Sequence Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcserrs](index.html) module"]
pub struct FCSERRS_SPEC;
impl crate::RegisterSpec for FCSERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcserrs::R](R) reader structure"]
impl crate::Readable for FCSERRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcserrs::W](W) writer structure"]
impl crate::Writable for FCSERRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCSERRS to value 0"]
impl crate::Resettable for FCSERRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
