#[doc = "Register `WRPROTCTRL` reader"]
pub struct R(crate::R<WRPROTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRPROTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRPROTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRPROTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRPROTCTRL` writer"]
pub struct W(crate::W<WRPROTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRPROTCTRL_SPEC>;
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
impl From<crate::W<WRPROTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRPROTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INV` reader - Write Protection Inversion Bit"]
pub type INV_R = crate::BitReader<bool>;
#[doc = "Field `INV` writer - Write Protection Inversion Bit"]
pub type INV_W<'a> = crate::BitWriter<'a, u32, WRPROTCTRL_SPEC, bool, 0>;
#[doc = "Field `ENB` reader - Write Protection Enable Bit"]
pub type ENB_R = crate::BitReader<bool>;
#[doc = "Field `ENB` writer - Write Protection Enable Bit"]
pub type ENB_W<'a> = crate::BitWriter<'a, u32, WRPROTCTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W::new(self)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&mut self) -> ENB_W {
        ENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrprotctrl](index.html) module"]
pub struct WRPROTCTRL_SPEC;
impl crate::RegisterSpec for WRPROTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrprotctrl::R](R) reader structure"]
impl crate::Readable for WRPROTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrprotctrl::W](W) writer structure"]
impl crate::Writable for WRPROTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRPROTCTRL to value 0"]
impl crate::Resettable for WRPROTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
