#[doc = "Register `TSUTIMERINCRSUBNSEC` reader"]
pub struct R(crate::R<TSUTIMERINCRSUBNSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSUTIMERINCRSUBNSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSUTIMERINCRSUBNSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSUTIMERINCRSUBNSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSUTIMERINCRSUBNSEC` writer"]
pub struct W(crate::W<TSUTIMERINCRSUBNSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSUTIMERINCRSUBNSEC_SPEC>;
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
impl From<crate::W<TSUTIMERINCRSUBNSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSUTIMERINCRSUBNSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBNSINCR` reader - MSB \\[23:8\\]
of the subscript-ns value"]
pub type SUBNSINCR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBNSINCR` writer - MSB \\[23:8\\]
of the subscript-ns value"]
pub type SUBNSINCR_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERINCRSUBNSEC_SPEC, u16, u16, 16, 0>;
#[doc = "Field `SUBNSINCRLSB` reader - LSB \\[7:0\\]
of the subscript-ns value"]
pub type SUBNSINCRLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBNSINCRLSB` writer - LSB \\[7:0\\]
of the subscript-ns value"]
pub type SUBNSINCRLSB_W<'a> = crate::FieldWriter<'a, u32, TSUTIMERINCRSUBNSEC_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:15 - MSB \\[23:8\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&self) -> SUBNSINCR_R {
        SUBNSINCR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&self) -> SUBNSINCRLSB_R {
        SUBNSINCRLSB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB \\[23:8\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&mut self) -> SUBNSINCR_W {
        SUBNSINCR_W::new(self)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&mut self) -> SUBNSINCRLSB_W {
        SUBNSINCRLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register subscript nsec\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsutimerincrsubnsec](index.html) module"]
pub struct TSUTIMERINCRSUBNSEC_SPEC;
impl crate::RegisterSpec for TSUTIMERINCRSUBNSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsutimerincrsubnsec::R](R) reader structure"]
impl crate::Readable for TSUTIMERINCRSUBNSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsutimerincrsubnsec::W](W) writer structure"]
impl crate::Writable for TSUTIMERINCRSUBNSEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSUTIMERINCRSUBNSEC to value 0"]
impl crate::Resettable for TSUTIMERINCRSUBNSEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
