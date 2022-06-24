#[doc = "Register `TXTHRESH` reader"]
pub struct R(crate::R<TXTHRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXTHRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXTHRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXTHRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXTHRESH` writer"]
pub struct W(crate::W<TXTHRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXTHRESH_SPEC>;
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
impl From<crate::W<TXTHRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXTHRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LEVEL_W<'a> = crate::FieldWriter<'a, u32, TXTHRESH_SPEC, u8, u8, 5, 0>;
impl R {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LEVEL_R {
        LEVEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Threshold Level"]
    #[inline(always)]
    pub fn level(&mut self) -> LEVEL_W {
        LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txthresh](index.html) module"]
pub struct TXTHRESH_SPEC;
impl crate::RegisterSpec for TXTHRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txthresh::R](R) reader structure"]
impl crate::Readable for TXTHRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txthresh::W](W) writer structure"]
impl crate::Writable for TXTHRESH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXTHRESH to value 0x01"]
impl crate::Resettable for TXTHRESH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
