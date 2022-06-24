#[doc = "Register `M33CTRL` reader"]
pub struct R(crate::R<M33CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M33CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M33CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M33CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M33CTRL` writer"]
pub struct W(crate::W<M33CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M33CTRL_SPEC>;
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
impl From<crate::W<M33CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M33CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSVTAIRCR` reader - LOCKSVTAIRCR control of M33 CPU"]
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR` writer - LOCKSVTAIRCR control of M33 CPU"]
pub type LOCKSVTAIRCR_W<'a> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, 0>;
#[doc = "Field `LOCKNSVTOR` reader - LOCKNSVTOR control of M33 CPU"]
pub type LOCKNSVTOR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSVTOR` writer - LOCKNSVTOR control of M33 CPU"]
pub type LOCKNSVTOR_W<'a> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, 1>;
#[doc = "Field `LOCKSMPU` reader - LOCKSMPU control of M33 CPU"]
pub type LOCKSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU` writer - LOCKSMPU control of M33 CPU"]
pub type LOCKSMPU_W<'a> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, 2>;
#[doc = "Field `LOCKNSMPU` reader - LOCKNSMPU control of M33 CPU"]
pub type LOCKNSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSMPU` writer - LOCKNSMPU control of M33 CPU"]
pub type LOCKNSMPU_W<'a> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, 3>;
#[doc = "Field `LOCKSAU` reader - LOCKSAU control of M33 CPU"]
pub type LOCKSAU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU` writer - LOCKSAU control of M33 CPU"]
pub type LOCKSAU_W<'a> = crate::BitWriter<'a, u32, M33CTRL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - LOCKSVTAIRCR control of M33 CPU"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCKNSVTOR control of M33 CPU"]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCKSMPU control of M33 CPU"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCKNSMPU control of M33 CPU"]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOCKSAU control of M33 CPU"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKSVTAIRCR control of M33 CPU"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W {
        LOCKSVTAIRCR_W::new(self)
    }
    #[doc = "Bit 1 - LOCKNSVTOR control of M33 CPU"]
    #[inline(always)]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W {
        LOCKNSVTOR_W::new(self)
    }
    #[doc = "Bit 2 - LOCKSMPU control of M33 CPU"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W {
        LOCKSMPU_W::new(self)
    }
    #[doc = "Bit 3 - LOCKNSMPU control of M33 CPU"]
    #[inline(always)]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W {
        LOCKNSMPU_W::new(self)
    }
    #[doc = "Bit 4 - LOCKSAU control of M33 CPU"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W {
        LOCKSAU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Holds the M33 control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m33ctrl](index.html) module"]
pub struct M33CTRL_SPEC;
impl crate::RegisterSpec for M33CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m33ctrl::R](R) reader structure"]
impl crate::Readable for M33CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m33ctrl::W](W) writer structure"]
impl crate::Writable for M33CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M33CTRL to value 0"]
impl crate::Resettable for M33CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
