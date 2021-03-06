#[doc = "Register `REQCLEAR` writer"]
pub struct W(crate::W<REQCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQCLEAR_SPEC>;
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
impl From<crate::W<REQCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQCLEAR` writer - DMA Request Clear"]
pub type REQCLEAR_W<'a> = crate::FieldWriter<'a, u32, REQCLEAR_SPEC, u16, u16, 12, 0>;
impl W {
    #[doc = "Bits 0:11 - DMA Request Clear"]
    #[inline(always)]
    pub fn reqclear(&mut self) -> REQCLEAR_W {
        REQCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Channel Request Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqclear](index.html) module"]
pub struct REQCLEAR_SPEC;
impl crate::RegisterSpec for REQCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [reqclear::W](W) writer structure"]
impl crate::Writable for REQCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REQCLEAR to value 0"]
impl crate::Resettable for REQCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
