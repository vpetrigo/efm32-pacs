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
#[doc = "Field `RAMWSEN` reader - RAM WAIT STATE Enable"]
pub type RAMWSEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMWSEN` writer - RAM WAIT STATE Enable"]
pub type RAMWSEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 1>;
#[doc = "Field `RAMPREFETCHEN` reader - RAM Prefetch Enable"]
pub type RAMPREFETCHEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMPREFETCHEN` writer - RAM Prefetch Enable"]
pub type RAMPREFETCHEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 2>;
#[doc = "Field `RAM1WSEN` reader - RAM1 WAIT STATE Enable"]
pub type RAM1WSEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM1WSEN` writer - RAM1 WAIT STATE Enable"]
pub type RAM1WSEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 9>;
#[doc = "Field `RAM1PREFETCHEN` reader - RAM1 Prefetch Enable"]
pub type RAM1PREFETCHEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM1PREFETCHEN` writer - RAM1 Prefetch Enable"]
pub type RAM1PREFETCHEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 10>;
#[doc = "Field `RAM2CACHEEN` reader - RAM2 CACHE Enable"]
pub type RAM2CACHEEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM2CACHEEN` writer - RAM2 CACHE Enable"]
pub type RAM2CACHEEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 16>;
#[doc = "Field `RAM2WSEN` reader - RAM2 WAIT STATE Enable"]
pub type RAM2WSEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM2WSEN` writer - RAM2 WAIT STATE Enable"]
pub type RAM2WSEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 17>;
#[doc = "Field `RAM2PREFETCHEN` reader - RAM2 Prefetch Enable"]
pub type RAM2PREFETCHEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM2PREFETCHEN` writer - RAM2 Prefetch Enable"]
pub type RAM2PREFETCHEN_W<'a> = crate::BitWriter<'a, u32, RAMCTRL_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&self) -> RAMWSEN_R {
        RAMWSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&self) -> RAMPREFETCHEN_R {
        RAMPREFETCHEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&self) -> RAM1WSEN_R {
        RAM1WSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&self) -> RAM1PREFETCHEN_R {
        RAM1PREFETCHEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&self) -> RAM2CACHEEN_R {
        RAM2CACHEEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&self) -> RAM2WSEN_R {
        RAM2WSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&self) -> RAM2PREFETCHEN_R {
        RAM2PREFETCHEN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - RAM WAIT STATE Enable"]
    #[inline(always)]
    pub fn ramwsen(&mut self) -> RAMWSEN_W {
        RAMWSEN_W::new(self)
    }
    #[doc = "Bit 2 - RAM Prefetch Enable"]
    #[inline(always)]
    pub fn ramprefetchen(&mut self) -> RAMPREFETCHEN_W {
        RAMPREFETCHEN_W::new(self)
    }
    #[doc = "Bit 9 - RAM1 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram1wsen(&mut self) -> RAM1WSEN_W {
        RAM1WSEN_W::new(self)
    }
    #[doc = "Bit 10 - RAM1 Prefetch Enable"]
    #[inline(always)]
    pub fn ram1prefetchen(&mut self) -> RAM1PREFETCHEN_W {
        RAM1PREFETCHEN_W::new(self)
    }
    #[doc = "Bit 16 - RAM2 CACHE Enable"]
    #[inline(always)]
    pub fn ram2cacheen(&mut self) -> RAM2CACHEEN_W {
        RAM2CACHEEN_W::new(self)
    }
    #[doc = "Bit 17 - RAM2 WAIT STATE Enable"]
    #[inline(always)]
    pub fn ram2wsen(&mut self) -> RAM2WSEN_W {
        RAM2WSEN_W::new(self)
    }
    #[doc = "Bit 18 - RAM2 Prefetch Enable"]
    #[inline(always)]
    pub fn ram2prefetchen(&mut self) -> RAM2PREFETCHEN_W {
        RAM2PREFETCHEN_W::new(self)
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
