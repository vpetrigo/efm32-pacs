#[doc = "Register `DCDC_ROUTEEN` reader"]
pub struct R(crate::R<DCDC_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDC_ROUTEEN` writer"]
pub struct W(crate::W<DCDC_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC_ROUTEEN_SPEC>;
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
impl From<crate::W<DCDC_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDCCOREHIDDENPEN` reader - DCDCCOREHIDDEN pin enable control bit"]
pub type DCDCCOREHIDDENPEN_R = crate::BitReader<bool>;
#[doc = "Field `DCDCCOREHIDDENPEN` writer - DCDCCOREHIDDEN pin enable control bit"]
pub type DCDCCOREHIDDENPEN_W<'a> = crate::BitWriter<'a, u32, DCDC_ROUTEEN_SPEC, bool, 0>;
impl R {
    #[doc = "Bit 0 - DCDCCOREHIDDEN pin enable control bit"]
    #[inline(always)]
    pub fn dcdccorehiddenpen(&self) -> DCDCCOREHIDDENPEN_R {
        DCDCCOREHIDDENPEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDCCOREHIDDEN pin enable control bit"]
    #[inline(always)]
    pub fn dcdccorehiddenpen(&mut self) -> DCDCCOREHIDDENPEN_W {
        DCDCCOREHIDDENPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc_routeen](index.html) module"]
pub struct DCDC_ROUTEEN_SPEC;
impl crate::RegisterSpec for DCDC_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc_routeen::R](R) reader structure"]
impl crate::Readable for DCDC_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc_routeen::W](W) writer structure"]
impl crate::Writable for DCDC_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDC_ROUTEEN to value 0"]
impl crate::Resettable for DCDC_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
