#[doc = "Register `TSUTIMERINCR` reader"]
pub struct R(crate::R<TSUTIMERINCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERINCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERINCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERINCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERINCR` writer"]
pub struct W(crate::W<TSUTIMERINCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERINCR_SPEC>;
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
impl From<crate::W<TSUTIMERINCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERINCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSINCREMENT` reader - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NSINCREMENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSINCREMENT` writer - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
pub type NSINCREMENT_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERINCR_SPEC, u8, u8, 8, 0>;
#[doc = "Field `ALTNSINCR` reader - Alternative nanoseconds count"]
pub type ALTNSINCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALTNSINCR` writer - Alternative nanoseconds count"]
pub type ALTNSINCR_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERINCR_SPEC, u8, u8, 8, 8>;
#[doc = "Field `NUMINCS` reader - Number of incs before alt inc"]
pub type NUMINCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMINCS` writer - Number of incs before alt inc"]
pub type NUMINCS_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERINCR_SPEC, u8, u8, 8, 16>;
impl R {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&self) -> NSINCREMENT_R {
        NSINCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&self) -> ALTNSINCR_R {
        ALTNSINCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&self) -> NUMINCS_R {
        NUMINCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle"]
    #[inline(always)]
    pub fn nsincrement(&mut self) -> NSINCREMENT_W {
        NSINCREMENT_W::new(self)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count"]
    #[inline(always)]
    pub fn altnsincr(&mut self) -> ALTNSINCR_W {
        ALTNSINCR_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc"]
    #[inline(always)]
    pub fn numincs(&mut self) -> NUMINCS_W {
        NUMINCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimerincr](index.html) module"]
pub struct TSUTIMERINCR_SPEC;
impl crate::RegisterSpec for TSUTIMERINCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimerincr::R](R) reader structure"]
impl crate::Readable for TSUTIMERINCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimerincr::W](W) writer structure"]
impl crate::Writable for TSUTIMERINCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERINCR to value 0"]
impl crate::Resettable for TSUTIMERINCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
