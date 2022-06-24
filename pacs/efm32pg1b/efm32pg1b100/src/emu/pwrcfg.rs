#[doc = "Register `PWRCFG` reader"]
pub struct R(crate::R<PWRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCFG` writer"]
pub struct W(crate::W<PWRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCFG_SPEC>;
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
impl From<crate::W<PWRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRCFG_A {
    #[doc = "0: Power up configuration. Works with any external configuration."]
    STARTUP = 0,
    #[doc = "2: Configured: DCDC control logic is enabled."]
    DCDCTODVDD = 2,
}
impl From<PWRCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRCFG` reader - Power Configuration"]
pub type PWRCFG_R = crate::FieldReader<u8, PWRCFG_A>;
impl PWRCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PWRCFG_A> {
        match self.bits {
            0 => Some(PWRCFG_A::STARTUP),
            2 => Some(PWRCFG_A::DCDCTODVDD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STARTUP`"]
    #[inline(always)]
    pub fn is_startup(&self) -> bool {
        *self == PWRCFG_A::STARTUP
    }
    #[doc = "Checks if the value of the field is `DCDCTODVDD`"]
    #[inline(always)]
    pub fn is_dcdctodvdd(&self) -> bool {
        *self == PWRCFG_A::DCDCTODVDD
    }
}
#[doc = "Field `PWRCFG` writer - Power Configuration"]
pub type PWRCFG_W<'a> = crate::FieldWriter<'a, u32, PWRCFG_SPEC, u8, PWRCFG_A, 4, 0>;
impl<'a> PWRCFG_W<'a> {
    #[doc = "Power up configuration. Works with any external configuration."]
    #[inline(always)]
    pub fn startup(self) -> &'a mut W {
        self.variant(PWRCFG_A::STARTUP)
    }
    #[doc = "Configured: DCDC control logic is enabled."]
    #[inline(always)]
    pub fn dcdctodvdd(self) -> &'a mut W {
        self.variant(PWRCFG_A::DCDCTODVDD)
    }
}
impl R {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&self) -> PWRCFG_R {
        PWRCFG_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Power Configuration"]
    #[inline(always)]
    pub fn pwrcfg(&mut self) -> PWRCFG_W {
        PWRCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcfg](index.html) module"]
pub struct PWRCFG_SPEC;
impl crate::RegisterSpec for PWRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcfg::R](R) reader structure"]
impl crate::Readable for PWRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcfg::W](W) writer structure"]
impl crate::Writable for PWRCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCFG to value 0"]
impl crate::Resettable for PWRCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
