#[doc = "Register `PUSHADDR` reader"]
pub struct R(crate::R<PUSHADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUSHADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUSHADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUSHADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUSHADDR` writer"]
pub struct W(crate::W<PUSHADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUSHADDR_SPEC>;
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
impl From<crate::W<PUSHADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUSHADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Start address of data block"]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Start address of data block"]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, PUSHADDR_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Start address of data block"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of data block"]
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
#[doc = "Pusher: Start address of data block (LSB). In direct mode, this register is written by the software. In scatter-gather mode, this register is updated after each processed descriptor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pushaddr](index.html) module"]
pub struct PUSHADDR_SPEC;
impl crate::RegisterSpec for PUSHADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pushaddr::R](R) reader structure"]
impl crate::Readable for PUSHADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pushaddr::W](W) writer structure"]
impl crate::Writable for PUSHADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUSHADDR to value 0"]
impl crate::Resettable for PUSHADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
