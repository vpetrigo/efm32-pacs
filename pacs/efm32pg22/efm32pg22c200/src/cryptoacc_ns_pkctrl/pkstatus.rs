#[doc = "Register `PKSTATUS` reader"]
pub struct R(crate::R<PKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FAILADDR` reader - Fail Address"]
pub type FAILADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NOTONCURVE` reader - Point Px not on curve"]
pub type NOTONCURVE_R = crate::BitReader<bool>;
#[doc = "Field `ATINFINITY` reader - Point Px at infinity"]
pub type ATINFINITY_R = crate::BitReader<bool>;
#[doc = "Field `COUPLENOTVALID` reader - Couple not valid"]
pub type COUPLENOTVALID_R = crate::BitReader<bool>;
#[doc = "Field `PARAMNNOTVALID` reader - Param n not valid"]
pub type PARAMNNOTVALID_R = crate::BitReader<bool>;
#[doc = "Field `NOTIMPLEMENTED` reader - Not implemented"]
pub type NOTIMPLEMENTED_R = crate::BitReader<bool>;
#[doc = "Field `SIGNOTVALID` reader - Signature not valid"]
pub type SIGNOTVALID_R = crate::BitReader<bool>;
#[doc = "Field `PARAMABNOTVALID` reader - Param AB not valid"]
pub type PARAMABNOTVALID_R = crate::BitReader<bool>;
#[doc = "Field `NOTINVERTIBLE` reader - Not invertible"]
pub type NOTINVERTIBLE_R = crate::BitReader<bool>;
#[doc = "Composite\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPOSITE_A {
    #[doc = "0: random number under test is probably prime"]
    FALSE = 0,
    #[doc = "1: random number under test is composite"]
    TRUE = 1,
}
impl From<COMPOSITE_A> for bool {
    #[inline(always)]
    fn from(variant: COMPOSITE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPOSITE` reader - Composite"]
pub type COMPOSITE_R = crate::BitReader<COMPOSITE_A>;
impl COMPOSITE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPOSITE_A {
        match self.bits {
            false => COMPOSITE_A::FALSE,
            true => COMPOSITE_A::TRUE,
        }
    }
    #[doc = "Checks if the value of the field is `FALSE`"]
    #[inline(always)]
    pub fn is_false(&self) -> bool {
        *self == COMPOSITE_A::FALSE
    }
    #[doc = "Checks if the value of the field is `TRUE`"]
    #[inline(always)]
    pub fn is_true(&self) -> bool {
        *self == COMPOSITE_A::TRUE
    }
}
#[doc = "Field `NOTQUAD` reader - Not quadratic residue"]
pub type NOTQUAD_R = crate::BitReader<bool>;
#[doc = "Field `PKBUSY` reader - PK busy"]
pub type PKBUSY_R = crate::BitReader<bool>;
#[doc = "Field `PKIF` reader - Interrupt status"]
pub type PKIF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Fail Address"]
    #[inline(always)]
    pub fn failaddr(&self) -> FAILADDR_R {
        FAILADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Point Px not on curve"]
    #[inline(always)]
    pub fn notoncurve(&self) -> NOTONCURVE_R {
        NOTONCURVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Point Px at infinity"]
    #[inline(always)]
    pub fn atinfinity(&self) -> ATINFINITY_R {
        ATINFINITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Couple not valid"]
    #[inline(always)]
    pub fn couplenotvalid(&self) -> COUPLENOTVALID_R {
        COUPLENOTVALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Param n not valid"]
    #[inline(always)]
    pub fn paramnnotvalid(&self) -> PARAMNNOTVALID_R {
        PARAMNNOTVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not implemented"]
    #[inline(always)]
    pub fn notimplemented(&self) -> NOTIMPLEMENTED_R {
        NOTIMPLEMENTED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Signature not valid"]
    #[inline(always)]
    pub fn signotvalid(&self) -> SIGNOTVALID_R {
        SIGNOTVALID_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Param AB not valid"]
    #[inline(always)]
    pub fn paramabnotvalid(&self) -> PARAMABNOTVALID_R {
        PARAMABNOTVALID_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Not invertible"]
    #[inline(always)]
    pub fn notinvertible(&self) -> NOTINVERTIBLE_R {
        NOTINVERTIBLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Composite"]
    #[inline(always)]
    pub fn composite(&self) -> COMPOSITE_R {
        COMPOSITE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Not quadratic residue"]
    #[inline(always)]
    pub fn notquad(&self) -> NOTQUAD_R {
        NOTQUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - PK busy"]
    #[inline(always)]
    pub fn pkbusy(&self) -> PKBUSY_R {
        PKBUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt status"]
    #[inline(always)]
    pub fn pkif(&self) -> PKIF_R {
        PKIF_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkstatus](index.html) module"]
pub struct PKSTATUS_SPEC;
impl crate::RegisterSpec for PKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkstatus::R](R) reader structure"]
impl crate::Readable for PKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PKSTATUS to value 0"]
impl crate::Resettable for PKSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
