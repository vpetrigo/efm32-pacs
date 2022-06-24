#[doc = "Register `WRITECTRL` reader"]
pub struct R(crate::R<WRITECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECTRL` writer"]
pub struct W(crate::W<WRITECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECTRL_SPEC>;
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
impl From<crate::W<WRITECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WREN` reader - Enable Write/Erase Controller"]
pub type WREN_R = crate::BitReader<bool>;
#[doc = "Field `WREN` writer - Enable Write/Erase Controller"]
pub type WREN_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 0>;
#[doc = "Field `IRQERASEABORT` reader - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_R = crate::BitReader<bool>;
#[doc = "Field `IRQERASEABORT` writer - Abort Page Erase on Interrupt"]
pub type IRQERASEABORT_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 1>;
#[doc = "Field `WDOUBLE` reader - Write two words at a time"]
pub type WDOUBLE_R = crate::BitReader<bool>;
#[doc = "Field `WDOUBLE` writer - Write two words at a time"]
pub type WDOUBLE_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 2>;
#[doc = "Field `LPWRITE` reader - Low-Power Erase"]
pub type LPWRITE_R = crate::BitReader<bool>;
#[doc = "Field `LPWRITE` writer - Low-Power Erase"]
pub type LPWRITE_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 3>;
#[doc = "Field `LPERASE` reader - Low-Power Erase"]
pub type LPERASE_R = crate::BitReader<bool>;
#[doc = "Field `LPERASE` writer - Low-Power Erase"]
pub type LPERASE_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 4>;
#[doc = "Field `RWWEN` reader - Read-While-Write Enable"]
pub type RWWEN_R = crate::BitReader<bool>;
#[doc = "Field `RWWEN` writer - Read-While-Write Enable"]
pub type RWWEN_W<'a> = crate::BitWriter<'a, u32, WRITECTRL_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&self) -> WREN_R {
        WREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&self) -> IRQERASEABORT_R {
        IRQERASEABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write two words at a time"]
    #[inline(always)]
    pub fn wdouble(&self) -> WDOUBLE_R {
        WDOUBLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Low-Power Erase"]
    #[inline(always)]
    pub fn lpwrite(&self) -> LPWRITE_R {
        LPWRITE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Low-Power Erase"]
    #[inline(always)]
    pub fn lperase(&self) -> LPERASE_R {
        LPERASE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read-While-Write Enable"]
    #[inline(always)]
    pub fn rwwen(&self) -> RWWEN_R {
        RWWEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write/Erase Controller"]
    #[inline(always)]
    pub fn wren(&mut self) -> WREN_W {
        WREN_W::new(self)
    }
    #[doc = "Bit 1 - Abort Page Erase on Interrupt"]
    #[inline(always)]
    pub fn irqeraseabort(&mut self) -> IRQERASEABORT_W {
        IRQERASEABORT_W::new(self)
    }
    #[doc = "Bit 2 - Write two words at a time"]
    #[inline(always)]
    pub fn wdouble(&mut self) -> WDOUBLE_W {
        WDOUBLE_W::new(self)
    }
    #[doc = "Bit 3 - Low-Power Erase"]
    #[inline(always)]
    pub fn lpwrite(&mut self) -> LPWRITE_W {
        LPWRITE_W::new(self)
    }
    #[doc = "Bit 4 - Low-Power Erase"]
    #[inline(always)]
    pub fn lperase(&mut self) -> LPERASE_W {
        LPERASE_W::new(self)
    }
    #[doc = "Bit 5 - Read-While-Write Enable"]
    #[inline(always)]
    pub fn rwwen(&mut self) -> RWWEN_W {
        RWWEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writectrl](index.html) module"]
pub struct WRITECTRL_SPEC;
impl crate::RegisterSpec for WRITECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writectrl::R](R) reader structure"]
impl crate::Readable for WRITECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writectrl::W](W) writer structure"]
impl crate::Writable for WRITECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITECTRL to value 0"]
impl crate::Resettable for WRITECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
