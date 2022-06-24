#[doc = "Register `SRAMFILL` reader"]
pub struct R(crate::R<SRAMFILL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMFILL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMFILL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMFILL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRAMFILLINDACREAD` reader - SRAM Fill Level (Indirect Read Partition)"]
pub type SRAMFILLINDACREAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SRAMFILLINDACWRITE` reader - SRAM Fill Level (Indirect Write Partition)"]
pub type SRAMFILLINDACWRITE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - SRAM Fill Level (Indirect Read Partition)"]
    #[inline(always)]
    pub fn sramfillindacread(&self) -> SRAMFILLINDACREAD_R {
        SRAMFILLINDACREAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM Fill Level (Indirect Write Partition)"]
    #[inline(always)]
    pub fn sramfillindacwrite(&self) -> SRAMFILLINDACWRITE_R {
        SRAMFILLINDACWRITE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SRAM Fill Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramfill](index.html) module"]
pub struct SRAMFILL_SPEC;
impl crate::RegisterSpec for SRAMFILL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramfill::R](R) reader structure"]
impl crate::Readable for SRAMFILL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRAMFILL to value 0"]
impl crate::Resettable for SRAMFILL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
