#[doc = "Register `BACFG` reader"]
pub struct R(crate::R<BACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BACFG` writer"]
pub struct W(crate::W<BACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BACFG_SPEC>;
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
impl From<crate::W<BACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASTATETOP` reader - ASTATE top cnt"]
pub type ASTATETOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ASTATETOP` writer - ASTATE top cnt"]
pub type ASTATETOP_W<'a> = crate::FieldWriter<'a, u32, BACFG_SPEC, u8, u8, 3, 0>;
#[doc = "Frame Counter Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FCPRESC_A {
    #[doc = "0: every frame clock"]
    DIV1 = 0,
    #[doc = "1: every 2nd frame clock"]
    DIV2 = 1,
    #[doc = "2: every 4th frame clock"]
    DIV4 = 2,
    #[doc = "3: every 8th frame clock"]
    DIV8 = 3,
}
impl From<FCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: FCPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FCPRESC` reader - Frame Counter Prescaler"]
pub type FCPRESC_R = crate::FieldReader<u8, FCPRESC_A>;
impl FCPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCPRESC_A {
        match self.bits {
            0 => FCPRESC_A::DIV1,
            1 => FCPRESC_A::DIV2,
            2 => FCPRESC_A::DIV4,
            3 => FCPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FCPRESC_A::DIV8
    }
}
#[doc = "Field `FCPRESC` writer - Frame Counter Prescaler"]
pub type FCPRESC_W<'a> = crate::FieldWriterSafe<'a, u32, BACFG_SPEC, u8, FCPRESC_A, 2, 16>;
impl<'a> FCPRESC_W<'a> {
    #[doc = "every frame clock"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV1)
    }
    #[doc = "every 2nd frame clock"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV2)
    }
    #[doc = "every 4th frame clock"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV4)
    }
    #[doc = "every 8th frame clock"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FCPRESC_A::DIV8)
    }
}
#[doc = "Field `FCTOP` reader - Frame Counter Top"]
pub type FCTOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCTOP` writer - Frame Counter Top"]
pub type FCTOP_W<'a> = crate::FieldWriter<'a, u32, BACFG_SPEC, u8, u8, 6, 18>;
impl R {
    #[doc = "Bits 0:2 - ASTATE top cnt"]
    #[inline(always)]
    pub fn astatetop(&self) -> ASTATETOP_R {
        ASTATETOP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&self) -> FCPRESC_R {
        FCPRESC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Frame Counter Top"]
    #[inline(always)]
    pub fn fctop(&self) -> FCTOP_R {
        FCTOP_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - ASTATE top cnt"]
    #[inline(always)]
    pub fn astatetop(&mut self) -> ASTATETOP_W {
        ASTATETOP_W::new(self)
    }
    #[doc = "Bits 16:17 - Frame Counter Prescaler"]
    #[inline(always)]
    pub fn fcpresc(&mut self) -> FCPRESC_W {
        FCPRESC_W::new(self)
    }
    #[doc = "Bits 18:23 - Frame Counter Top"]
    #[inline(always)]
    pub fn fctop(&mut self) -> FCTOP_W {
        FCTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bacfg](index.html) module"]
pub struct BACFG_SPEC;
impl crate::RegisterSpec for BACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bacfg::R](R) reader structure"]
impl crate::Readable for BACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bacfg::W](W) writer structure"]
impl crate::Writable for BACFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BACFG to value 0x07"]
impl crate::Resettable for BACFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
