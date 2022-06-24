#[doc = "Register `ECCCTRL` reader"]
pub struct R(crate::R<ECCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECCCTRL` writer"]
pub struct W(crate::W<ECCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECCCTRL_SPEC>;
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
impl From<crate::W<ECCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMECCEWEN` reader - RAM ECC Write Enable"]
pub type RAMECCEWEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMECCEWEN` writer - RAM ECC Write Enable"]
pub type RAMECCEWEN_W<'a> = crate::BitWriter<'a, u32, ECCCTRL_SPEC, bool, 0>;
#[doc = "Field `RAMECCCHKEN` reader - RAM ECC Check Enable"]
pub type RAMECCCHKEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMECCCHKEN` writer - RAM ECC Check Enable"]
pub type RAMECCCHKEN_W<'a> = crate::BitWriter<'a, u32, ECCCTRL_SPEC, bool, 1>;
#[doc = "Field `RAM1ECCEWEN` reader - RAM1 ECC Write Enable"]
pub type RAM1ECCEWEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM1ECCEWEN` writer - RAM1 ECC Write Enable"]
pub type RAM1ECCEWEN_W<'a> = crate::BitWriter<'a, u32, ECCCTRL_SPEC, bool, 2>;
#[doc = "Field `RAM1ECCCHKEN` reader - RAM1 ECC Check Enable"]
pub type RAM1ECCCHKEN_R = crate::BitReader<bool>;
#[doc = "Field `RAM1ECCCHKEN` writer - RAM1 ECC Check Enable"]
pub type RAM1ECCCHKEN_W<'a> = crate::BitWriter<'a, u32, ECCCTRL_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RAMECCEWEN_R {
        RAMECCEWEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&self) -> RAMECCCHKEN_R {
        RAMECCCHKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&self) -> RAM1ECCEWEN_R {
        RAM1ECCEWEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&self) -> RAM1ECCCHKEN_R {
        RAM1ECCCHKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&mut self) -> RAMECCEWEN_W {
        RAMECCEWEN_W::new(self)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&mut self) -> RAMECCCHKEN_W {
        RAMECCCHKEN_W::new(self)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&mut self) -> RAM1ECCEWEN_W {
        RAM1ECCEWEN_W::new(self)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&mut self) -> RAM1ECCCHKEN_W {
        RAM1ECCCHKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAM ECC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccctrl](index.html) module"]
pub struct ECCCTRL_SPEC;
impl crate::RegisterSpec for ECCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eccctrl::R](R) reader structure"]
impl crate::Readable for ECCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eccctrl::W](W) writer structure"]
impl crate::Writable for ECCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECCCTRL to value 0"]
impl crate::Resettable for ECCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
