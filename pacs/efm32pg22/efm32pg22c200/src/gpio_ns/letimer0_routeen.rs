#[doc = "Register `LETIMER0_ROUTEEN` reader"]
pub struct R(crate::R<LETIMER0_ROUTEEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LETIMER0_ROUTEEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LETIMER0_ROUTEEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LETIMER0_ROUTEEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LETIMER0_ROUTEEN` writer"]
pub struct W(crate::W<LETIMER0_ROUTEEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LETIMER0_ROUTEEN_SPEC>;
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
impl From<crate::W<LETIMER0_ROUTEEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LETIMER0_ROUTEEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0PEN` reader - OUT0 pin enable control bit"]
pub type OUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `OUT0PEN` writer - OUT0 pin enable control bit"]
pub type OUT0PEN_W<'a> = crate::BitWriter<'a, u32, LETIMER0_ROUTEEN_SPEC, bool, 0>;
#[doc = "Field `OUT1PEN` reader - OUT1 pin enable control bit"]
pub type OUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `OUT1PEN` writer - OUT1 pin enable control bit"]
pub type OUT1PEN_W<'a> = crate::BitWriter<'a, u32, LETIMER0_ROUTEEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&self) -> OUT0PEN_R {
        OUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&self) -> OUT1PEN_R {
        OUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> OUT0PEN_W {
        OUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> OUT1PEN_W {
        OUT1PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LETIMER pin enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [letimer0_routeen](index.html) module"]
pub struct LETIMER0_ROUTEEN_SPEC;
impl crate::RegisterSpec for LETIMER0_ROUTEEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [letimer0_routeen::R](R) reader structure"]
impl crate::Readable for LETIMER0_ROUTEEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [letimer0_routeen::W](W) writer structure"]
impl crate::Writable for LETIMER0_ROUTEEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LETIMER0_ROUTEEN to value 0"]
impl crate::Resettable for LETIMER0_ROUTEEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
