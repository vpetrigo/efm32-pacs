#[doc = "Register `AIS31CONF1` reader"]
pub struct R(crate::R<AIS31CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIS31CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIS31CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIS31CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIS31CONF1` writer"]
pub struct W(crate::W<AIS31CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIS31CONF1_SPEC>;
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
impl From<crate::W<AIS31CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIS31CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HEXPECTEDVALUE` reader - Expected History Value"]
pub type HEXPECTEDVALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEXPECTEDVALUE` writer - Expected History Value"]
pub type HEXPECTEDVALUE_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF1_SPEC, u16, u16, 15, 0>;
#[doc = "Field `ONLINEREPTHRESH` reader - Online Repeat Threshold"]
pub type ONLINEREPTHRESH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ONLINEREPTHRESH` writer - Online Repeat Threshold"]
pub type ONLINEREPTHRESH_W<'a> = crate::FieldWriter<'a, u32, AIS31CONF1_SPEC, u16, u16, 15, 16>;
impl R {
    #[doc = "Bits 0:14 - Expected History Value"]
    #[inline(always)]
    pub fn hexpectedvalue(&self) -> HEXPECTEDVALUE_R {
        HEXPECTEDVALUE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Online Repeat Threshold"]
    #[inline(always)]
    pub fn onlinerepthresh(&self) -> ONLINEREPTHRESH_R {
        ONLINEREPTHRESH_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Expected History Value"]
    #[inline(always)]
    pub fn hexpectedvalue(&mut self) -> HEXPECTEDVALUE_W {
        HEXPECTEDVALUE_W::new(self)
    }
    #[doc = "Bits 16:30 - Online Repeat Threshold"]
    #[inline(always)]
    pub fn onlinerepthresh(&mut self) -> ONLINEREPTHRESH_W {
        ONLINEREPTHRESH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ais31conf1](index.html) module"]
pub struct AIS31CONF1_SPEC;
impl crate::RegisterSpec for AIS31CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ais31conf1::R](R) reader structure"]
impl crate::Readable for AIS31CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ais31conf1::W](W) writer structure"]
impl crate::Writable for AIS31CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AIS31CONF1 to value 0x03c0_0680"]
impl crate::Resettable for AIS31CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03c0_0680
    }
}
