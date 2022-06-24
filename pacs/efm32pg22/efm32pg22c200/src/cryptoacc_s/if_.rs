#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FETCHERENDOFBLOCK` reader - End of block interrupt flag"]
pub type FETCHERENDOFBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERSTOPPED` reader - Stopped interrupt flag"]
pub type FETCHERSTOPPED_R = crate::BitReader<bool>;
#[doc = "Field `FETCHERERROR` reader - Error interrupt flag"]
pub type FETCHERERROR_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERENDOFBLOCK` reader - End of block interrupt flag"]
pub type PUSHERENDOFBLOCK_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERSTOPPED` reader - Stopped interrupt flag"]
pub type PUSHERSTOPPED_R = crate::BitReader<bool>;
#[doc = "Field `PUSHERERROR` reader - Error interrupt flag"]
pub type PUSHERERROR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - End of block interrupt flag"]
    #[inline(always)]
    pub fn fetcherendofblock(&self) -> FETCHERENDOFBLOCK_R {
        FETCHERENDOFBLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stopped interrupt flag"]
    #[inline(always)]
    pub fn fetcherstopped(&self) -> FETCHERSTOPPED_R {
        FETCHERSTOPPED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error interrupt flag"]
    #[inline(always)]
    pub fn fetchererror(&self) -> FETCHERERROR_R {
        FETCHERERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of block interrupt flag"]
    #[inline(always)]
    pub fn pusherendofblock(&self) -> PUSHERENDOFBLOCK_R {
        PUSHERENDOFBLOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stopped interrupt flag"]
    #[inline(always)]
    pub fn pusherstopped(&self) -> PUSHERSTOPPED_R {
        PUSHERSTOPPED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt flag"]
    #[inline(always)]
    pub fn pushererror(&self) -> PUSHERERROR_R {
        PUSHERERROR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
