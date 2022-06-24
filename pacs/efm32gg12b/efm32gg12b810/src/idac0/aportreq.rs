#[doc = "Register `APORTREQ` reader"]
pub struct R(crate::R<APORTREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APORTREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APORTREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APORTREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APORT1XREQ` reader - 1 If the APORT Bus Connected to APORT1X is Requested"]
pub type APORT1XREQ_R = crate::BitReader<bool>;
#[doc = "Field `APORT1YREQ` reader - 1 If the Bus Connected to APORT1Y is Requested"]
pub type APORT1YREQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - 1 If the APORT Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> APORT1XREQ_R {
        APORT1XREQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> APORT1YREQ_R {
        APORT1YREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "APORT Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportreq](index.html) module"]
pub struct APORTREQ_SPEC;
impl crate::RegisterSpec for APORTREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aportreq::R](R) reader structure"]
impl crate::Readable for APORTREQ_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APORTREQ to value 0"]
impl crate::Resettable for APORTREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
