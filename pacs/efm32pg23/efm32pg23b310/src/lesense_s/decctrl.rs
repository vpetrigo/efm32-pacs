#[doc = "Register `DECCTRL` reader"]
pub struct R(crate::R<DECCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DECCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DECCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DECCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DECCTRL` writer"]
pub struct W(crate::W<DECCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DECCTRL_SPEC>;
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
impl From<crate::W<DECCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DECCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECDIS` reader - Disable the decoder"]
pub type DECDIS_R = crate::BitReader<bool>;
#[doc = "Field `DECDIS` writer - Disable the decoder"]
pub type DECDIS_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 0>;
#[doc = "Field `INTMAP` reader - Enable decoder to channel interrupt map"]
pub type INTMAP_R = crate::BitReader<bool>;
#[doc = "Field `INTMAP` writer - Enable decoder to channel interrupt map"]
pub type INTMAP_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 2>;
#[doc = "Field `HYSTPRS0` reader - Enable decoder hysteresis on PRS0 output"]
pub type HYSTPRS0_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS0` writer - Enable decoder hysteresis on PRS0 output"]
pub type HYSTPRS0_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 3>;
#[doc = "Field `HYSTPRS1` reader - Enable decoder hysteresis on PRS1 output"]
pub type HYSTPRS1_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS1` writer - Enable decoder hysteresis on PRS1 output"]
pub type HYSTPRS1_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 4>;
#[doc = "Field `HYSTPRS2` reader - Enable decoder hysteresis on PRS2 output"]
pub type HYSTPRS2_R = crate::BitReader<bool>;
#[doc = "Field `HYSTPRS2` writer - Enable decoder hysteresis on PRS2 output"]
pub type HYSTPRS2_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 5>;
#[doc = "Field `HYSTIRQ` reader - Enable decoder hysteresis on interrupt r"]
pub type HYSTIRQ_R = crate::BitReader<bool>;
#[doc = "Field `HYSTIRQ` writer - Enable decoder hysteresis on interrupt r"]
pub type HYSTIRQ_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 6>;
#[doc = "Field `PRSCNT` reader - Enable count mode on decoder PRS channel"]
pub type PRSCNT_R = crate::BitReader<bool>;
#[doc = "Field `PRSCNT` writer - Enable count mode on decoder PRS channel"]
pub type PRSCNT_W<'a> = crate::BitWriter<'a, u32, DECCTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    pub fn decdis(&self) -> DECDIS_R {
        DECDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt map"]
    #[inline(always)]
    pub fn intmap(&self) -> INTMAP_R {
        INTMAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    pub fn hystprs0(&self) -> HYSTPRS0_R {
        HYSTPRS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    pub fn hystprs1(&self) -> HYSTPRS1_R {
        HYSTPRS1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    pub fn hystprs2(&self) -> HYSTPRS2_R {
        HYSTPRS2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt r"]
    #[inline(always)]
    pub fn hystirq(&self) -> HYSTIRQ_R {
        HYSTIRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channel"]
    #[inline(always)]
    pub fn prscnt(&self) -> PRSCNT_R {
        PRSCNT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the decoder"]
    #[inline(always)]
    pub fn decdis(&mut self) -> DECDIS_W {
        DECDIS_W::new(self)
    }
    #[doc = "Bit 2 - Enable decoder to channel interrupt map"]
    #[inline(always)]
    pub fn intmap(&mut self) -> INTMAP_W {
        INTMAP_W::new(self)
    }
    #[doc = "Bit 3 - Enable decoder hysteresis on PRS0 output"]
    #[inline(always)]
    pub fn hystprs0(&mut self) -> HYSTPRS0_W {
        HYSTPRS0_W::new(self)
    }
    #[doc = "Bit 4 - Enable decoder hysteresis on PRS1 output"]
    #[inline(always)]
    pub fn hystprs1(&mut self) -> HYSTPRS1_W {
        HYSTPRS1_W::new(self)
    }
    #[doc = "Bit 5 - Enable decoder hysteresis on PRS2 output"]
    #[inline(always)]
    pub fn hystprs2(&mut self) -> HYSTPRS2_W {
        HYSTPRS2_W::new(self)
    }
    #[doc = "Bit 6 - Enable decoder hysteresis on interrupt r"]
    #[inline(always)]
    pub fn hystirq(&mut self) -> HYSTIRQ_W {
        HYSTIRQ_W::new(self)
    }
    #[doc = "Bit 7 - Enable count mode on decoder PRS channel"]
    #[inline(always)]
    pub fn prscnt(&mut self) -> PRSCNT_W {
        PRSCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Decoder control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [decctrl](index.html) module"]
pub struct DECCTRL_SPEC;
impl crate::RegisterSpec for DECCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [decctrl::R](R) reader structure"]
impl crate::Readable for DECCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [decctrl::W](W) writer structure"]
impl crate::Writable for DECCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DECCTRL to value 0"]
impl crate::Resettable for DECCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
