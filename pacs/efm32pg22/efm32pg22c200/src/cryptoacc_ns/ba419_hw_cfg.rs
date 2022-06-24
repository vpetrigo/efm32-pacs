#[doc = "Register `BA419_HW_CFG` reader"]
pub struct R(crate::R<BA419_HW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BA419_HW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BA419_HW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BA419_HW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_SM4ModesPoss` reader - Generic g_SM4ModesPoss value"]
pub type G_SM4MODESPOSS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Generic g_SM4ModesPoss value"]
    #[inline(always)]
    pub fn g_sm4modes_poss(&self) -> G_SM4MODESPOSS_R {
        G_SM4MODESPOSS_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ba419_hw_cfg](index.html) module"]
pub struct BA419_HW_CFG_SPEC;
impl crate::RegisterSpec for BA419_HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ba419_hw_cfg::R](R) reader structure"]
impl crate::Readable for BA419_HW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BA419_HW_CFG to value 0x5f"]
impl crate::Resettable for BA419_HW_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x5f
    }
}
