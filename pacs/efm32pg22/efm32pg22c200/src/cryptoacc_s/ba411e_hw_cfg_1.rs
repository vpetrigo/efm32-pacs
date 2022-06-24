#[doc = "Register `BA411E_HW_CFG_1` reader"]
pub struct R(crate::R<BA411E_HW_CFG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BA411E_HW_CFG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BA411E_HW_CFG_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BA411E_HW_CFG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_AesModesPoss` reader - AES Modes Supported"]
pub type G_AESMODESPOSS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `g_CS` reader - Generic g_CS value"]
pub type G_CS_R = crate::BitReader<bool>;
#[doc = "Field `g_UseMasking` reader - Generic g_UseMasking value"]
pub type G_USEMASKING_R = crate::BitReader<bool>;
#[doc = "Field `g_Keysize` reader - Generic g_Keysize value"]
pub type G_KEYSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:8 - AES Modes Supported"]
    #[inline(always)]
    pub fn g_aes_modes_poss(&self) -> G_AESMODESPOSS_R {
        G_AESMODESPOSS_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - Generic g_CS value"]
    #[inline(always)]
    pub fn g_cs(&self) -> G_CS_R {
        G_CS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Generic g_UseMasking value"]
    #[inline(always)]
    pub fn g_use_masking(&self) -> G_USEMASKING_R {
        G_USEMASKING_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Generic g_Keysize value"]
    #[inline(always)]
    pub fn g_keysize(&self) -> G_KEYSIZE_R {
        G_KEYSIZE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ba411e_hw_cfg_1](index.html) module"]
pub struct BA411E_HW_CFG_1_SPEC;
impl crate::RegisterSpec for BA411E_HW_CFG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ba411e_hw_cfg_1::R](R) reader structure"]
impl crate::Readable for BA411E_HW_CFG_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BA411E_HW_CFG_1 to value 0x0700_017f"]
impl crate::Resettable for BA411E_HW_CFG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_017f
    }
}
