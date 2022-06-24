#[doc = "Register `IF_CLR` writer"]
pub struct W(crate::W<IF_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_CLR_SPEC>;
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
impl From<crate::W<IF_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHERENDOFBLOCK` writer - End of block interrupt flag clear"]
pub type FETCHERENDOFBLOCK_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 0>;
#[doc = "Field `FETCHERSTOPPED` writer - Stopped interrupt flag clear"]
pub type FETCHERSTOPPED_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 1>;
#[doc = "Field `FETCHERERROR` writer - Error interrupt flag clear"]
pub type FETCHERERROR_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 2>;
#[doc = "Field `PUSHERENDOFBLOCK` writer - End of block interrupt flag clear"]
pub type PUSHERENDOFBLOCK_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 3>;
#[doc = "Field `PUSHERSTOPPED` writer - Stopped interrupt flag clear"]
pub type PUSHERSTOPPED_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 4>;
#[doc = "Field `PUSHERERROR` writer - Error interrupt flag clear"]
pub type PUSHERERROR_W<'a> = crate::BitWriter<'a, u32, IF_CLR_SPEC, bool, 5>;
impl W {
    #[doc = "Bit 0 - End of block interrupt flag clear"]
    #[inline(always)]
    pub fn fetcherendofblock(&mut self) -> FETCHERENDOFBLOCK_W {
        FETCHERENDOFBLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Stopped interrupt flag clear"]
    #[inline(always)]
    pub fn fetcherstopped(&mut self) -> FETCHERSTOPPED_W {
        FETCHERSTOPPED_W::new(self)
    }
    #[doc = "Bit 2 - Error interrupt flag clear"]
    #[inline(always)]
    pub fn fetchererror(&mut self) -> FETCHERERROR_W {
        FETCHERERROR_W::new(self)
    }
    #[doc = "Bit 3 - End of block interrupt flag clear"]
    #[inline(always)]
    pub fn pusherendofblock(&mut self) -> PUSHERENDOFBLOCK_W {
        PUSHERENDOFBLOCK_W::new(self)
    }
    #[doc = "Bit 4 - Stopped interrupt flag clear"]
    #[inline(always)]
    pub fn pusherstopped(&mut self) -> PUSHERSTOPPED_W {
        PUSHERSTOPPED_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt flag clear"]
    #[inline(always)]
    pub fn pushererror(&mut self) -> PUSHERERROR_W {
        PUSHERERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Writing a '1' clears the interrupt status. Writing a '0' has no effect.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_clr](index.html) module"]
pub struct IF_CLR_SPEC;
impl crate::RegisterSpec for IF_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [if_clr::W](W) writer structure"]
impl crate::Writable for IF_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF_CLR to value 0"]
impl crate::Resettable for IF_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
