#[doc = "Register `RXSTATUS` reader"]
pub struct R(crate::R<RXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXSTATUS` writer"]
pub struct W(crate::W<RXSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXSTATUS_SPEC>;
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
impl From<crate::W<RXSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFFNOTAVAIL` reader - Buffer not available"]
pub type BUFFNOTAVAIL_R = crate::BitReader<bool>;
#[doc = "Field `BUFFNOTAVAIL` writer - Buffer not available"]
pub type BUFFNOTAVAIL_W<'a> = crate::BitWriter<'a, u32, RXSTATUS_SPEC, bool, 0>;
#[doc = "Field `FRMRX` reader - Frame received"]
pub type FRMRX_R = crate::BitReader<bool>;
#[doc = "Field `FRMRX` writer - Frame received"]
pub type FRMRX_W<'a> = crate::BitWriter<'a, u32, RXSTATUS_SPEC, bool, 1>;
#[doc = "Field `RXOVERRUN` reader - Receive overrun"]
pub type RXOVERRUN_R = crate::BitReader<bool>;
#[doc = "Field `RXOVERRUN` writer - Receive overrun"]
pub type RXOVERRUN_W<'a> = crate::BitWriter<'a, u32, RXSTATUS_SPEC, bool, 2>;
#[doc = "Field `RESPNOTOK` reader - bresp/hresp not OK"]
pub type RESPNOTOK_R = crate::BitReader<bool>;
#[doc = "Field `RESPNOTOK` writer - bresp/hresp not OK"]
pub type RESPNOTOK_W<'a> = crate::BitWriter<'a, u32, RXSTATUS_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&self) -> BUFFNOTAVAIL_R {
        BUFFNOTAVAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&self) -> FRMRX_R {
        FRMRX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&mut self) -> BUFFNOTAVAIL_W {
        BUFFNOTAVAIL_W::new(self)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&mut self) -> FRMRX_W {
        FRMRX_W::new(self)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W::new(self)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
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
#[doc = "Receive status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](index.html) module"]
pub struct RXSTATUS_SPEC;
impl crate::RegisterSpec for RXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxstatus::R](R) reader structure"]
impl crate::Readable for RXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxstatus::W](W) writer structure"]
impl crate::Writable for RXSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXSTATUS to value 0"]
impl crate::Resettable for RXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
