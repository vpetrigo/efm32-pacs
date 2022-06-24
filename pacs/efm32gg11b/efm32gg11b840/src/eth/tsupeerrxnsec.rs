#[doc = "Register `TSUPEERRXNSEC` reader"]
pub struct R(crate::R<TSUPEERRXNSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUPEERRXNSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUPEERRXNSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUPEERRXNSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - PTP Peer Event Frame Received Nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsupeerrxnsec](index.html) module"]
pub struct TSUPEERRXNSEC_SPEC;
impl crate::RegisterSpec for TSUPEERRXNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsupeerrxnsec::R](R) reader structure"]
impl crate::Readable for TSUPEERRXNSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TSUPEERRXNSEC to value 0"]
impl crate::Resettable for TSUPEERRXNSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
