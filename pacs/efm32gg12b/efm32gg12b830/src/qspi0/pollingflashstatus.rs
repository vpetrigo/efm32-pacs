#[doc = "Register `POLLINGFLASHSTATUS` reader"]
pub struct R(crate::R<POLLINGFLASHSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POLLINGFLASHSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POLLINGFLASHSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POLLINGFLASHSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POLLINGFLASHSTATUS` writer"]
pub struct W(crate::W<POLLINGFLASHSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POLLINGFLASHSTATUS_SPEC>;
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
impl From<crate::W<POLLINGFLASHSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POLLINGFLASHSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVICESTATUS` reader - Device Status"]
pub type DEVICESTATUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVICESTATUSVALID` reader - Device Status Valid"]
pub type DEVICESTATUSVALID_R = crate::BitReader<bool>;
#[doc = "Field `DEVICESTATUSNBDUMMY` reader - Auto-polling Dummy Cycles"]
pub type DEVICESTATUSNBDUMMY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVICESTATUSNBDUMMY` writer - Auto-polling Dummy Cycles"]
pub type DEVICESTATUSNBDUMMY_W<'a> =
    crate::FieldWriter<'a, u32, POLLINGFLASHSTATUS_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:7 - Device Status"]
    #[inline(always)]
    pub fn devicestatus(&self) -> DEVICESTATUS_R {
        DEVICESTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Device Status Valid"]
    #[inline(always)]
    pub fn devicestatusvalid(&self) -> DEVICESTATUSVALID_R {
        DEVICESTATUSVALID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&self) -> DEVICESTATUSNBDUMMY_R {
        DEVICESTATUSNBDUMMY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&mut self) -> DEVICESTATUSNBDUMMY_W {
        DEVICESTATUSNBDUMMY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polling Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pollingflashstatus](index.html) module"]
pub struct POLLINGFLASHSTATUS_SPEC;
impl crate::RegisterSpec for POLLINGFLASHSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pollingflashstatus::R](R) reader structure"]
impl crate::Readable for POLLINGFLASHSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pollingflashstatus::W](W) writer structure"]
impl crate::Writable for POLLINGFLASHSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POLLINGFLASHSTATUS to value 0"]
impl crate::Resettable for POLLINGFLASHSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
