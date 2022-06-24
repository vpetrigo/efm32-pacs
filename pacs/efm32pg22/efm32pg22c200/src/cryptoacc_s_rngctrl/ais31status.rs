#[doc = "Register `AIS31STATUS` reader"]
pub struct R(crate::R<AIS31STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIS31STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIS31STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIS31STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIS31STATUS` writer"]
pub struct W(crate::W<AIS31STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIS31STATUS_SPEC>;
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
impl From<crate::W<AIS31STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIS31STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMPRELIMALARMS` reader - Number of preliminary alarms"]
pub type NUMPRELIMALARMS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NUMPRELIMALARMS` writer - Number of preliminary alarms"]
pub type NUMPRELIMALARMS_W<'a> = crate::FieldWriter<'a, u32, AIS31STATUS_SPEC, u16, u16, 16, 0>;
#[doc = "Field `PRELIMNOISEALARMRNG` reader - Preliminary noise alarm RNG"]
pub type PRELIMNOISEALARMRNG_R = crate::BitReader<bool>;
#[doc = "Field `PRELIMNOISEALARMRNG` writer - Preliminary noise alarm RNG"]
pub type PRELIMNOISEALARMRNG_W<'a> = crate::BitWriter<'a, u32, AIS31STATUS_SPEC, bool, 16>;
#[doc = "Field `PRELIMNOISEALARMREP` reader - Preliminary noise alarm Rep"]
pub type PRELIMNOISEALARMREP_R = crate::BitReader<bool>;
#[doc = "Field `PRELIMNOISEALARMREP` writer - Preliminary noise alarm Rep"]
pub type PRELIMNOISEALARMREP_W<'a> = crate::BitWriter<'a, u32, AIS31STATUS_SPEC, bool, 17>;
impl R {
    #[doc = "Bits 0:15 - Number of preliminary alarms"]
    #[inline(always)]
    pub fn numprelimalarms(&self) -> NUMPRELIMALARMS_R {
        NUMPRELIMALARMS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Preliminary noise alarm RNG"]
    #[inline(always)]
    pub fn prelimnoisealarmrng(&self) -> PRELIMNOISEALARMRNG_R {
        PRELIMNOISEALARMRNG_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Preliminary noise alarm Rep"]
    #[inline(always)]
    pub fn prelimnoisealarmrep(&self) -> PRELIMNOISEALARMREP_R {
        PRELIMNOISEALARMREP_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of preliminary alarms"]
    #[inline(always)]
    pub fn numprelimalarms(&mut self) -> NUMPRELIMALARMS_W {
        NUMPRELIMALARMS_W::new(self)
    }
    #[doc = "Bit 16 - Preliminary noise alarm RNG"]
    #[inline(always)]
    pub fn prelimnoisealarmrng(&mut self) -> PRELIMNOISEALARMRNG_W {
        PRELIMNOISEALARMRNG_W::new(self)
    }
    #[doc = "Bit 17 - Preliminary noise alarm Rep"]
    #[inline(always)]
    pub fn prelimnoisealarmrep(&mut self) -> PRELIMNOISEALARMREP_W {
        PRELIMNOISEALARMREP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to obtain diagnostic information about the AIS31 start-up and online tests when g_AIS31=True. Writing to this register clears all fields\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ais31status](index.html) module"]
pub struct AIS31STATUS_SPEC;
impl crate::RegisterSpec for AIS31STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ais31status::R](R) reader structure"]
impl crate::Readable for AIS31STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ais31status::W](W) writer structure"]
impl crate::Writable for AIS31STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIS31STATUS to value 0"]
impl crate::Resettable for AIS31STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
