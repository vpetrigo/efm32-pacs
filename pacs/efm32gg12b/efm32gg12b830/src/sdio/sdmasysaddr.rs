#[doc = "Register `SDMASYSADDR` reader"]
pub struct R(crate::R<SDMASYSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDMASYSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDMASYSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDMASYSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDMASYSADDR` writer"]
pub struct W(crate::W<SDMASYSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDMASYSADDR_SPEC>;
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
impl From<crate::W<SDMASYSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDMASYSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDMASYSADDRARG` reader - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SDMASYSADDRARG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SDMASYSADDRARG` writer - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SDMASYSADDRARG_W<'a> = crate::FieldWriter<'a, u32, SDMASYSADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&self) -> SDMASYSADDRARG_R {
        SDMASYSADDRARG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&mut self) -> SDMASYSADDRARG_W {
        SDMASYSADDRARG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDMA System Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdmasysaddr](index.html) module"]
pub struct SDMASYSADDR_SPEC;
impl crate::RegisterSpec for SDMASYSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdmasysaddr::R](R) reader structure"]
impl crate::Readable for SDMASYSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdmasysaddr::W](W) writer structure"]
impl crate::Writable for SDMASYSADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDMASYSADDR to value 0"]
impl crate::Resettable for SDMASYSADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
