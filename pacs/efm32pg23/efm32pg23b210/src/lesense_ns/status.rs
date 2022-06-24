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
#[doc = "Field `RESFIFOV` reader - Result fifo valid"]
pub type RESFIFOV_R = crate::BitReader<bool>;
#[doc = "Field `RESFIFOFULL` reader - Result fifo full"]
pub type RESFIFOFULL_R = crate::BitReader<bool>;
#[doc = "Field `SCANACTIVE` reader - LESENSE scan active"]
pub type SCANACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `RUNNING` reader - LESENSE periodic counter running"]
pub type RUNNING_R = crate::BitReader<bool>;
#[doc = "Field `READBUSY` reader - FIFO Read Busy"]
pub type READBUSY_R = crate::BitReader<bool>;
#[doc = "Field `FLUSHING` reader - FIFO Flushing"]
pub type FLUSHING_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Result fifo valid"]
    #[inline(always)]
    pub fn resfifov(&self) -> RESFIFOV_R {
        RESFIFOV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result fifo full"]
    #[inline(always)]
    pub fn resfifofull(&self) -> RESFIFOFULL_R {
        RESFIFOFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LESENSE scan active"]
    #[inline(always)]
    pub fn scanactive(&self) -> SCANACTIVE_R {
        SCANACTIVE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LESENSE periodic counter running"]
    #[inline(always)]
    pub fn running(&self) -> RUNNING_R {
        RUNNING_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO Read Busy"]
    #[inline(always)]
    pub fn readbusy(&self) -> READBUSY_R {
        READBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Flushing"]
    #[inline(always)]
    pub fn flushing(&self) -> FLUSHING_R {
        FLUSHING_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
