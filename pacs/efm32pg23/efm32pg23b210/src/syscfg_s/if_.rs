#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW0` reader - Software Interrupt Flag"]
pub type SW0_R = crate::BitReader<bool>;
#[doc = "Field `SW0` writer - Software Interrupt Flag"]
pub type SW0_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `SW1` reader - Software Interrupt Flag"]
pub type SW1_R = crate::BitReader<bool>;
#[doc = "Field `SW1` writer - Software Interrupt Flag"]
pub type SW1_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `SW2` reader - Software Interrupt Flag"]
pub type SW2_R = crate::BitReader<bool>;
#[doc = "Field `SW2` writer - Software Interrupt Flag"]
pub type SW2_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `SW3` reader - Software Interrupt Flag"]
pub type SW3_R = crate::BitReader<bool>;
#[doc = "Field `SW3` writer - Software Interrupt Flag"]
pub type SW3_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 3>;
#[doc = "Field `FPIOC` reader - FPU Invalid Operation interrupt flag"]
pub type FPIOC_R = crate::BitReader<bool>;
#[doc = "Field `FPIOC` writer - FPU Invalid Operation interrupt flag"]
pub type FPIOC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 8>;
#[doc = "Field `FPDZC` reader - FPU Divide by zero interrupt flag"]
pub type FPDZC_R = crate::BitReader<bool>;
#[doc = "Field `FPDZC` writer - FPU Divide by zero interrupt flag"]
pub type FPDZC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 9>;
#[doc = "Field `FPUFC` reader - FPU Underflow interrupt flag"]
pub type FPUFC_R = crate::BitReader<bool>;
#[doc = "Field `FPUFC` writer - FPU Underflow interrupt flag"]
pub type FPUFC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 10>;
#[doc = "Field `FPOFC` reader - FPU Overflow interrupt flag"]
pub type FPOFC_R = crate::BitReader<bool>;
#[doc = "Field `FPOFC` writer - FPU Overflow interrupt flag"]
pub type FPOFC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 11>;
#[doc = "Field `FPIDC` reader - FPU Input denormal interrupt flag"]
pub type FPIDC_R = crate::BitReader<bool>;
#[doc = "Field `FPIDC` writer - FPU Input denormal interrupt flag"]
pub type FPIDC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 12>;
#[doc = "Field `FPIXC` reader - FPU Inexact interrupt flag"]
pub type FPIXC_R = crate::BitReader<bool>;
#[doc = "Field `FPIXC` writer - FPU Inexact interrupt flag"]
pub type FPIXC_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 13>;
#[doc = "Field `SEQRAMERR1B` reader - SEQRAM Error 1-bit Interrupt Flag"]
pub type SEQRAMERR1B_R = crate::BitReader<bool>;
#[doc = "Field `SEQRAMERR1B` writer - SEQRAM Error 1-bit Interrupt Flag"]
pub type SEQRAMERR1B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 24>;
#[doc = "Field `SEQRAMERR2B` reader - SEQRAM Error 2-bit Interrupt Flag"]
pub type SEQRAMERR2B_R = crate::BitReader<bool>;
#[doc = "Field `SEQRAMERR2B` writer - SEQRAM Error 2-bit Interrupt Flag"]
pub type SEQRAMERR2B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 25>;
#[doc = "Field `FRCRAMERR1B` reader - FRCRAM Error 1-bit Interrupt Flag"]
pub type FRCRAMERR1B_R = crate::BitReader<bool>;
#[doc = "Field `FRCRAMERR1B` writer - FRCRAM Error 1-bit Interrupt Flag"]
pub type FRCRAMERR1B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 28>;
#[doc = "Field `FRCRAMERR2B` reader - FRCRAM Error 2-bit Interrupt Flag"]
pub type FRCRAMERR2B_R = crate::BitReader<bool>;
#[doc = "Field `FRCRAMERR2B` writer - FRCRAM Error 2-bit Interrupt Flag"]
pub type FRCRAMERR2B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw2(&self) -> SW2_R {
        SW2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw3(&self) -> SW3_R {
        SW3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - FPU Invalid Operation interrupt flag"]
    #[inline(always)]
    pub fn fpioc(&self) -> FPIOC_R {
        FPIOC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPU Divide by zero interrupt flag"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FPDZC_R {
        FPDZC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPU Underflow interrupt flag"]
    #[inline(always)]
    pub fn fpufc(&self) -> FPUFC_R {
        FPUFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FPU Overflow interrupt flag"]
    #[inline(always)]
    pub fn fpofc(&self) -> FPOFC_R {
        FPOFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FPU Input denormal interrupt flag"]
    #[inline(always)]
    pub fn fpidc(&self) -> FPIDC_R {
        FPIDC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FPU Inexact interrupt flag"]
    #[inline(always)]
    pub fn fpixc(&self) -> FPIXC_R {
        FPIXC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - SEQRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&self) -> SEQRAMERR1B_R {
        SEQRAMERR1B_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SEQRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&self) -> SEQRAMERR2B_R {
        SEQRAMERR2B_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - FRCRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&self) -> FRCRAMERR1B_R {
        FRCRAMERR1B_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FRCRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&self) -> FRCRAMERR2B_R {
        FRCRAMERR2B_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw0(&mut self) -> SW0_W {
        SW0_W::new(self)
    }
    #[doc = "Bit 1 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W {
        SW1_W::new(self)
    }
    #[doc = "Bit 2 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw2(&mut self) -> SW2_W {
        SW2_W::new(self)
    }
    #[doc = "Bit 3 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw3(&mut self) -> SW3_W {
        SW3_W::new(self)
    }
    #[doc = "Bit 8 - FPU Invalid Operation interrupt flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FPIOC_W {
        FPIOC_W::new(self)
    }
    #[doc = "Bit 9 - FPU Divide by zero interrupt flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FPDZC_W {
        FPDZC_W::new(self)
    }
    #[doc = "Bit 10 - FPU Underflow interrupt flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FPUFC_W {
        FPUFC_W::new(self)
    }
    #[doc = "Bit 11 - FPU Overflow interrupt flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FPOFC_W {
        FPOFC_W::new(self)
    }
    #[doc = "Bit 12 - FPU Input denormal interrupt flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FPIDC_W {
        FPIDC_W::new(self)
    }
    #[doc = "Bit 13 - FPU Inexact interrupt flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FPIXC_W {
        FPIXC_W::new(self)
    }
    #[doc = "Bit 24 - SEQRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&mut self) -> SEQRAMERR1B_W {
        SEQRAMERR1B_W::new(self)
    }
    #[doc = "Bit 25 - SEQRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&mut self) -> SEQRAMERR2B_W {
        SEQRAMERR2B_W::new(self)
    }
    #[doc = "Bit 28 - FRCRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&mut self) -> FRCRAMERR1B_W {
        FRCRAMERR1B_W::new(self)
    }
    #[doc = "Bit 29 - FRCRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&mut self) -> FRCRAMERR2B_W {
        FRCRAMERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to get system status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
