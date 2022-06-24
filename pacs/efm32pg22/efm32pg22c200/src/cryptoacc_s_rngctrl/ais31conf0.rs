#[doc = "Register `AIS31CONF0` reader"]
pub struct R(crate::R<AIS31CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIS31CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIS31CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIS31CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIS31CONF0` writer"]
pub struct W(crate::W<AIS31CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIS31CONF0_SPEC>;
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
impl From<crate::W<AIS31CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIS31CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTUPTHRES` reader - Start-up Threshold"]
pub type STARTUPTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTUPTHRES` writer - Start-up Threshold"]
pub type STARTUPTHRES_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF0_SPEC, u16, u16, 15, 0>;
#[doc = "Field `ONLINETHRESH` reader - Online Threshold"]
pub type ONLINETHRESH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ONLINETHRESH` writer - Online Threshold"]
pub type ONLINETHRESH_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF0_SPEC, u16, u16, 15, 16>;
impl R {
    #[doc = "Bits 0:14 - Start-up Threshold"]
    #[inline(always)]
    pub fn startupthres(&self) -> STARTUPTHRES_R {
        STARTUPTHRES_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Online Threshold"]
    #[inline(always)]
    pub fn onlinethresh(&self) -> ONLINETHRESH_R {
        ONLINETHRESH_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Start-up Threshold"]
    #[inline(always)]
    pub fn startupthres(&mut self) -> STARTUPTHRES_W {
        STARTUPTHRES_W::new(self)
    }
    #[doc = "Bits 16:30 - Online Threshold"]
    #[inline(always)]
    pub fn onlinethresh(&mut self) -> ONLINETHRESH_W {
        ONLINETHRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ais31conf0](index.html) module"]
pub struct AIS31CONF0_SPEC;
impl crate::RegisterSpec for AIS31CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ais31conf0::R](R) reader structure"]
impl crate::Readable for AIS31CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ais31conf0::W](W) writer structure"]
impl crate::Writable for AIS31CONF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIS31CONF0 to value 0x4340_1040"]
impl crate::Resettable for AIS31CONF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4340_1040
    }
}
