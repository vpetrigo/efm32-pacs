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
#[doc = "Field `SINGLESTART` writer - Single Queue Start"]
pub type SINGLESTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `SINGLESTOP` writer - Single Queue Stop"]
pub type SINGLESTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `SCANSTART` writer - Scan Queue Start"]
pub type SCANSTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `SCANSTOP` writer - Scan Queue Stop"]
pub type SCANSTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
#[doc = "Field `TIMEREN` writer - Timer Enable"]
pub type TIMEREN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 16>;
#[doc = "Field `TIMERDIS` writer - Timer Disable"]
pub type TIMERDIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 17>;
#[doc = "Field `SINGLEFIFOFLUSH` writer - Flush the Single FIFO"]
pub type SINGLEFIFOFLUSH_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 24>;
#[doc = "Field `SCANFIFOFLUSH` writer - Flush the Scan FIFO"]
pub type SCANFIFOFLUSH_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 25>;
impl W {
    #[doc = "Bit 0 - Single Queue Start"]
    #[inline(always)]
    pub fn singlestart(&mut self) -> SINGLESTART_W {
        SINGLESTART_W::new(self)
    }
    #[doc = "Bit 1 - Single Queue Stop"]
    #[inline(always)]
    pub fn singlestop(&mut self) -> SINGLESTOP_W {
        SINGLESTOP_W::new(self)
    }
    #[doc = "Bit 3 - Scan Queue Start"]
    #[inline(always)]
    pub fn scanstart(&mut self) -> SCANSTART_W {
        SCANSTART_W::new(self)
    }
    #[doc = "Bit 4 - Scan Queue Stop"]
    #[inline(always)]
    pub fn scanstop(&mut self) -> SCANSTOP_W {
        SCANSTOP_W::new(self)
    }
    #[doc = "Bit 16 - Timer Enable"]
    #[inline(always)]
    pub fn timeren(&mut self) -> TIMEREN_W {
        TIMEREN_W::new(self)
    }
    #[doc = "Bit 17 - Timer Disable"]
    #[inline(always)]
    pub fn timerdis(&mut self) -> TIMERDIS_W {
        TIMERDIS_W::new(self)
    }
    #[doc = "Bit 24 - Flush the Single FIFO"]
    #[inline(always)]
    pub fn singlefifoflush(&mut self) -> SINGLEFIFOFLUSH_W {
        SINGLEFIFOFLUSH_W::new(self)
    }
    #[doc = "Bit 25 - Flush the Scan FIFO"]
    #[inline(always)]
    pub fn scanfifoflush(&mut self) -> SCANFIFOFLUSH_W {
        SCANFIFOFLUSH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
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
