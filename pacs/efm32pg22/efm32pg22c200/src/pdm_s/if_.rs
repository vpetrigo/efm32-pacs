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
#[doc = "Field `DV` reader - Data Valid Interrupt Flag"]
pub type DV_R = crate::BitReader<bool>;
#[doc = "Field `DV` writer - Data Valid Interrupt Flag"]
pub type DV_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `DVL` reader - Data Valid Level Interrupt Flag"]
pub type DVL_R = crate::BitReader<bool>;
#[doc = "Field `DVL` writer - Data Valid Level Interrupt Flag"]
pub type DVL_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `OF` reader - FIFO Overflow Interrupt Flag"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - FIFO Overflow Interrupt Flag"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `UF` reader - FIFO Undeflow Interrupt Flag"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - FIFO Undeflow Interrupt Flag"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn dvl(&self) -> DVL_R {
        DVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO Undeflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Valid Interrupt Flag"]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W {
        DV_W::new(self)
    }
    #[doc = "Bit 1 - Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DVL_W {
        DVL_W::new(self)
    }
    #[doc = "Bit 2 - FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 3 - FIFO Undeflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
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
