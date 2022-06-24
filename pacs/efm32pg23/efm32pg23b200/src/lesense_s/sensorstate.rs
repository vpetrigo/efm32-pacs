#[doc = "Register `SENSORSTATE` reader"]
pub struct R(crate::R<SENSORSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SENSORSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SENSORSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SENSORSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SENSORSTATE` reader - Sensor State"]
pub type SENSORSTATE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Sensor State"]
    #[inline(always)]
    pub fn sensorstate(&self) -> SENSORSTATE_R {
        SENSORSTATE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Decoder input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sensorstate](index.html) module"]
pub struct SENSORSTATE_SPEC;
impl crate::RegisterSpec for SENSORSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sensorstate::R](R) reader structure"]
impl crate::Readable for SENSORSTATE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SENSORSTATE to value 0"]
impl crate::Resettable for SENSORSTATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
