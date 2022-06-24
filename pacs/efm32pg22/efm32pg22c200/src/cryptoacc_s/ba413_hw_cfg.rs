#[doc = "Register `BA413_HW_CFG` reader"]
pub struct R(crate::R<BA413_HW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BA413_HW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BA413_HW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BA413_HW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_HashMaskFunc` reader - Generic g_HashMaskFunc value"]
pub type G_HASHMASKFUNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `g_HashPadding` reader - Generic g_HashPadding value"]
pub type G_HASHPADDING_R = crate::BitReader<bool>;
#[doc = "Field `g_HMAC_enabled` reader - Generic g_HMAC_enabled value"]
pub type G_HMAC_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `g_HashVerifyDigest` reader - Generic g_HashVerifyDigest value"]
pub type G_HASHVERIFYDIGEST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:6 - Generic g_HashMaskFunc value"]
    #[inline(always)]
    pub fn g_hash_mask_func(&self) -> G_HASHMASKFUNC_R {
        G_HASHMASKFUNC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Generic g_HashPadding value"]
    #[inline(always)]
    pub fn g_hash_padding(&self) -> G_HASHPADDING_R {
        G_HASHPADDING_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_HMAC_enabled value"]
    #[inline(always)]
    pub fn g_hmac_enabled(&self) -> G_HMAC_ENABLED_R {
        G_HMAC_ENABLED_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Generic g_HashVerifyDigest value"]
    #[inline(always)]
    pub fn g_hash_verify_digest(&self) -> G_HASHVERIFYDIGEST_R {
        G_HASHVERIFYDIGEST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ba413_hw_cfg](index.html) module"]
pub struct BA413_HW_CFG_SPEC;
impl crate::RegisterSpec for BA413_HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ba413_hw_cfg::R](R) reader structure"]
impl crate::Readable for BA413_HW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BA413_HW_CFG to value 0x0003_007f"]
impl crate::Resettable for BA413_HW_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_007f
    }
}
