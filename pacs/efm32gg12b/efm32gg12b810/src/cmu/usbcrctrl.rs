#[doc = "Register `USBCRCTRL` reader"]
pub struct R(crate::R<USBCRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCRCTRL` writer"]
pub struct W(crate::W<USBCRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCRCTRL_SPEC>;
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
impl From<crate::W<USBCRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBCREN` reader - Clock Recovery Enable"]
pub type USBCREN_R = crate::BitReader<bool>;
#[doc = "Field `USBCREN` writer - Clock Recovery Enable"]
pub type USBCREN_W<'a> = crate::BitWriter<'a, u32, USBCRCTRL_SPEC, bool, 0>;
#[doc = "Field `USBLSCRMD` reader - Low Speed Clock Recovery Mode"]
pub type USBLSCRMD_R = crate::BitReader<bool>;
#[doc = "Field `USBLSCRMD` writer - Low Speed Clock Recovery Mode"]
pub type USBLSCRMD_W<'a> = crate::BitWriter<'a, u32, USBCRCTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&self) -> USBCREN_R {
        USBCREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&self) -> USBLSCRMD_R {
        USBLSCRMD_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Recovery Enable"]
    #[inline(always)]
    pub fn usbcren(&mut self) -> USBCREN_W {
        USBCREN_W::new(self)
    }
    #[doc = "Bit 1 - Low Speed Clock Recovery Mode"]
    #[inline(always)]
    pub fn usblscrmd(&mut self) -> USBLSCRMD_W {
        USBLSCRMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Recovery Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcrctrl](index.html) module"]
pub struct USBCRCTRL_SPEC;
impl crate::RegisterSpec for USBCRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcrctrl::R](R) reader structure"]
impl crate::Readable for USBCRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcrctrl::W](W) writer structure"]
impl crate::Writable for USBCRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCRCTRL to value 0"]
impl crate::Resettable for USBCRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
