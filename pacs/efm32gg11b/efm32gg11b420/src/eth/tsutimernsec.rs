#[doc = "Register `TSUTIMERNSEC` reader"]
pub struct R(crate::R<TSUTIMERNSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERNSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERNSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERNSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERNSEC` writer"]
pub struct W(crate::W<TSUTIMERNSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERNSEC_SPEC>;
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
impl From<crate::W<TSUTIMERNSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERNSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER` reader - Timer count in nanoseconds"]
pub type TIMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER` writer - Timer count in nanoseconds"]
pub type TIMER_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERNSEC_SPEC, u32, u32, 30, 0>;
impl R {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer count in nanoseconds"]
    #[inline(always)]
    pub fn timer(&mut self) -> TIMER_W {
        TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimernsec](index.html) module"]
pub struct TSUTIMERNSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimernsec::R](R) reader structure"]
impl crate::Readable for TSUTIMERNSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimernsec::W](W) writer structure"]
impl crate::Writable for TSUTIMERNSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERNSEC to value 0"]
impl crate::Resettable for TSUTIMERNSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
