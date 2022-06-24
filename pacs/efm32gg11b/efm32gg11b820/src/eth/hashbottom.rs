#[doc = "Register `HASHBOTTOM` reader"]
pub struct R(crate::R<HASHBOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASHBOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASHBOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASHBOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASHBOTTOM` writer"]
pub struct W(crate::W<HASHBOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASHBOTTOM_SPEC>;
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
impl From<crate::W<HASHBOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASHBOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - The first 32 bits of the hash address register."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - The first 32 bits of the hash address register."]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, HASHBOTTOM_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The first 32 bits of the hash address register."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Register Bottom \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashbottom](index.html) module"]
pub struct HASHBOTTOM_SPEC;
impl crate::RegisterSpec for HASHBOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hashbottom::R](R) reader structure"]
impl crate::Readable for HASHBOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hashbottom::W](W) writer structure"]
impl crate::Writable for HASHBOTTOM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASHBOTTOM to value 0"]
impl crate::Resettable for HASHBOTTOM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
