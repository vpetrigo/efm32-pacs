#[doc = "Register `R5VSYNC` reader"]
pub struct R(crate::R<R5VSYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R5VSYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R5VSYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R5VSYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLEVELBUSY` reader - 5V Regulator Voltage Register Transfer Busy"]
pub type OUTLEVELBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 5V Regulator Voltage Register Transfer Busy"]
    #[inline(always)]
    pub fn outlevelbusy(&self) -> OUTLEVELBUSY_R {
        OUTLEVELBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "5V Read Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5vsync](index.html) module"]
pub struct R5VSYNC_SPEC;
impl crate::RegisterSpec for R5VSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r5vsync::R](R) reader structure"]
impl crate::Readable for R5VSYNC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R5VSYNC to value 0"]
impl crate::Resettable for R5VSYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
