#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - Prescalar Setting for PDM sample"]
pub type PRESC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESC` writer - Prescalar Setting for PDM sample"]
pub type PRESC_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u16, u16, 10, 0>;
#[doc = "Field `DLYMUXSEL` reader - Data delay buffer mux selection"]
pub type DLYMUXSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYMUXSEL` writer - Data delay buffer mux selection"]
pub type DLYMUXSEL_W<'a> = crate::FieldWriter<'a, u32, CFG1_SPEC, u8, u8, 2, 24>;
impl R {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 24:25 - Data delay buffer mux selection"]
    #[inline(always)]
    pub fn dlymuxsel(&self) -> DLYMUXSEL_R {
        DLYMUXSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Prescalar Setting for PDM sample"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W::new(self)
    }
    #[doc = "Bits 24:25 - Data delay buffer mux selection"]
    #[inline(always)]
    pub fn dlymuxsel(&mut self) -> DLYMUXSEL_W {
        DLYMUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
