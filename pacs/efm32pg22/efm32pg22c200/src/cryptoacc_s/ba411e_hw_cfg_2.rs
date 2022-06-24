#[doc = "Register `BA411E_HW_CFG_2` reader"]
pub struct R(crate::R<BA411E_HW_CFG_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BA411E_HW_CFG_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BA411E_HW_CFG_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BA411E_HW_CFG_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_CtrSize` reader - Generic g_CtrSize value"]
pub type G_CTRSIZE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Generic g_CtrSize value"]
    #[inline(always)]
    pub fn g_ctr_size(&self) -> G_CTRSIZE_R {
        G_CTRSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ba411e_hw_cfg_2](index.html) module"]
pub struct BA411E_HW_CFG_2_SPEC;
impl crate::RegisterSpec for BA411E_HW_CFG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ba411e_hw_cfg_2::R](R) reader structure"]
impl crate::Readable for BA411E_HW_CFG_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BA411E_HW_CFG_2 to value 0x80"]
impl crate::Resettable for BA411E_HW_CFG_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
