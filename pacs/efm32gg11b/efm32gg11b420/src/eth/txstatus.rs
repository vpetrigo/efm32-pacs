#[doc = "Register `TXSTATUS` reader"]
pub struct R(crate::R<TXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXSTATUS` writer"]
pub struct W(crate::W<TXSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXSTATUS_SPEC>;
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
impl From<crate::W<TXSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USEDBITREAD` reader - Used bit read"]
pub type USEDBITREAD_R = crate::BitReader<bool>;
#[doc = "Field `USEDBITREAD` writer - Used bit read"]
pub type USEDBITREAD_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 0>;
#[doc = "Field `COLOCCRD` reader - Collision occurred"]
pub type COLOCCRD_R = crate::BitReader<bool>;
#[doc = "Field `COLOCCRD` writer - Collision occurred"]
pub type COLOCCRD_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 1>;
#[doc = "Field `RETRYLMTEXCD` reader - Retry limit exceeded"]
pub type RETRYLMTEXCD_R = crate::BitReader<bool>;
#[doc = "Field `RETRYLMTEXCD` writer - Retry limit exceeded"]
pub type RETRYLMTEXCD_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 2>;
#[doc = "Field `TXGO` reader - Transmit go"]
pub type TXGO_R = crate::BitReader<bool>;
#[doc = "Field `AMBAERR` reader - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AMBAERR_R = crate::BitReader<bool>;
#[doc = "Field `AMBAERR` writer - Transmit frame corruption due to AMBA (AHB) errors."]
pub type AMBAERR_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 4>;
#[doc = "Field `TXCMPLT` reader - Transmit complete"]
pub type TXCMPLT_R = crate::BitReader<bool>;
#[doc = "Field `TXCMPLT` writer - Transmit complete"]
pub type TXCMPLT_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 5>;
#[doc = "Field `TXUNDERRUN` reader - Transmit under run"]
pub type TXUNDERRUN_R = crate::BitReader<bool>;
#[doc = "Field `TXUNDERRUN` writer - Transmit under run"]
pub type TXUNDERRUN_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 6>;
#[doc = "Field `LATECOLOCCRD` reader - Late collision occurred"]
pub type LATECOLOCCRD_R = crate::BitReader<bool>;
#[doc = "Field `LATECOLOCCRD` writer - Late collision occurred"]
pub type LATECOLOCCRD_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 7>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RESPNOTOK_R = crate::BitReader<bool>;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RESPNOTOK_W<'a> = crate::BitWriter<'a, u32, TXSTATUS_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&self) -> USEDBITREAD_R {
        USEDBITREAD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&self) -> COLOCCRD_R {
        COLOCCRD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&self) -> RETRYLMTEXCD_R {
        RETRYLMTEXCD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit go"]
    #[inline(always)]
    pub fn txgo(&self) -> TXGO_R {
        TXGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&self) -> AMBAERR_R {
        AMBAERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&self) -> TXCMPLT_R {
        TXCMPLT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&self) -> TXUNDERRUN_R {
        TXUNDERRUN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&self) -> LATECOLOCCRD_R {
        LATECOLOCCRD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Used bit read"]
    #[inline(always)]
    pub fn usedbitread(&mut self) -> USEDBITREAD_W {
        USEDBITREAD_W::new(self)
    }
    #[doc = "Bit 1 - Collision occurred"]
    #[inline(always)]
    pub fn coloccrd(&mut self) -> COLOCCRD_W {
        COLOCCRD_W::new(self)
    }
    #[doc = "Bit 2 - Retry limit exceeded"]
    #[inline(always)]
    pub fn retrylmtexcd(&mut self) -> RETRYLMTEXCD_W {
        RETRYLMTEXCD_W::new(self)
    }
    #[doc = "Bit 4 - Transmit frame corruption due to AMBA (AHB) errors."]
    #[inline(always)]
    pub fn ambaerr(&mut self) -> AMBAERR_W {
        AMBAERR_W::new(self)
    }
    #[doc = "Bit 5 - Transmit complete"]
    #[inline(always)]
    pub fn txcmplt(&mut self) -> TXCMPLT_W {
        TXCMPLT_W::new(self)
    }
    #[doc = "Bit 6 - Transmit under run"]
    #[inline(always)]
    pub fn txunderrun(&mut self) -> TXUNDERRUN_W {
        TXUNDERRUN_W::new(self)
    }
    #[doc = "Bit 7 - Late collision occurred"]
    #[inline(always)]
    pub fn latecoloccrd(&mut self) -> LATECOLOCCRD_W {
        LATECOLOCCRD_W::new(self)
    }
    #[doc = "Bit 8 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](index.html) module"]
pub struct TXSTATUS_SPEC;
impl crate::RegisterSpec for TXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txstatus::R](R) reader structure"]
impl crate::Readable for TXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txstatus::W](W) writer structure"]
impl crate::Writable for TXSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
