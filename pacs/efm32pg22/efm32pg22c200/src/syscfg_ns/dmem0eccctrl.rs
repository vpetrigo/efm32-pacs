#[doc = "Register `DMEM0ECCCTRL` reader"]
pub struct R(crate::R<DMEM0ECCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMEM0ECCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMEM0ECCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMEM0ECCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMEM0ECCCTRL` writer"]
pub struct W(crate::W<DMEM0ECCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMEM0ECCCTRL_SPEC>;
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
impl From<crate::W<DMEM0ECCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMEM0ECCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMECCEN` reader - RAM ECC Enable"]
pub type RAMECCEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMECCEN` writer - RAM ECC Enable"]
pub type RAMECCEN_W<'a> = crate::BitWriter<'a, u32, DMEM0ECCCTRL_SPEC, bool, 0>;
#[doc = "Field `RAMECCEWEN` reader - RAM ECC Error Writeback Enable"]
pub type RAMECCEWEN_R = crate::BitReader<bool>;
#[doc = "Field `RAMECCEWEN` writer - RAM ECC Error Writeback Enable"]
pub type RAMECCEWEN_W<'a> = crate::BitWriter<'a, u32, DMEM0ECCCTRL_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - RAM ECC Enable"]
    #[inline(always)]
    pub fn rameccen(&self) -> RAMECCEN_R {
        RAMECCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Error Writeback Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RAMECCEWEN_R {
        RAMECCEWEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Enable"]
    #[inline(always)]
    pub fn rameccen(&mut self) -> RAMECCEN_W {
        RAMECCEN_W::new(self)
    }
    #[doc = "Bit 1 - RAM ECC Error Writeback Enable"]
    #[inline(always)]
    pub fn rameccewen(&mut self) -> RAMECCEWEN_W {
        RAMECCEWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configure to set RAM ECC control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmem0eccctrl](index.html) module"]
pub struct DMEM0ECCCTRL_SPEC;
impl crate::RegisterSpec for DMEM0ECCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmem0eccctrl::R](R) reader structure"]
impl crate::Readable for DMEM0ECCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmem0eccctrl::W](W) writer structure"]
impl crate::Writable for DMEM0ECCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMEM0ECCCTRL to value 0"]
impl crate::Resettable for DMEM0ECCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
