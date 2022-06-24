#[doc = "Register `RXRESOURCEERRS` reader"]
pub struct R(crate::R<RXRESOURCEERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXRESOURCEERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXRESOURCEERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXRESOURCEERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXRESOURCEERRS` writer"]
pub struct W(crate::W<RXRESOURCEERRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXRESOURCEERRS_SPEC>;
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
impl From<crate::W<RXRESOURCEERRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXRESOURCEERRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Receive resource errors"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COUNT` writer - Receive resource errors"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, RXRESOURCEERRS_SPEC, u32, u32, 18, 0>;
impl R {
    #[doc = "Bits 0:17 - Receive resource errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Receive resource errors"]
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
#[doc = "Receive Resource Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxresourceerrs](index.html) module"]
pub struct RXRESOURCEERRS_SPEC;
impl crate::RegisterSpec for RXRESOURCEERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxresourceerrs::R](R) reader structure"]
impl crate::Readable for RXRESOURCEERRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxresourceerrs::W](W) writer structure"]
impl crate::Writable for RXRESOURCEERRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXRESOURCEERRS to value 0"]
impl crate::Resettable for RXRESOURCEERRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
