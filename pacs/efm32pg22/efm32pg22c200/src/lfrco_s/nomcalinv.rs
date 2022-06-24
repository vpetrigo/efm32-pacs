#[doc = "Register `NOMCALINV` reader"]
pub struct R(crate::R<NOMCALINV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOMCALINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOMCALINV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOMCALINV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOMCALINV` writer"]
pub struct W(crate::W<NOMCALINV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOMCALINV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<NOMCALINV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOMCALINV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOMCALCNTINV` reader - Nominal Calibration Count Inverted"]
pub type NOMCALCNTINV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NOMCALCNTINV` writer - Nominal Calibration Count Inverted"]
pub type NOMCALCNTINV_W<'a> = crate::FieldWriter<'a, u32, NOMCALINV_SPEC, u32, u32, 17, 0>;
impl R {
    #[doc = "Bits 0:16 - Nominal Calibration Count Inverted"]
    #[inline(always)]
    pub fn nomcalcntinv(&self) -> NOMCALCNTINV_R {
        NOMCALCNTINV_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Nominal Calibration Count Inverted"]
    #[inline(always)]
    pub fn nomcalcntinv(&mut self) -> NOMCALCNTINV_W {
        NOMCALCNTINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Nominal calibration inverted register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nomcalinv](index.html) module"]
pub struct NOMCALINV_SPEC;
impl crate::RegisterSpec for NOMCALINV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nomcalinv::R](R) reader structure"]
impl crate::Readable for NOMCALINV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nomcalinv::W](W) writer structure"]
impl crate::Writable for NOMCALINV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOMCALINV to value 0x597a"]
impl crate::Resettable for NOMCALINV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x597a
    }
}
