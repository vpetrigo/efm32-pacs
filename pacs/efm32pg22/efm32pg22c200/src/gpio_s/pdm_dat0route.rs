#[doc = "Register `PDM_DAT0ROUTE` reader"]
pub struct R(crate::R<PDM_DAT0ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDM_DAT0ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDM_DAT0ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDM_DAT0ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDM_DAT0ROUTE` writer"]
pub struct W(crate::W<PDM_DAT0ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDM_DAT0ROUTE_SPEC>;
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
impl From<crate::W<PDM_DAT0ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDM_DAT0ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - DAT0 port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - DAT0 port select register"]
pub type PORT_W<'a> = crate::FieldWriter<'a, u32, PDM_DAT0ROUTE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PIN` reader - DAT0 pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - DAT0 pin select register"]
pub type PIN_W<'a> = crate::FieldWriter<'a, u32, PDM_DAT0ROUTE_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:1 - DAT0 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - DAT0 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DAT0 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - DAT0 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAT0 port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdm_dat0route](index.html) module"]
pub struct PDM_DAT0ROUTE_SPEC;
impl crate::RegisterSpec for PDM_DAT0ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdm_dat0route::R](R) reader structure"]
impl crate::Readable for PDM_DAT0ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdm_dat0route::W](W) writer structure"]
impl crate::Writable for PDM_DAT0ROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDM_DAT0ROUTE to value 0"]
impl crate::Resettable for PDM_DAT0ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
