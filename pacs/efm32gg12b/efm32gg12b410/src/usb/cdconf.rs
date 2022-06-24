#[doc = "Register `CDCONF` reader"]
pub struct R(crate::R<CDCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCONF` writer"]
pub struct W(crate::W<CDCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCONF_SPEC>;
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
impl From<crate::W<CDCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCDTOCONF` reader - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DCDTOCONF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DCDTOCONF` writer - DCD Timeout (TDCD_TIMEOUT) Configuration"]
pub type DCDTOCONF_W<'a> = crate::FieldWriter<'a, u32, CDCONF_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&self) -> DCDTOCONF_R {
        DCDTOCONF_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DCD Timeout (TDCD_TIMEOUT) Configuration"]
    #[inline(always)]
    pub fn dcdtoconf(&mut self) -> DCDTOCONF_W {
        DCDTOCONF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Charger Detect Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdconf](index.html) module"]
pub struct CDCONF_SPEC;
impl crate::RegisterSpec for CDCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdconf::R](R) reader structure"]
impl crate::Readable for CDCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdconf::W](W) writer structure"]
impl crate::Writable for CDCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDCONF to value 0"]
impl crate::Resettable for CDCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
