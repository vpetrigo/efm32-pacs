#[doc = "Register `BA418_HW_CFG` reader"]
pub struct R(crate::R<BA418_HW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BA418_HW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BA418_HW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BA418_HW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_Sha3CtxtEn` reader - Generic g_Sha3CtxtEn value"]
pub type G_SHA3CTXTEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Generic g_Sha3CtxtEn value"]
    #[inline(always)]
    pub fn g_sha3ctxt_en(&self) -> G_SHA3CTXTEN_R {
        G_SHA3CTXTEN_R::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ba418_hw_cfg](index.html) module"]
pub struct BA418_HW_CFG_SPEC;
impl crate::RegisterSpec for BA418_HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ba418_hw_cfg::R](R) reader structure"]
impl crate::Readable for BA418_HW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BA418_HW_CFG to value 0x01"]
impl crate::Resettable for BA418_HW_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
