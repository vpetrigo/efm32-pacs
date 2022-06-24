#[doc = "Register `MULTICOLS` reader"]
pub struct R(crate::R<MULTICOLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MULTICOLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MULTICOLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MULTICOLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MULTICOLS` writer"]
pub struct W(crate::W<MULTICOLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MULTICOLS_SPEC>;
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
impl From<crate::W<MULTICOLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MULTICOLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Multiple collision frames"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Multiple collision frames"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, MULTICOLS_SPEC, u32, u32, 18, 0>;
impl R {
    #[doc = "Bits 0:17 - Multiple collision frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Multiple collision frames"]
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
#[doc = "Multiple Collision Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [multicols](index.html) module"]
pub struct MULTICOLS_SPEC;
impl crate::RegisterSpec for MULTICOLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [multicols::R](R) reader structure"]
impl crate::Readable for MULTICOLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [multicols::W](W) writer structure"]
impl crate::Writable for MULTICOLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MULTICOLS to value 0"]
impl crate::Resettable for MULTICOLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
