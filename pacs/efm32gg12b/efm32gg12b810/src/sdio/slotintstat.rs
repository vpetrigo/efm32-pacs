#[doc = "Register `SLOTINTSTAT` reader"]
pub struct R(crate::R<SLOTINTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOTINTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOTINTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOTINTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTSLOT0` reader - Interrupt Signal for Slot#0"]
pub type INTSLOT0_R = crate::BitReader<bool>;
#[doc = "Field `SPECVERNUM` reader - Host Controller Compliant Spec Version Number"]
pub type SPECVERNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VENDVERNUM` reader - Vendor Version Number"]
pub type VENDVERNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Slot#0"]
    #[inline(always)]
    pub fn intslot0(&self) -> INTSLOT0_R {
        INTSLOT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Host Controller Compliant Spec Version Number"]
    #[inline(always)]
    pub fn specvernum(&self) -> SPECVERNUM_R {
        SPECVERNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor Version Number"]
    #[inline(always)]
    pub fn vendvernum(&self) -> VENDVERNUM_R {
        VENDVERNUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Slot Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slotintstat](index.html) module"]
pub struct SLOTINTSTAT_SPEC;
impl crate::RegisterSpec for SLOTINTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slotintstat::R](R) reader structure"]
impl crate::Readable for SLOTINTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SLOTINTSTAT to value 0x1002_0000"]
impl crate::Resettable for SLOTINTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1002_0000
    }
}
