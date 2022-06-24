#[doc = "Register `BIASCONF` reader"]
pub struct R(crate::R<BIASCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASCONF` writer"]
pub struct W(crate::W<BIASCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASCONF_SPEC>;
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
impl From<crate::W<BIASCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NADUTYEM01` reader - NA DUTY in EM01"]
pub type NADUTYEM01_R = crate::BitReader<bool>;
#[doc = "Field `NADUTYEM01` writer - NA DUTY in EM01"]
pub type NADUTYEM01_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 2>;
#[doc = "Field `LPEM01` reader - LP in EM01"]
pub type LPEM01_R = crate::BitReader<bool>;
#[doc = "Field `LPEM01` writer - LP in EM01"]
pub type LPEM01_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 3>;
#[doc = "Field `GMCEM23` reader - GMC in EM234"]
pub type GMCEM23_R = crate::BitReader<bool>;
#[doc = "Field `GMCEM23` writer - GMC in EM234"]
pub type GMCEM23_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 4>;
#[doc = "Field `UADUTYEM23` reader - UADUTY in EM234"]
pub type UADUTYEM23_R = crate::BitReader<bool>;
#[doc = "Field `UADUTYEM23` writer - UADUTY in EM234"]
pub type UADUTYEM23_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 5>;
#[doc = "Field `NADUTYEM23` reader - NA DUTY in EM234"]
pub type NADUTYEM23_R = crate::BitReader<bool>;
#[doc = "Field `NADUTYEM23` writer - NA DUTY in EM234"]
pub type NADUTYEM23_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 6>;
#[doc = "Field `LPEM23` reader - LP in EM234"]
pub type LPEM23_R = crate::BitReader<bool>;
#[doc = "Field `LPEM23` writer - LP in EM234"]
pub type LPEM23_W<'a> = crate::BitWriter<'a, u32, BIASCONF_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&self) -> NADUTYEM01_R {
        NADUTYEM01_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&self) -> LPEM01_R {
        LPEM01_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&self) -> GMCEM23_R {
        GMCEM23_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&self) -> UADUTYEM23_R {
        UADUTYEM23_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&self) -> NADUTYEM23_R {
        NADUTYEM23_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&self) -> LPEM23_R {
        LPEM23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - NA DUTY in EM01"]
    #[inline(always)]
    pub fn nadutyem01(&mut self) -> NADUTYEM01_W {
        NADUTYEM01_W::new(self)
    }
    #[doc = "Bit 3 - LP in EM01"]
    #[inline(always)]
    pub fn lpem01(&mut self) -> LPEM01_W {
        LPEM01_W::new(self)
    }
    #[doc = "Bit 4 - GMC in EM234"]
    #[inline(always)]
    pub fn gmcem23(&mut self) -> GMCEM23_W {
        GMCEM23_W::new(self)
    }
    #[doc = "Bit 5 - UADUTY in EM234"]
    #[inline(always)]
    pub fn uadutyem23(&mut self) -> UADUTYEM23_W {
        UADUTYEM23_W::new(self)
    }
    #[doc = "Bit 6 - NA DUTY in EM234"]
    #[inline(always)]
    pub fn nadutyem23(&mut self) -> NADUTYEM23_W {
        NADUTYEM23_W::new(self)
    }
    #[doc = "Bit 7 - LP in EM234"]
    #[inline(always)]
    pub fn lpem23(&mut self) -> LPEM23_W {
        LPEM23_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configurations Related to the Bias\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasconf](index.html) module"]
pub struct BIASCONF_SPEC;
impl crate::RegisterSpec for BIASCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasconf::R](R) reader structure"]
impl crate::Readable for BIASCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasconf::W](W) writer structure"]
impl crate::Writable for BIASCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIASCONF to value 0xf8"]
impl crate::Resettable for BIASCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf8
    }
}
