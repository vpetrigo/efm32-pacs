#[doc = "Register `WRTIMING` reader"]
pub struct R(crate::R<WRTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRTIMING` writer"]
pub struct W(crate::W<WRTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRTIMING_SPEC>;
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
impl From<crate::W<WRTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRSETUP` reader - Write Setup Time"]
pub type WRSETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRSETUP` writer - Write Setup Time"]
pub type WRSETUP_W<'a> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 2, 0>;
#[doc = "Field `WRSTRB` reader - Write Strobe Time"]
pub type WRSTRB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRSTRB` writer - Write Strobe Time"]
pub type WRSTRB_W<'a> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 4, 8>;
#[doc = "Field `WRHOLD` reader - Write Hold Time"]
pub type WRHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRHOLD` writer - Write Hold Time"]
pub type WRHOLD_W<'a> = crate::FieldWriter<'a, u32, WRTIMING_SPEC, u8, u8, 2, 16>;
impl R {
    #[doc = "Bits 0:1 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&mut self) -> WRSETUP_W {
        WRSETUP_W::new(self)
    }
    #[doc = "Bits 8:11 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&mut self) -> WRSTRB_W {
        WRSTRB_W::new(self)
    }
    #[doc = "Bits 16:17 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&mut self) -> WRHOLD_W {
        WRHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming](index.html) module"]
pub struct WRTIMING_SPEC;
impl crate::RegisterSpec for WRTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrtiming::R](R) reader structure"]
impl crate::Readable for WRTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrtiming::W](W) writer structure"]
impl crate::Writable for WRTIMING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRTIMING to value 0x0001_0000"]
impl crate::Resettable for WRTIMING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
