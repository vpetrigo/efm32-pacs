#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4UNLATCH` writer - EM4 unlatch"]
pub type EM4UNLATCH_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `TEMPAVGREQ` writer - Temperature Average Request"]
pub type TEMPAVGREQ_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
#[doc = "Field `EM01VSCALE1` writer - Scale voltage to Vscale1"]
pub type EM01VSCALE1_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 10>;
#[doc = "Field `EM01VSCALE2` writer - Scale voltage to Vscale2"]
pub type EM01VSCALE2_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 11>;
#[doc = "Field `RSTCAUSECLR` writer - Reset Cause Clear"]
pub type RSTCAUSECLR_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 17>;
impl W {
    #[doc = "Bit 1 - EM4 unlatch"]
    #[inline(always)]
    pub fn em4unlatch(&mut self) -> EM4UNLATCH_W {
        EM4UNLATCH_W::new(self)
    }
    #[doc = "Bit 4 - Temperature Average Request"]
    #[inline(always)]
    pub fn tempavgreq(&mut self) -> TEMPAVGREQ_W {
        TEMPAVGREQ_W::new(self)
    }
    #[doc = "Bit 10 - Scale voltage to Vscale1"]
    #[inline(always)]
    pub fn em01vscale1(&mut self) -> EM01VSCALE1_W {
        EM01VSCALE1_W::new(self)
    }
    #[doc = "Bit 11 - Scale voltage to Vscale2"]
    #[inline(always)]
    pub fn em01vscale2(&mut self) -> EM01VSCALE2_W {
        EM01VSCALE2_W::new(self)
    }
    #[doc = "Bit 17 - Reset Cause Clear"]
    #[inline(always)]
    pub fn rstcauseclr(&mut self) -> RSTCAUSECLR_W {
        RSTCAUSECLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
