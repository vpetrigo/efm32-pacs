#[doc = "Register `INDAHBADDRTRIGGER` reader"]
pub struct R(crate::R<INDAHBADDRTRIGGER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INDAHBADDRTRIGGER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INDAHBADDRTRIGGER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INDAHBADDRTRIGGER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INDAHBADDRTRIGGER` writer"]
pub struct W(crate::W<INDAHBADDRTRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INDAHBADDRTRIGGER_SPEC>;
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
impl From<crate::W<INDAHBADDRTRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INDAHBADDRTRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Indirect Address Trigger Register"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Indirect Address Trigger Register"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, INDAHBADDRTRIGGER_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Indirect Address Trigger Register"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indirect Address Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [indahbaddrtrigger](index.html) module"]
pub struct INDAHBADDRTRIGGER_SPEC;
impl crate::RegisterSpec for INDAHBADDRTRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [indahbaddrtrigger::R](R) reader structure"]
impl crate::Readable for INDAHBADDRTRIGGER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [indahbaddrtrigger::W](W) writer structure"]
impl crate::Writable for INDAHBADDRTRIGGER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INDAHBADDRTRIGGER to value 0"]
impl crate::Resettable for INDAHBADDRTRIGGER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
