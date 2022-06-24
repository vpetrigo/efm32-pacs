#[doc = "Register `EUSART1_CSROUTE` reader"]
pub struct R(crate::R<EUSART1_CSROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EUSART1_CSROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EUSART1_CSROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EUSART1_CSROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EUSART1_CSROUTE` writer"]
pub struct W(crate::W<EUSART1_CSROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EUSART1_CSROUTE_SPEC>;
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
impl From<crate::W<EUSART1_CSROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EUSART1_CSROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - CS port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - CS port select register"]
pub type PORT_W<'a> = crate::FieldWriter<'a, u32, EUSART1_CSROUTE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PIN` reader - CS pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - CS pin select register"]
pub type PIN_W<'a> = crate::FieldWriter<'a, u32, EUSART1_CSROUTE_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:1 - CS port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CS pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CS port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - CS pin select register"]
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
#[doc = "CS port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eusart1_csroute](index.html) module"]
pub struct EUSART1_CSROUTE_SPEC;
impl crate::RegisterSpec for EUSART1_CSROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eusart1_csroute::R](R) reader structure"]
impl crate::Readable for EUSART1_CSROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eusart1_csroute::W](W) writer structure"]
impl crate::Writable for EUSART1_CSROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EUSART1_CSROUTE to value 0"]
impl crate::Resettable for EUSART1_CSROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}