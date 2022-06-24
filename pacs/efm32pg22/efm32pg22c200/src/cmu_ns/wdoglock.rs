#[doc = "Register `WDOGLOCK` writer"]
pub struct W(crate::W<WDOGLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDOGLOCK_SPEC>;
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
impl From<crate::W<WDOGLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDOGLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configuration Lock Key\n\nValue on reset: 21079"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum LOCKKEY_AW {
    #[doc = "37879: Write this value to unlock"]
    UNLOCK = 37879,
}
impl From<LOCKKEY_AW> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCKKEY` writer - Configuration Lock Key"]
pub type LOCKKEY_W<'a> = crate::FieldWriter<'a, u32, WDOGLOCK_SPEC, u16, LOCKKEY_AW, 16, 0>;
impl<'a> LOCKKEY_W<'a> {
    #[doc = "Write this value to unlock"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(LOCKKEY_AW::UNLOCK)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LOCKKEY_W {
        LOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdoglock](index.html) module"]
pub struct WDOGLOCK_SPEC;
impl crate::RegisterSpec for WDOGLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wdoglock::W](W) writer structure"]
impl crate::Writable for WDOGLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDOGLOCK to value 0x5257"]
impl crate::Resettable for WDOGLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5257
    }
}
