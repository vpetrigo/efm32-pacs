#[doc = "Register `NOOFPOLLSBEFEXP` reader"]
pub struct R(crate::R<NOOFPOLLSBEFEXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOOFPOLLSBEFEXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOOFPOLLSBEFEXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOOFPOLLSBEFEXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOOFPOLLSBEFEXP` writer"]
pub struct W(crate::W<NOOFPOLLSBEFEXP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOOFPOLLSBEFEXP_SPEC>;
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
impl From<crate::W<NOOFPOLLSBEFEXP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOOFPOLLSBEFEXP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOOFPOLLSBEFEXP` reader - Number of Polls Cycles Before Expiration"]
pub type NOOFPOLLSBEFEXP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NOOFPOLLSBEFEXP` writer - Number of Polls Cycles Before Expiration"]
pub type NOOFPOLLSBEFEXP_W<'a> = crate::FieldWriter<'a, u32, NOOFPOLLSBEFEXP_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&self) -> NOOFPOLLSBEFEXP_R {
        NOOFPOLLSBEFEXP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&mut self) -> NOOFPOLLSBEFEXP_W {
        NOOFPOLLSBEFEXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polling Expiration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [noofpollsbefexp](index.html) module"]
pub struct NOOFPOLLSBEFEXP_SPEC;
impl crate::RegisterSpec for NOOFPOLLSBEFEXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [noofpollsbefexp::R](R) reader structure"]
impl crate::Readable for NOOFPOLLSBEFEXP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [noofpollsbefexp::W](W) writer structure"]
impl crate::Writable for NOOFPOLLSBEFEXP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NOOFPOLLSBEFEXP to value 0xffff_ffff"]
impl crate::Resettable for NOOFPOLLSBEFEXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
