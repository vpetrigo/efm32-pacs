#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STARTBUSY` reader - START sync busy"]
pub type STARTBUSY_R = crate::BitReader<bool>;
#[doc = "Field `STOPBUSY` reader - STOP sync busy"]
pub type STOPBUSY_R = crate::BitReader<bool>;
#[doc = "Field `CLEARBUSY` reader - CLEAR sync busy"]
pub type CLEARBUSY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOFLBUSY` reader - FIFO Flush Sync busy"]
pub type FIFOFLBUSY_R = crate::BitReader<bool>;
#[doc = "Field `PRESCBUSY` reader - PRESC Sync busy"]
pub type PRESCBUSY_R = crate::BitReader<bool>;
#[doc = "Field `CTRLREGBUSY` reader - CTRLREGBUSY busy"]
pub type CTRLREGBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - START sync busy"]
    #[inline(always)]
    pub fn startbusy(&self) -> STARTBUSY_R {
        STARTBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP sync busy"]
    #[inline(always)]
    pub fn stopbusy(&self) -> STOPBUSY_R {
        STOPBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLEAR sync busy"]
    #[inline(always)]
    pub fn clearbusy(&self) -> CLEARBUSY_R {
        CLEARBUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Flush Sync busy"]
    #[inline(always)]
    pub fn fifoflbusy(&self) -> FIFOFLBUSY_R {
        FIFOFLBUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - PRESC Sync busy"]
    #[inline(always)]
    pub fn prescbusy(&self) -> PRESCBUSY_R {
        PRESCBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - CTRLREGBUSY busy"]
    #[inline(always)]
    pub fn ctrlregbusy(&self) -> CTRLREGBUSY_R {
        CTRLREGBUSY_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
