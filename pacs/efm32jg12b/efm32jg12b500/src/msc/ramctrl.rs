#[doc = "Register `RAMCTRL` reader"]
pub struct R(crate::R<RAMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAMCTRL` writer"]
pub struct W(crate::W<RAMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAMCTRL_SPEC>;
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
impl From<crate::W<RAMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMCACHEEN` reader - RAM CACHE Enable"]
pub type RAMCACHEEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMCACHEEN` writer - RAM CACHE Enable"]
pub type RAMCACHEEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 0>;
#[doc = "Field `RAM1CACHEEN` reader - RAM1 CACHE Enable"]
pub type RAM1CACHEEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM1CACHEEN` writer - RAM1 CACHE Enable"]
pub type RAM1CACHEEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - RAM CACHE Enable"]
    #[inline(always)]
    pub fn ramcacheen(&self) -> RAMCACHEEN_R {
        RAMCACHEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - RAM1 CACHE Enable"]
    #[inline(always)]
    pub fn ram1cacheen(&self) -> RAM1CACHEEN_R {
        RAM1CACHEEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM CACHE Enable"]
    #[inline(always)]
    pub fn ramcacheen(&mut self) -> RAMCACHEEN_W {
        RAMCACHEEN_W::new(self)
    }
    #[doc = "Bit 8 - RAM1 CACHE Enable"]
    #[inline(always)]
    pub fn ram1cacheen(&mut self) -> RAM1CACHEEN_W {
        RAM1CACHEEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM Control Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramctrl](index.html) module"]
pub struct RAMCTRL_SPEC;
impl crate::RegisterSpec for RAMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ramctrl::R](R) reader structure"]
impl crate::Readable for RAMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ramctrl::W](W) writer structure"]
impl crate::Writable for RAMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RAMCTRL to value 0"]
impl crate::Resettable for RAMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
