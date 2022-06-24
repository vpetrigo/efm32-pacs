#[doc = "Register `RXUDPCKERRS` reader"]
pub struct R(crate::R<RXUDPCKERRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXUDPCKERRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXUDPCKERRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXUDPCKERRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXUDPCKERRS` writer"]
pub struct W(crate::W<RXUDPCKERRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXUDPCKERRS_SPEC>;
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
impl From<crate::W<RXUDPCKERRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXUDPCKERRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - UDP checksum errors"]
pub type COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNT` writer - UDP checksum errors"]
pub type COUNT_W<'a> = crate::FieldWriter<'a, u32, RXUDPCKERRS_SPEC, u8, u8, 8, 0>;
impl R {
    #[doc = "Bits 0:7 - UDP checksum errors"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UDP checksum errors"]
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
#[doc = "UDP Checksum Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxudpckerrs](index.html) module"]
pub struct RXUDPCKERRS_SPEC;
impl crate::RegisterSpec for RXUDPCKERRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxudpckerrs::R](R) reader structure"]
impl crate::Readable for RXUDPCKERRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxudpckerrs::W](W) writer structure"]
impl crate::Writable for RXUDPCKERRS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXUDPCKERRS to value 0"]
impl crate::Resettable for RXUDPCKERRS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
