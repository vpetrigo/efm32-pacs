#[doc = "Register `AIS31CONF2` reader"]
pub struct R(crate::R<AIS31CONF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIS31CONF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIS31CONF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIS31CONF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIS31CONF2` writer"]
pub struct W(crate::W<AIS31CONF2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIS31CONF2_SPEC>;
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
impl From<crate::W<AIS31CONF2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIS31CONF2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HMIN` reader - Minimum Allowed History Value"]
pub type HMIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HMIN` writer - Minimum Allowed History Value"]
pub type HMIN_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF2_SPEC, u16, u16, 15, 0>;
#[doc = "Field `HMAX` reader - Maximum Allowed History Value"]
pub type HMAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HMAX` writer - Maximum Allowed History Value"]
pub type HMAX_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF2_SPEC, u16, u16, 15, 16>;
impl R {
    #[doc = "Bits 0:14 - Minimum Allowed History Value"]
    #[inline(always)]
    pub fn hmin(&self) -> HMIN_R {
        HMIN_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Maximum Allowed History Value"]
    #[inline(always)]
    pub fn hmax(&self) -> HMAX_R {
        HMAX_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Minimum Allowed History Value"]
    #[inline(always)]
    pub fn hmin(&mut self) -> HMIN_W {
        HMIN_W::new(self)
    }
    #[doc = "Bits 16:30 - Maximum Allowed History Value"]
    #[inline(always)]
    pub fn hmax(&mut self) -> HMAX_W {
        HMAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ais31conf2](index.html) module"]
pub struct AIS31CONF2_SPEC;
impl crate::RegisterSpec for AIS31CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ais31conf2::R](R) reader structure"]
impl crate::Readable for AIS31CONF2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ais31conf2::W](W) writer structure"]
impl crate::Writable for AIS31CONF2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIS31CONF2 to value 0x0440_0340"]
impl crate::Resettable for AIS31CONF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0440_0340
    }
}
