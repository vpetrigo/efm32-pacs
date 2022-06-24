#[doc = "Register `IFCR` reader"]
pub struct R(crate::R<IFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFCR` writer"]
pub struct W(crate::W<IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFCR_SPEC>;
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
impl From<crate::W<IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MNGMNTDONE` reader - Management frame sent"]
pub type MNGMNTDONE_R = crate::BitReader<bool>;
#[doc = "Field `MNGMNTDONE` writer - Management frame sent"]
pub type MNGMNTDONE_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 0>;
#[doc = "Field `RXCMPLT` reader - Receive complete"]
pub type RXCMPLT_R = crate::BitReader<bool>;
#[doc = "Field `RXCMPLT` writer - Receive complete"]
pub type RXCMPLT_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 1>;
#[doc = "Field `RXUSEDBITREAD` reader - RX used bit read"]
pub type RXUSEDBITREAD_R = crate::BitReader<bool>;
#[doc = "Field `RXUSEDBITREAD` writer - RX used bit read"]
pub type RXUSEDBITREAD_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 2>;
#[doc = "Field `TXUSEDBITREAD` reader - TX used bit read"]
pub type TXUSEDBITREAD_R = crate::BitReader<bool>;
#[doc = "Field `TXUSEDBITREAD` writer - TX used bit read"]
pub type TXUSEDBITREAD_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 3>;
#[doc = "Field `TXUNDERRUN` reader - Transmit under run"]
pub type TXUNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRUN` writer - Transmit under run"]
pub type TXUNDERRUN_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 4>;
#[doc = "Field `RTRYLMTORLATECOL` reader - Retry limit exceeded or late collision"]
pub type RTRYLMTORLATECOL_R = crate::BitReader<bool>;
#[doc = "Field `RTRYLMTORLATECOL` writer - Retry limit exceeded or late collision"]
pub type RTRYLMTORLATECOL_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 5>;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) error."]
pub type AMBAERR_R = crate::BitReader<bool>;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) error."]
pub type AMBAERR_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 6>;
#[doc = "Field `TXCMPLT` reader - Transmit complete"]
pub type TXCMPLT_R = crate::BitReader<bool>;
#[doc = "Field `TXCMPLT` writer - Transmit complete"]
pub type TXCMPLT_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 7>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun"]
pub type RXOVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRUN` writer - Receive overrun"]
pub type RXOVERRUN_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 10>;
#[doc = "Field `RESPNOTOK` reader - Hresp not OK"]
pub type RESPNOTOK_R = crate::BitReader<bool>;
#[doc = "Field `RESPNOTOK` writer - Hresp not OK"]
pub type RESPNOTOK_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 11>;
#[doc = "Field `NONZEROPFRMQUANT` reader - Pause frame with non-zero pause quantum received"]
pub type NONZEROPFRMQUANT_R = crate::BitReader<bool>;
#[doc = "Field `NONZEROPFRMQUANT` writer - Pause frame with non-zero pause quantum received"]
pub type NONZEROPFRMQUANT_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 12>;
#[doc = "Field `PAUSETIMEZERO` reader - Pause Time zero"]
pub type PAUSETIMEZERO_R = crate::BitReader<bool>;
#[doc = "Field `PAUSETIMEZERO` writer - Pause Time zero"]
pub type PAUSETIMEZERO_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 13>;
#[doc = "Field `PFRMTX` reader - Pause frame transmitted"]
pub type PFRMTX_R = crate::BitReader<bool>;
#[doc = "Field `PFRMTX` writer - Pause frame transmitted"]
pub type PFRMTX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 14>;
#[doc = "Field `PTPDLYREQFRMRX` reader - PTP delay_req frame received"]
pub type PTPDLYREQFRMRX_R = crate::BitReader<bool>;
#[doc = "Field `PTPDLYREQFRMRX` writer - PTP delay_req frame received"]
pub type PTPDLYREQFRMRX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 18>;
#[doc = "Field `PTPSYNCFRMRX` reader - PTP sync frame received"]
pub type PTPSYNCFRMRX_R = crate::BitReader<bool>;
#[doc = "Field `PTPSYNCFRMRX` writer - PTP sync frame received"]
pub type PTPSYNCFRMRX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 19>;
#[doc = "Field `PTPDLYREQFRMTX` reader - PTP delay_req frame transmitted"]
pub type PTPDLYREQFRMTX_R = crate::BitReader<bool>;
#[doc = "Field `PTPDLYREQFRMTX` writer - PTP delay_req frame transmitted"]
pub type PTPDLYREQFRMTX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 20>;
#[doc = "Field `PTPSYNCFRMTX` reader - PTP sync frame transmitted"]
pub type PTPSYNCFRMTX_R = crate::BitReader<bool>;
#[doc = "Field `PTPSYNCFRMTX` writer - PTP sync frame transmitted"]
pub type PTPSYNCFRMTX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 21>;
#[doc = "Field `PTPPDLYREQFRMRX` reader - PTP pdelay_req frame received"]
pub type PTPPDLYREQFRMRX_R = crate::BitReader<bool>;
#[doc = "Field `PTPPDLYREQFRMRX` writer - PTP pdelay_req frame received"]
pub type PTPPDLYREQFRMRX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 22>;
#[doc = "Field `PTPPDLYRESPFRMRX` reader - PTP pdelay_resp frame received"]
pub type PTPPDLYRESPFRMRX_R = crate::BitReader<bool>;
#[doc = "Field `PTPPDLYRESPFRMRX` writer - PTP pdelay_resp frame received"]
pub type PTPPDLYRESPFRMRX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 23>;
#[doc = "Field `PTPPDLYREQFRMTX` reader - PTP pdelay_req frame transmitted"]
pub type PTPPDLYREQFRMTX_R = crate::BitReader<bool>;
#[doc = "Field `PTPPDLYREQFRMTX` writer - PTP pdelay_req frame transmitted"]
pub type PTPPDLYREQFRMTX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 24>;
#[doc = "Field `PTPPDLYRESPFRMTX` reader - PTP pdelay_resp frame transmitted"]
pub type PTPPDLYRESPFRMTX_R = crate::BitReader<bool>;
#[doc = "Field `PTPPDLYRESPFRMTX` writer - PTP pdelay_resp frame transmitted"]
pub type PTPPDLYRESPFRMTX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 25>;
#[doc = "Field `TSUSECREGINCR` reader - TSU seconds register increment"]
pub type TSUSECREGINCR_R = crate::BitReader<bool>;
#[doc = "Field `TSUSECREGINCR` writer - TSU seconds register increment"]
pub type TSUSECREGINCR_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 26>;
#[doc = "Field `RXLPIINDC` reader - Receive LPI indication status bit change"]
pub type RXLPIINDC_R = crate::BitReader<bool>;
#[doc = "Field `RXLPIINDC` writer - Receive LPI indication status bit change"]
pub type RXLPIINDC_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 27>;
#[doc = "Field `WOLEVNTRX` reader - WOL event received interrupt."]
pub type WOLEVNTRX_R = crate::BitReader<bool>;
#[doc = "Field `WOLEVNTRX` writer - WOL event received interrupt."]
pub type WOLEVNTRX_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 28>;
#[doc = "Field `TSUTIMERCOMP` reader - TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_R = crate::BitReader<bool>;
#[doc = "Field `TSUTIMERCOMP` writer - TSU timer comparison interrupt."]
pub type TSUTIMERCOMP_W<'a> = crate::BitWriter<'a, u32, IFCR_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&self) -> MNGMNTDONE_R {
        MNGMNTDONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&self) -> RXCMPLT_R {
        RXCMPLT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&self) -> RXUSEDBITREAD_R {
        RXUSEDBITREAD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&self) -> TXUSEDBITREAD_R {
        TXUSEDBITREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&self) -> RTRYLMTORLATECOL_R {
        RTRYLMTORLATECOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AMBAERR_R {
        AMBAERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TXCMPLT_R {
        TXCMPLT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&self) -> NONZEROPFRMQUANT_R {
        NONZEROPFRMQUANT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&self) -> PAUSETIMEZERO_R {
        PAUSETIMEZERO_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&self) -> PFRMTX_R {
        PFRMTX_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&self) -> PTPDLYREQFRMRX_R {
        PTPDLYREQFRMRX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&self) -> PTPSYNCFRMRX_R {
        PTPSYNCFRMRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&self) -> PTPDLYREQFRMTX_R {
        PTPDLYREQFRMTX_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&self) -> PTPSYNCFRMTX_R {
        PTPSYNCFRMTX_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&self) -> PTPPDLYREQFRMRX_R {
        PTPPDLYREQFRMRX_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&self) -> PTPPDLYRESPFRMRX_R {
        PTPPDLYRESPFRMRX_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&self) -> PTPPDLYREQFRMTX_R {
        PTPPDLYREQFRMTX_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&self) -> PTPPDLYRESPFRMTX_R {
        PTPPDLYRESPFRMTX_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&self) -> TSUSECREGINCR_R {
        TSUSECREGINCR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&self) -> RXLPIINDC_R {
        RXLPIINDC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&self) -> WOLEVNTRX_R {
        WOLEVNTRX_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
    #[inline(always)]
    pub fn tsutimercomp(&self) -> TSUTIMERCOMP_R {
        TSUTIMERCOMP_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Management frame sent"]
    #[inline(always)]
    pub fn mngmntdone(&mut self) -> MNGMNTDONE_W {
        MNGMNTDONE_W::new(self)
    }
    #[doc = "Bit 1 - Receive complete"]
    #[inline(always)]
    pub fn rxcmplt(&mut self) -> RXCMPLT_W {
        RXCMPLT_W::new(self)
    }
    #[doc = "Bit 2 - RX used bit read"]
    #[inline(always)]
    pub fn rxusedbitread(&mut self) -> RXUSEDBITREAD_W {
        RXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 3 - TX used bit read"]
    #[inline(always)]
    pub fn txusedbitread(&mut self) -> TXUSEDBITREAD_W {
        TXUSEDBITREAD_W::new(self)
    }
    #[doc = "Bit 4 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 5 - Retry limit exceeded or late collision"]
    #[inline(always)]
    pub fn rtrylmtorlatecol(&mut self) -> RTRYLMTORLATECOL_W {
        RTRYLMTORLATECOL_W::new(self)
    }
    #[doc = "Bit 6 - Transmit frame corruption due to AMBA (AHB) error."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 7 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 10 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 11 - Hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Bit 12 - Pause frame with non-zero pause quantum received"]
    #[inline(always)]
    pub fn nonzeropfrmquant(&mut self) -> NONZEROPFRMQUANT_W {
        NONZEROPFRMQUANT_W::new(self)
    }
    #[doc = "Bit 13 - Pause Time zero"]
    #[inline(always)]
    pub fn pausetimezero(&mut self) -> PAUSETIMEZERO_W {
        PAUSETIMEZERO_W::new(self)
    }
    #[doc = "Bit 14 - Pause frame transmitted"]
    #[inline(always)]
    pub fn pfrmtx(&mut self) -> PFRMTX_W {
        PFRMTX_W::new(self)
    }
    #[doc = "Bit 18 - PTP delay_req frame received"]
    #[inline(always)]
    pub fn ptpdlyreqfrmrx(&mut self) -> PTPDLYREQFRMRX_W {
        PTPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 19 - PTP sync frame received"]
    #[inline(always)]
    pub fn ptpsyncfrmrx(&mut self) -> PTPSYNCFRMRX_W {
        PTPSYNCFRMRX_W::new(self)
    }
    #[doc = "Bit 20 - PTP delay_req frame transmitted"]
    #[inline(always)]
    pub fn ptpdlyreqfrmtx(&mut self) -> PTPDLYREQFRMTX_W {
        PTPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 21 - PTP sync frame transmitted"]
    #[inline(always)]
    pub fn ptpsyncfrmtx(&mut self) -> PTPSYNCFRMTX_W {
        PTPSYNCFRMTX_W::new(self)
    }
    #[doc = "Bit 22 - PTP pdelay_req frame received"]
    #[inline(always)]
    pub fn ptppdlyreqfrmrx(&mut self) -> PTPPDLYREQFRMRX_W {
        PTPPDLYREQFRMRX_W::new(self)
    }
    #[doc = "Bit 23 - PTP pdelay_resp frame received"]
    #[inline(always)]
    pub fn ptppdlyrespfrmrx(&mut self) -> PTPPDLYRESPFRMRX_W {
        PTPPDLYRESPFRMRX_W::new(self)
    }
    #[doc = "Bit 24 - PTP pdelay_req frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyreqfrmtx(&mut self) -> PTPPDLYREQFRMTX_W {
        PTPPDLYREQFRMTX_W::new(self)
    }
    #[doc = "Bit 25 - PTP pdelay_resp frame transmitted"]
    #[inline(always)]
    pub fn ptppdlyrespfrmtx(&mut self) -> PTPPDLYRESPFRMTX_W {
        PTPPDLYRESPFRMTX_W::new(self)
    }
    #[doc = "Bit 26 - TSU seconds register increment"]
    #[inline(always)]
    pub fn tsusecregincr(&mut self) -> TSUSECREGINCR_W {
        TSUSECREGINCR_W::new(self)
    }
    #[doc = "Bit 27 - Receive LPI indication status bit change"]
    #[inline(always)]
    pub fn rxlpiindc(&mut self) -> RXLPIINDC_W {
        RXLPIINDC_W::new(self)
    }
    #[doc = "Bit 28 - WOL event received interrupt."]
    #[inline(always)]
    pub fn wolevntrx(&mut self) -> WOLEVNTRX_W {
        WOLEVNTRX_W::new(self)
    }
    #[doc = "Bit 29 - TSU timer comparison interrupt."]
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
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](index.html) module"]
pub struct IFCR_SPEC;
impl crate::RegisterSpec for IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifcr::R](R) reader structure"]
impl crate::Readable for IFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifcr::W](W) writer structure"]
impl crate::Writable for IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
