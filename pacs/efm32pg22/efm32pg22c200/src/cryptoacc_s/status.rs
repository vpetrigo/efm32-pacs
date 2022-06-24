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
#[doc = "Field `FETCHERBSY` reader - Fetcher busy"]
pub type FETCHERBSY_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERBSY` reader - Pusher busy"]
pub type PUSHERBSY_R = crate::BitReader<bool>;
#[doc = "Field `NOTEMPTY` reader - Not empty flag from input FIFO (fetcher)"]
pub type NOTEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `WAITING` reader - Pusher waiting for FIFO"]
pub type WAITING_R = crate::BitReader<bool>;
#[doc = "Field `SOFTRSTBSY` reader - Software reset busy"]
pub type SOFTRSTBSY_R = crate::BitReader<bool>;
#[doc = "Field `FIFODATANUM` reader - Number of data in output FIFO"]
pub type FIFODATANUM_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Fetcher busy"]
    #[inline(always)]
    pub fn fetcherbsy(&self) -> FETCHERBSY_R {
        FETCHERBSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pusher busy"]
    #[inline(always)]
    pub fn pusherbsy(&self) -> PUSHERBSY_R {
        PUSHERBSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Not empty flag from input FIFO (fetcher)"]
    #[inline(always)]
    pub fn notempty(&self) -> NOTEMPTY_R {
        NOTEMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pusher waiting for FIFO"]
    #[inline(always)]
    pub fn waiting(&self) -> WAITING_R {
        WAITING_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software reset busy"]
    #[inline(always)]
    pub fn softrstbsy(&self) -> SOFTRSTBSY_R {
        SOFTRSTBSY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Number of data in output FIFO"]
    #[inline(always)]
    pub fn fifodatanum(&self) -> FIFODATANUM_R {
        FIFODATANUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
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
