#[doc = "Register `POINTER` reader"]
pub struct R(crate::R<POINTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POINTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POINTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POINTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POINTER` writer"]
pub struct W(crate::W<POINTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POINTER_SPEC>;
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
impl From<crate::W<POINTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POINTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPPTRA` reader - OpPtrA"]
pub type OPPTRA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPPTRA` writer - OpPtrA"]
pub type OPPTRA_W<'a> = crate::FieldWriter<'a, u32, POINTER_SPEC, u8, u8, 4, 0>;
#[doc = "Field `OPPTRB` reader - OpPtrB"]
pub type OPPTRB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPPTRB` writer - OpPtrB"]
pub type OPPTRB_W<'a> = crate::FieldWriter<'a, u32, POINTER_SPEC, u8, u8, 4, 8>;
#[doc = "Field `OPPTRC` reader - OpPtrC"]
pub type OPPTRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPPTRC` writer - OpPtrC"]
pub type OPPTRC_W<'a> = crate::FieldWriter<'a, u32, POINTER_SPEC, u8, u8, 4, 16>;
#[doc = "Field `OPPTRN` reader - OpPtrN"]
pub type OPPTRN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPPTRN` writer - OpPtrN"]
pub type OPPTRN_W<'a> = crate::FieldWriter<'a, u32, POINTER_SPEC, u8, u8, 4, 24>;
impl R {
    #[doc = "Bits 0:3 - OpPtrA"]
    #[inline(always)]
    pub fn opptra(&self) -> OPPTRA_R {
        OPPTRA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - OpPtrB"]
    #[inline(always)]
    pub fn opptrb(&self) -> OPPTRB_R {
        OPPTRB_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - OpPtrC"]
    #[inline(always)]
    pub fn opptrc(&self) -> OPPTRC_R {
        OPPTRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - OpPtrN"]
    #[inline(always)]
    pub fn opptrn(&self) -> OPPTRN_R {
        OPPTRN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - OpPtrA"]
    #[inline(always)]
    pub fn opptra(&mut self) -> OPPTRA_W {
        OPPTRA_W::new(self)
    }
    #[doc = "Bits 8:11 - OpPtrB"]
    #[inline(always)]
    pub fn opptrb(&mut self) -> OPPTRB_W {
        OPPTRB_W::new(self)
    }
    #[doc = "Bits 16:19 - OpPtrC"]
    #[inline(always)]
    pub fn opptrc(&mut self) -> OPPTRC_W {
        OPPTRC_W::new(self)
    }
    #[doc = "Bits 24:27 - OpPtrN"]
    #[inline(always)]
    pub fn opptrn(&mut self) -> OPPTRN_W {
        OPPTRN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pointer](index.html) module"]
pub struct POINTER_SPEC;
impl crate::RegisterSpec for POINTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pointer::R](R) reader structure"]
impl crate::Readable for POINTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pointer::W](W) writer structure"]
impl crate::Writable for POINTER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POINTER to value 0"]
impl crate::Resettable for POINTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
