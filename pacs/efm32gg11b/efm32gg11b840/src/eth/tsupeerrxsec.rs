#[doc = "Register `TSUPEERRXSEC` reader"]
pub struct R(crate::R<TSUPEERRXSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPEERRXSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPEERRXSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPEERRXSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Seconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Peer Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(self.bits)
    }
}
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsupeerrxsec](index.html) module"]
pub struct TSUPEERRXSEC_SPEC;
impl crate::RegisterSpec for TSUPEERRXSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsupeerrxsec::R](R) reader structure"]
impl crate::Readable for TSUPEERRXSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPEERRXSEC to value 0"]
impl crate::Resettable for TSUPEERRXSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
