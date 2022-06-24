#[doc = "Register `RXLPITIME` reader"]
pub struct R(crate::R<RXLPITIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXLPITIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXLPITIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXLPITIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXLPITIME` writer"]
pub struct W(crate::W<RXLPITIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXLPITIME_SPEC>;
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
impl From<crate::W<RXLPITIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXLPITIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPITIME` reader - Time in LPI"]
pub type LPITIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPITIME` writer - Time in LPI"]
pub type LPITIME_W<'a> = crate::FieldWriter<'a, u32, RXLPITIME_SPEC, u32, u32, 24, 0>;
impl R {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&self) -> LPITIME_R {
        LPITIME_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time in LPI"]
    #[inline(always)]
    pub fn lpitime(&mut self) -> LPITIME_W {
        LPITIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Received LPI time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxlpitime](index.html) module"]
pub struct RXLPITIME_SPEC;
impl crate::RegisterSpec for RXLPITIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxlpitime::R](R) reader structure"]
impl crate::Readable for RXLPITIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxlpitime::W](W) writer structure"]
impl crate::Writable for RXLPITIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXLPITIME to value 0"]
impl crate::Resettable for RXLPITIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
