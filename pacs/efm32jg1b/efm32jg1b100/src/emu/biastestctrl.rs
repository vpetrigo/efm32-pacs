#[doc = "Register `BIASTESTCTRL` reader"]
pub struct R(crate::R<BIASTESTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASTESTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASTESTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASTESTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASTESTCTRL` writer"]
pub struct W(crate::W<BIASTESTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASTESTCTRL_SPEC>;
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
impl From<crate::W<BIASTESTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASTESTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIAS_RIP_RESET` reader - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_R = crate::BitReader<bool>;
#[doc = "Field `BIAS_RIP_RESET` writer - Reset Bias Ripple Counter"]
pub type BIAS_RIP_RESET_W<'a> = crate::BitWriter<'a, u32, BIASTESTCTRL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&self) -> BIAS_RIP_RESET_R {
        BIAS_RIP_RESET_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Reset Bias Ripple Counter"]
    #[inline(always)]
    pub fn bias_rip_reset(&mut self) -> BIAS_RIP_RESET_W {
        BIAS_RIP_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Control Register for Regulator and BIAS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biastestctrl](index.html) module"]
pub struct BIASTESTCTRL_SPEC;
impl crate::RegisterSpec for BIASTESTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biastestctrl::R](R) reader structure"]
impl crate::Readable for BIASTESTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biastestctrl::W](W) writer structure"]
impl crate::Writable for BIASTESTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIASTESTCTRL to value 0"]
impl crate::Resettable for BIASTESTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
