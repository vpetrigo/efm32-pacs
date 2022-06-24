#[doc = "Register `GUSBCFG` reader"]
pub struct R(crate::R<GUSBCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GUSBCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GUSBCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GUSBCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GUSBCFG` writer"]
pub struct W(crate::W<GUSBCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GUSBCFG_SPEC>;
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
impl From<crate::W<GUSBCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GUSBCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUTCAL` reader - Timeout Calibration"]
pub type TOUTCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOUTCAL` writer - Timeout Calibration"]
pub type TOUTCAL_W<'a> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 3, 0>;
#[doc = "Field `FSINTF` reader - Full-Speed Serial Interface Select"]
pub type FSINTF_R = crate::BitReader<bool>;
#[doc = "Field `FSINTF` writer - Full-Speed Serial Interface Select"]
pub type FSINTF_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 5>;
#[doc = "Field `USBTRDTIM` reader - USB Turnaround Time"]
pub type USBTRDTIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBTRDTIM` writer - USB Turnaround Time"]
pub type USBTRDTIM_W<'a> = crate::FieldWriter<'a, u32, GUSBCFG_SPEC, u8, u8, 4, 10>;
#[doc = "Field `TERMSELDLPULSE` reader - TermSel DLine Pulsing Selection"]
pub type TERMSELDLPULSE_R = crate::BitReader<bool>;
#[doc = "Field `TERMSELDLPULSE` writer - TermSel DLine Pulsing Selection"]
pub type TERMSELDLPULSE_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 22>;
#[doc = "Field `TXENDDELAY` reader - Tx End Delay"]
pub type TXENDDELAY_R = crate::BitReader<bool>;
#[doc = "Field `TXENDDELAY` writer - Tx End Delay"]
pub type TXENDDELAY_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 28>;
#[doc = "Field `CORRUPTTXPKT` writer - Corrupt Tx packet"]
pub type CORRUPTTXPKT_W<'a> = crate::BitWriter<'a, u32, GUSBCFG_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:2 - Timeout Calibration"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    pub fn fsintf(&self) -> FSINTF_R {
        FSINTF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&self) -> TERMSELDLPULSE_R {
        TERMSELDLPULSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&self) -> TXENDDELAY_R {
        TXENDDELAY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timeout Calibration"]
    #[inline(always)]
    pub fn toutcal(&mut self) -> TOUTCAL_W {
        TOUTCAL_W::new(self)
    }
    #[doc = "Bit 5 - Full-Speed Serial Interface Select"]
    #[inline(always)]
    pub fn fsintf(&mut self) -> FSINTF_W {
        FSINTF_W::new(self)
    }
    #[doc = "Bits 10:13 - USB Turnaround Time"]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W {
        USBTRDTIM_W::new(self)
    }
    #[doc = "Bit 22 - TermSel DLine Pulsing Selection"]
    #[inline(always)]
    pub fn termseldlpulse(&mut self) -> TERMSELDLPULSE_W {
        TERMSELDLPULSE_W::new(self)
    }
    #[doc = "Bit 28 - Tx End Delay"]
    #[inline(always)]
    pub fn txenddelay(&mut self) -> TXENDDELAY_W {
        TXENDDELAY_W::new(self)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn corrupttxpkt(&mut self) -> CORRUPTTXPKT_W {
        CORRUPTTXPKT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](index.html) module"]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gusbcfg::R](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x1440"]
impl crate::Resettable for GUSBCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1440
    }
}
