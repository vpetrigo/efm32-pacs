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
#[doc = "Field `AHB0ERR1B` reader - AHB0 1-bit ECC Error Interrupt Flag"]
pub type AHB0ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `AHB0ERR1B` writer - AHB0 1-bit ECC Error Interrupt Flag"]
pub type AHB0ERR1B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `AHB1ERR1B` reader - AHB1 1-bit ECC Error Interrupt Flag"]
pub type AHB1ERR1B_R = crate::BitReader<bool>;
#[doc = "Field `AHB1ERR1B` writer - AHB1 1-bit ECC Error Interrupt Flag"]
pub type AHB1ERR1B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `AHB0ERR2B` reader - AHB0 2-bit ECC Error Interrupt Flag"]
pub type AHB0ERR2B_R = crate::BitReader<bool>;
#[doc = "Field `AHB0ERR2B` writer - AHB0 2-bit ECC Error Interrupt Flag"]
pub type AHB0ERR2B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 4>;
#[doc = "Field `AHB1ERR2B` reader - AHB1 2-bit ECC Error Interrupt Flag"]
pub type AHB1ERR2B_R = crate::BitReader<bool>;
#[doc = "Field `AHB1ERR2B` writer - AHB1 2-bit ECC Error Interrupt Flag"]
pub type AHB1ERR2B_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb0err1b(&self) -> AHB0ERR1B_R {
        AHB0ERR1B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb1err1b(&self) -> AHB1ERR1B_R {
        AHB1ERR1B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb0err2b(&self) -> AHB0ERR2B_R {
        AHB0ERR2B_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb1err2b(&self) -> AHB1ERR2B_R {
        AHB1ERR2B_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AHB0 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb0err1b(&mut self) -> AHB0ERR1B_W {
        AHB0ERR1B_W::new(self)
    }
    #[doc = "Bit 1 - AHB1 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb1err1b(&mut self) -> AHB1ERR1B_W {
        AHB1ERR1B_W::new(self)
    }
    #[doc = "Bit 4 - AHB0 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb0err2b(&mut self) -> AHB0ERR2B_W {
        AHB0ERR2B_W::new(self)
    }
    #[doc = "Bit 5 - AHB1 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ahb1err2b(&mut self) -> AHB1ERR2B_W {
        AHB1ERR2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
