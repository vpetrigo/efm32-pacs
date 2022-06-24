#[doc = "Register `IENC` writer"]
pub struct W(crate::W<IENC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IENC_SPEC>;
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
impl From<crate::W<IENC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IENC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNGMNTDONE` writer - Disable management done interrupt"]
pub type MNGMNTDONE_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 0>;
#[doc = "Field `RXCMPLT` writer - Disable receive complete interrupt"]
pub type RXCMPLT_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 1>;
#[doc = "Field `RXUSEDBITREAD` writer - Disable receive used bit read interrupt"]
pub type RXUSEDBITREAD_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 2>;
#[doc = "Field `TXUSEDBITREAD` writer - Disable transmit used bit read interrupt"]
pub type TXUSEDBITREAD_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 3>;
#[doc = "Field `TXUNDERRUN` writer - Disable transmit buffer under run interrupt"]
pub type TXUNDERRUN_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 4>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Disable retry limit exceeded or late collision interrupt"]
pub type RTRYLMTORLATECOL_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 5>;
#[doc = "Field `AMBAERR` writer - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
pub type AMBAERR_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 6>;
#[doc = "Field `TXCMPLT` writer - Disable transmit complete interrupt"]
pub type TXCMPLT_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 7>;
#[doc = "Field `RXOVERRUN` writer - Disable receive overrun interrupt"]
pub type RXOVERRUN_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 10>;
#[doc = "Field `RESPNOTOK` writer - Disable bresp/hresp not OK interrupt"]
pub type RESPNOTOK_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 11>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Disable pause frame with non-zero pause quantum interrupt"]
pub type NONZEROPFRMQUANT_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 12>;
#[doc = "Field `PAUSETIMEZERO` writer - Disable pause time zero interrupt"]
pub type PAUSETIMEZERO_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 13>;
#[doc = "Field `PFRMTX` writer - Disable pause frame transmitted interrupt"]
pub type PFRMTX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 14>;
#[doc = "Field `PTPDLYREQFRMRX` writer - Disable PTP delay_req frame received interrupt"]
pub type PTPDLYREQFRMRX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 18>;
#[doc = "Field `PTPSYNCFRMRX` writer - Disable PTP sync frame received interrupt"]
pub type PTPSYNCFRMRX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 19>;
#[doc = "Field `PTPDLYREQFRMTX` writer - Disable PTP delay_req frame transmitted interrupt"]
pub type PTPDLYREQFRMTX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 20>;
#[doc = "Field `PTPSYNCFRMTX` writer - Disable PTP sync frame transmitted interrupt"]
pub type PTPSYNCFRMTX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 21>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - Disable PTP pdelay_req frame received interrupt"]
pub type PTPPDLYREQFRMRX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 22>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - Disable PTP pdelay_resp frame received interrupt"]
pub type PTPPDLYRESPFRMRX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 23>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - Disable PTP pdelay_req frame transmitted interrupt"]
pub type PTPPDLYREQFRMTX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 24>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - Disable PTP pdelay_resp frame transmitted interrupt"]
pub type PTPPDLYRESPFRMTX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 25>;
#[doc = "Field `TSUSECREGINCR` writer - Disable TSU seconds register increment interrupt"]
pub type TSUSECREGINCR_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 26>;
#[doc = "Field `RXLPIINDC` writer - Disable RX LPI indication interrupt"]
pub type RXLPIINDC_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 27>;
#[doc = "Field `WOLEVNTRX` writer - Disable WOL event received interrupt"]
pub type WOLEVNTRX_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 28>;
#[doc = "Field `TSUTIMERCOMP` writer - Disable TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_W<'a> = crate::BitWriter<'a, u32, IENC_SPEC, bool, 29>;
impl W {
    #[doc = "Bit 0 - Disable management done interrupt"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W {
        MNGMNTDONE_W::new(self)
    }
    #[doc = "Bit 1 - Disable receive complete interrupt"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W {
        RXCMPLT_W::new(self)
    }
    #[doc = "Bit 2 - Disable receive used bit read interrupt"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W {
        RXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 3 - Disable transmit used bit read interrupt"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W {
        TXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 4 - Disable transmit buffer under run interrupt"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Disable retry limit exceeded or late collision interrupt"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W {
        RTRYLMTORLATECOL_W::new(self)
    }
    #[doc = "Bit 6 - Disable transmit frame corruption due to AMBA (AHB) error interrupt"]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 7 - Disable transmit complete interrupt"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 10 - Disable receive overrun interrupt"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 11 - Disable bresp/hresp not OK interrupt"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Bit 12 - Disable pause frame with non-zero pause quantum interrupt"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W {
        NONZEROPFRMQUANT_W::new(self)
    }
    #[doc = "Bit 13 - Disable pause time zero interrupt"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W {
        PAUSETIMEZERO_W::new(self)
    }
    #[doc = "Bit 14 - Disable pause frame transmitted interrupt"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PFRMTX_W {
        PFRMTX_W::new(self)
    }
    #[doc = "Bit 18 - Disable PTP delay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W {
        PTPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 19 - Disable PTP sync frame received interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W {
        PTPSYNCFRMRX_W::new(self)
    }
    #[doc = "Bit 20 - Disable PTP delay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W {
        PTPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 21 - Disable PTP sync frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W {
        PTPSYNCFRMTX_W::new(self)
    }
    #[doc = "Bit 22 - Disable PTP pdelay_req frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W {
        PTPPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 23 - Disable PTP pdelay_resp frame received interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W {
        PTPPDLYRESPFRMRX_W::new(self)
    }
    #[doc = "Bit 24 - Disable PTP pdelay_req frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W {
        PTPPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 25 - Disable PTP pdelay_resp frame transmitted interrupt"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W {
        PTPPDLYRESPFRMTX_W::new(self)
    }
    #[doc = "Bit 26 - Disable TSU seconds register increment interrupt"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W {
        TSUSECREGINCR_W::new(self)
    }
    #[doc = "Bit 27 - Disable RX LPI indication interrupt"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W {
        RXLPIINDC_W::new(self)
    }
    #[doc = "Bit 28 - Disable WOL event received interrupt"]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W {
        WOLEVNTRX_W::new(self)
    }
    #[doc = "Bit 29 - Disable TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&mut self) -> TSUTIMERCOMP_W {
        TSUTIMERCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienc](index.html) module"]
pub struct IENC_SPEC;
impl crate::RegisterSpec for IENC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ienc::W](W) writer structure"]
impl crate::Writable for IENC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IENC to value 0"]
impl crate::Resettable for IENC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
