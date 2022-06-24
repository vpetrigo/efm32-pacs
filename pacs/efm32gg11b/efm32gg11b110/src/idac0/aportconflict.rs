#[doc = "Register `APORTCONFLICT` reader"]
pub struct R(crate::R<APORTCONFLICT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APORTCONFLICT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APORTCONFLICT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APORTCONFLICT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type APORT1XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type APORT1YCONFLICT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportconflict](index.html) module"]
pub struct APORTCONFLICT_SPEC;
impl crate::RegisterSpec for APORTCONFLICT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aportconflict::R](R) reader structure"]
impl crate::Readable for APORTCONFLICT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
