#[doc = "Register `MASKADD1TOP` reader"]
pub struct R(crate::R<MASKADD1TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASKADD1TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASKADD1TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASKADD1TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASKADD1TOP` writer"]
pub struct W(crate::W<MASKADD1TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASKADD1TOP_SPEC>;
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
impl From<crate::W<MASKADD1TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASKADD1TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMASK` reader - Specific Address Mask"]
pub type ADDRMASK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRMASK` writer - Specific Address Mask"]
pub type ADDRMASK_W<'a> = crate::FieldWriter<'a, u32, MASKADD1TOP_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> ADDRMASK_W {
        ADDRMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address Mask 1 Top 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maskadd1top](index.html) module"]
pub struct MASKADD1TOP_SPEC;
impl crate::RegisterSpec for MASKADD1TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maskadd1top::R](R) reader structure"]
impl crate::Readable for MASKADD1TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maskadd1top::W](W) writer structure"]
impl crate::Writable for MASKADD1TOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASKADD1TOP to value 0"]
impl crate::Resettable for MASKADD1TOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
