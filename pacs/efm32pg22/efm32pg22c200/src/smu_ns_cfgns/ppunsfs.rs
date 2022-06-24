#[doc = "Register `PPUNSFS` reader"]
pub struct R(crate::R<PPUNSFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUNSFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUNSFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUNSFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPUFSPERIPHID` reader - Peripheral ID"]
pub type PPUFSPERIPHID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Peripheral ID"]
    #[inline(always)]
    pub fn ppufsperiphid(&self) -> PPUFSPERIPHID_R {
        PPUFSPERIPHID_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Read this register to query the fault status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppunsfs](index.html) module"]
pub struct PPUNSFS_SPEC;
impl crate::RegisterSpec for PPUNSFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppunsfs::R](R) reader structure"]
impl crate::Readable for PPUNSFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PPUNSFS to value 0"]
impl crate::Resettable for PPUNSFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
