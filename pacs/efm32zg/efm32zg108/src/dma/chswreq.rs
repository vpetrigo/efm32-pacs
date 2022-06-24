#[doc = "Register `CHSWREQ` writer"]
pub struct W(crate::W<CHSWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHSWREQ_SPEC>;
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
impl From<crate::W<CHSWREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHSWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0SWREQ` writer - Channel 0 Software Request"]
pub type CH0SWREQ_W<'a> = crate::BitWriter<'a, u32, CHSWREQ_SPEC, bool, 0>;
#[doc = "Field `CH1SWREQ` writer - Channel 1 Software Request"]
pub type CH1SWREQ_W<'a> = crate::BitWriter<'a, u32, CHSWREQ_SPEC, bool, 1>;
#[doc = "Field `CH2SWREQ` writer - Channel 2 Software Request"]
pub type CH2SWREQ_W<'a> = crate::BitWriter<'a, u32, CHSWREQ_SPEC, bool, 2>;
#[doc = "Field `CH3SWREQ` writer - Channel 3 Software Request"]
pub type CH3SWREQ_W<'a> = crate::BitWriter<'a, u32, CHSWREQ_SPEC, bool, 3>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Request"]
    #[inline(always)]
    pub fn ch0swreq(&mut self) -> CH0SWREQ_W {
        CH0SWREQ_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Request"]
    #[inline(always)]
    pub fn ch1swreq(&mut self) -> CH1SWREQ_W {
        CH1SWREQ_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Request"]
    #[inline(always)]
    pub fn ch2swreq(&mut self) -> CH2SWREQ_W {
        CH2SWREQ_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Request"]
    #[inline(always)]
    pub fn ch3swreq(&mut self) -> CH3SWREQ_W {
        CH3SWREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chswreq](index.html) module"]
pub struct CHSWREQ_SPEC;
impl crate::RegisterSpec for CHSWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chswreq::W](W) writer structure"]
impl crate::Writable for CHSWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHSWREQ to value 0"]
impl crate::Resettable for CHSWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
