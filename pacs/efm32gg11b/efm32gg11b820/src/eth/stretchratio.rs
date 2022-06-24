#[doc = "Register `STRETCHRATIO` reader"]
pub struct R(crate::R<STRETCHRATIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STRETCHRATIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STRETCHRATIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STRETCHRATIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STRETCHRATIO` writer"]
pub struct W(crate::W<STRETCHRATIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STRETCHRATIO_SPEC>;
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
impl From<crate::W<STRETCHRATIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STRETCHRATIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPGSTRETCH` reader - IPG Stretch"]
pub type IPGSTRETCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IPGSTRETCH` writer - IPG Stretch"]
pub type IPGSTRETCH_W<'a> = crate::FieldWriter<'a, u32, STRETCHRATIO_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&self) -> IPGSTRETCH_R {
        IPGSTRETCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&mut self) -> IPGSTRETCH_W {
        IPGSTRETCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPG stretch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stretchratio](index.html) module"]
pub struct STRETCHRATIO_SPEC;
impl crate::RegisterSpec for STRETCHRATIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stretchratio::R](R) reader structure"]
impl crate::Readable for STRETCHRATIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stretchratio::W](W) writer structure"]
impl crate::Writable for STRETCHRATIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STRETCHRATIO to value 0"]
impl crate::Resettable for STRETCHRATIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
