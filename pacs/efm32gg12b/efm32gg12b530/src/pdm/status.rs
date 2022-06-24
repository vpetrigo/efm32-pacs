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
#[doc = "Field `ACT` reader - PDM is active"]
pub type ACT_R = crate::BitReader<bool>;
#[doc = "Field `FULL` reader - FIFO FULL Status"]
pub type FULL_R = crate::BitReader<bool>;
#[doc = "Field `EMPTY` reader - FIFO EMPTY Status"]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCNT` reader - FIFO CNT"]
pub type FIFOCNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - PDM is active"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO FULL Status"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO EMPTY Status"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - FIFO CNT"]
    #[inline(always)]
    pub fn fifocnt(&self) -> FIFOCNT_R {
        FIFOCNT_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "PDM Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x20"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
