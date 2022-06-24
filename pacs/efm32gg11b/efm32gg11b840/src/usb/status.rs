#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VBUSDETH` reader - VBUS Detect High"]
pub type VBUSDETH_R = crate::BitReader<bool>;
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub type LEMACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `DCDTO` reader - Data Contact Detection Timeout"]
pub type DCDTO_R = crate::BitReader<bool>;
#[doc = "Field `SDP` reader - Standard Downstream Port Detected"]
pub type SDP_R = crate::BitReader<bool>;
#[doc = "Field `CDP` reader - Charging Downstream Port Detected"]
pub type CDP_R = crate::BitReader<bool>;
#[doc = "Field `DCP` reader - Dedicated Charging Port Detected"]
pub type DCP_R = crate::BitReader<bool>;
#[doc = "Field `ACAFS` reader - ACA Full Speed TypeB Device"]
pub type ACAFS_R = crate::BitReader<bool>;
#[doc = "Field `ACALS` reader - ACA Low Speed TypeB Device"]
pub type ACALS_R = crate::BitReader<bool>;
#[doc = "Field `USBCDBUSY` reader - USB Charger Detect Busy"]
pub type USBCDBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VBUS Detect High"]
    #[inline(always)]
    pub fn vbusdeth(&self) -> VBUSDETH_R {
        VBUSDETH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Data Contact Detection Timeout"]
    #[inline(always)]
    pub fn dcdto(&self) -> DCDTO_R {
        DCDTO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Standard Downstream Port Detected"]
    #[inline(always)]
    pub fn sdp(&self) -> SDP_R {
        SDP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Charging Downstream Port Detected"]
    #[inline(always)]
    pub fn cdp(&self) -> CDP_R {
        CDP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Dedicated Charging Port Detected"]
    #[inline(always)]
    pub fn dcp(&self) -> DCP_R {
        DCP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ACA Full Speed TypeB Device"]
    #[inline(always)]
    pub fn acafs(&self) -> ACAFS_R {
        ACAFS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ACA Low Speed TypeB Device"]
    #[inline(always)]
    pub fn acals(&self) -> ACALS_R {
        ACALS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - USB Charger Detect Busy"]
    #[inline(always)]
    pub fn usbcdbusy(&self) -> USBCDBUSY_R {
        USBCDBUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
