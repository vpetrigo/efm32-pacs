#[doc = "Register `TSUMSBSECCMP` reader"]
pub struct R(crate::R<TSUMSBSECCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUMSBSECCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUMSBSECCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUMSBSECCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUMSBSECCMP` writer"]
pub struct W(crate::W<TSUMSBSECCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUMSBSECCMP_SPEC>;
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
impl From<crate::W<TSUMSBSECCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUMSBSECCMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPVAL` reader - TSU timer comparison value (s)"]
pub type COMPVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPVAL` writer - TSU timer comparison value (s)"]
pub type COMPVAL_W<'a> = crate::FieldWriter<'a, u32, TSUMSBSECCMP_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&self) -> COMPVAL_R {
        COMPVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer comparison value (s)"]
    #[inline(always)]
    pub fn compval(&mut self) -> COMPVAL_W {
        COMPVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSU timer comparison value seconds \\[47:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsumsbseccmp](index.html) module"]
pub struct TSUMSBSECCMP_SPEC;
impl crate::RegisterSpec for TSUMSBSECCMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsumsbseccmp::R](R) reader structure"]
impl crate::Readable for TSUMSBSECCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsumsbseccmp::W](W) writer structure"]
impl crate::Writable for TSUMSBSECCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUMSBSECCMP to value 0"]
impl crate::Resettable for TSUMSBSECCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
