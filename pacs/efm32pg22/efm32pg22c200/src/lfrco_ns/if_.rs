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
#[doc = "Field `RDY` reader - Ready Flag"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - Ready Flag"]
pub type RDY_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 0>;
#[doc = "Field `POSEDGE` reader - Rising Edge Flag"]
pub type POSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `POSEDGE` writer - Rising Edge Flag"]
pub type POSEDGE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 1>;
#[doc = "Field `NEGEDGE` reader - Falling Edge Flag"]
pub type NEGEDGE_R = crate::BitReader<bool>;
#[doc = "Field `NEGEDGE` writer - Falling Edge Flag"]
pub type NEGEDGE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 2>;
#[doc = "Field `TCDONE` reader - Temperature Check Done Flag"]
pub type TCDONE_R = crate::BitReader<bool>;
#[doc = "Field `TCDONE` writer - Temperature Check Done Flag"]
pub type TCDONE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 8>;
#[doc = "Field `CALDONE` reader - Calibration Done Flag"]
pub type CALDONE_R = crate::BitReader<bool>;
#[doc = "Field `CALDONE` writer - Calibration Done Flag"]
pub type CALDONE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 9>;
#[doc = "Field `TEMPCHANGE` reader - Temperature Change Flag"]
pub type TEMPCHANGE_R = crate::BitReader<bool>;
#[doc = "Field `TEMPCHANGE` writer - Temperature Change Flag"]
pub type TEMPCHANGE_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 10>;
#[doc = "Field `SCHEDERR` reader - Scheduling Error Flag"]
pub type SCHEDERR_R = crate::BitReader<bool>;
#[doc = "Field `SCHEDERR` writer - Scheduling Error Flag"]
pub type SCHEDERR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 16>;
#[doc = "Field `TCOOR` reader - Temperature Check Out Of Range Flag"]
pub type TCOOR_R = crate::BitReader<bool>;
#[doc = "Field `TCOOR` writer - Temperature Check Out Of Range Flag"]
pub type TCOOR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 17>;
#[doc = "Field `CALOOR` reader - Calibration Out Of Range Flag"]
pub type CALOOR_R = crate::BitReader<bool>;
#[doc = "Field `CALOOR` writer - Calibration Out Of Range Flag"]
pub type CALOOR_W<'a> = crate::BitWriter<'a, u32, IF_SPEC, bool, 18>;
impl R {
    #[doc = "Bit 0 - Ready Flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising Edge Flag"]
    #[inline(always)]
    pub fn posedge(&self) -> POSEDGE_R {
        POSEDGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling Edge Flag"]
    #[inline(always)]
    pub fn negedge(&self) -> NEGEDGE_R {
        NEGEDGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Temperature Check Done Flag"]
    #[inline(always)]
    pub fn tcdone(&self) -> TCDONE_R {
        TCDONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Calibration Done Flag"]
    #[inline(always)]
    pub fn caldone(&self) -> CALDONE_R {
        CALDONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Temperature Change Flag"]
    #[inline(always)]
    pub fn tempchange(&self) -> TEMPCHANGE_R {
        TEMPCHANGE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Scheduling Error Flag"]
    #[inline(always)]
    pub fn schederr(&self) -> SCHEDERR_R {
        SCHEDERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Temperature Check Out Of Range Flag"]
    #[inline(always)]
    pub fn tcoor(&self) -> TCOOR_R {
        TCOOR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Calibration Out Of Range Flag"]
    #[inline(always)]
    pub fn caloor(&self) -> CALOOR_R {
        CALOOR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Flag"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RDY_W {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - Rising Edge Flag"]
    #[inline(always)]
    pub fn posedge(&mut self) -> POSEDGE_W {
        POSEDGE_W::new(self)
    }
    #[doc = "Bit 2 - Falling Edge Flag"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NEGEDGE_W {
        NEGEDGE_W::new(self)
    }
    #[doc = "Bit 8 - Temperature Check Done Flag"]
    #[inline(always)]
    pub fn tcdone(&mut self) -> TCDONE_W {
        TCDONE_W::new(self)
    }
    #[doc = "Bit 9 - Calibration Done Flag"]
    #[inline(always)]
    pub fn caldone(&mut self) -> CALDONE_W {
        CALDONE_W::new(self)
    }
    #[doc = "Bit 10 - Temperature Change Flag"]
    #[inline(always)]
    pub fn tempchange(&mut self) -> TEMPCHANGE_W {
        TEMPCHANGE_W::new(self)
    }
    #[doc = "Bit 16 - Scheduling Error Flag"]
    #[inline(always)]
    pub fn schederr(&mut self) -> SCHEDERR_W {
        SCHEDERR_W::new(self)
    }
    #[doc = "Bit 17 - Temperature Check Out Of Range Flag"]
    #[inline(always)]
    pub fn tcoor(&mut self) -> TCOOR_W {
        TCOOR_W::new(self)
    }
    #[doc = "Bit 18 - Calibration Out Of Range Flag"]
    #[inline(always)]
    pub fn caloor(&mut self) -> CALOOR_W {
        CALOOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
