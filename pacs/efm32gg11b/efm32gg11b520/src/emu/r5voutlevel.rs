#[doc = "Register `R5VOUTLEVEL` reader"]
pub struct R(crate::R<R5VOUTLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R5VOUTLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R5VOUTLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R5VOUTLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R5VOUTLEVEL` writer"]
pub struct W(crate::W<R5VOUTLEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R5VOUTLEVEL_SPEC>;
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
impl From<crate::W<R5VOUTLEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R5VOUTLEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTLEVEL` reader - 5V Regulator Voltage"]
pub type OUTLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTLEVEL` writer - 5V Regulator Voltage"]
pub type OUTLEVEL_W<'a> = crate::FieldWriter<'a, u32, R5VOUTLEVEL_SPEC, u8, u8, 4, 0>;
impl R {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&self) -> OUTLEVEL_R {
        OUTLEVEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 5V Regulator Voltage"]
    #[inline(always)]
    pub fn outlevel(&mut self) -> OUTLEVEL_W {
        OUTLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "5V Regulator Voltage Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r5voutlevel](index.html) module"]
pub struct R5VOUTLEVEL_SPEC;
impl crate::RegisterSpec for R5VOUTLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r5voutlevel::R](R) reader structure"]
impl crate::Readable for R5VOUTLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r5voutlevel::W](W) writer structure"]
impl crate::Writable for R5VOUTLEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R5VOUTLEVEL to value 0x01"]
impl crate::Resettable for R5VOUTLEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
