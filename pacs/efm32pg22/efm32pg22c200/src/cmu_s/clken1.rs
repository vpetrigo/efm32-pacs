#[doc = "Register `CLKEN1` reader"]
pub struct R(crate::R<CLKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKEN1` writer"]
pub struct W(crate::W<CLKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKEN1_SPEC>;
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
impl From<crate::W<CLKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTOACC` reader - Enable Bus Clock"]
pub type CRYPTOACC_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTOACC` writer - Enable Bus Clock"]
pub type CRYPTOACC_W<'a> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, 13>;
#[doc = "Field `SMU` reader - Enable Bus Clock"]
pub type SMU_R = crate::BitReader<bool>;
#[doc = "Field `SMU` writer - Enable Bus Clock"]
pub type SMU_W<'a> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, 15>;
#[doc = "Field `ICACHE0` reader - Enable Bus Clock"]
pub type ICACHE0_R = crate::BitReader<bool>;
#[doc = "Field `ICACHE0` writer - Enable Bus Clock"]
pub type ICACHE0_W<'a> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, 16>;
#[doc = "Field `MSC` reader - Enable Bus Clock"]
pub type MSC_R = crate::BitReader<bool>;
#[doc = "Field `MSC` writer - Enable Bus Clock"]
pub type MSC_W<'a> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, 17>;
#[doc = "Field `TIMER4` reader - Enable Bus Clock"]
pub type TIMER4_R = crate::BitReader<bool>;
#[doc = "Field `TIMER4` writer - Enable Bus Clock"]
pub type TIMER4_W<'a> = crate::BitWriter<'a, u32, CLKEN1_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn cryptoacc(&self) -> CRYPTOACC_R {
        CRYPTOACC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&self) -> SMU_R {
        SMU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&self) -> ICACHE0_R {
        ICACHE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&self) -> MSC_R {
        MSC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Enable Bus Clock"]
    #[inline(always)]
    pub fn cryptoacc(&mut self) -> CRYPTOACC_W {
        CRYPTOACC_W::new(self)
    }
    #[doc = "Bit 15 - Enable Bus Clock"]
    #[inline(always)]
    pub fn smu(&mut self) -> SMU_W {
        SMU_W::new(self)
    }
    #[doc = "Bit 16 - Enable Bus Clock"]
    #[inline(always)]
    pub fn icache0(&mut self) -> ICACHE0_W {
        ICACHE0_W::new(self)
    }
    #[doc = "Bit 17 - Enable Bus Clock"]
    #[inline(always)]
    pub fn msc(&mut self) -> MSC_W {
        MSC_W::new(self)
    }
    #[doc = "Bit 18 - Enable Bus Clock"]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clken1](index.html) module"]
pub struct CLKEN1_SPEC;
impl crate::RegisterSpec for CLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clken1::R](R) reader structure"]
impl crate::Readable for CLKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clken1::W](W) writer structure"]
impl crate::Writable for CLKEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKEN1 to value 0"]
impl crate::Resettable for CLKEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
