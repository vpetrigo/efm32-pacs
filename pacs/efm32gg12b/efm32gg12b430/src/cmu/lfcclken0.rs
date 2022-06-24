#[doc = "Register `LFCCLKEN0` reader"]
pub struct R(crate::R<LFCCLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFCCLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFCCLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFCCLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFCCLKEN0` writer"]
pub struct W(crate::W<LFCCLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFCCLKEN0_SPEC>;
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
impl From<crate::W<LFCCLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFCCLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB` reader - Universal Serial Bus Interface Clock Enable"]
pub type USB_R = crate::BitReader<bool>;
#[doc = "Field `USB` writer - Universal Serial Bus Interface Clock Enable"]
pub type USB_W<'a> = crate::BitWriter<'a, u32, LFCCLKEN0_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Serial Bus Interface Clock Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency C Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfcclken0](index.html) module"]
pub struct LFCCLKEN0_SPEC;
impl crate::RegisterSpec for LFCCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfcclken0::R](R) reader structure"]
impl crate::Readable for LFCCLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfcclken0::W](W) writer structure"]
impl crate::Writable for LFCCLKEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFCCLKEN0 to value 0"]
impl crate::Resettable for LFCCLKEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
