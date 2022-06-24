#[doc = "Register `HFXOTRIMSTATUS` reader"]
pub struct R(crate::R<HFXOTRIMSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOTRIMSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOTRIMSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOTRIMSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IBTRIMXOCORE` reader - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
pub type IBTRIMXOCORE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGISH` reader - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
pub type REGISH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:10 - Value of REGISH Found By Automatic HFXO Shunt Current Optimization Algorithm"]
    #[inline(always)]
    pub fn regish(&self) -> REGISH_R {
        REGISH_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
#[doc = "HFXO Trim Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxotrimstatus](index.html) module"]
pub struct HFXOTRIMSTATUS_SPEC;
impl crate::RegisterSpec for HFXOTRIMSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxotrimstatus::R](R) reader structure"]
impl crate::Readable for HFXOTRIMSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFXOTRIMSTATUS to value 0x0500"]
impl crate::Resettable for HFXOTRIMSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0500
    }
}
