#[doc = "Register `BANKSWITCHLOCK` reader"]
pub struct R(crate::R<BANKSWITCHLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BANKSWITCHLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BANKSWITCHLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BANKSWITCHLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BANKSWITCHLOCK` writer"]
pub struct W(crate::W<BANKSWITCHLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BANKSWITCHLOCK_SPEC>;
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
impl From<crate::W<BANKSWITCHLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BANKSWITCHLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Bank Switching Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BANKSWITCHLOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<BANKSWITCHLOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: BANKSWITCHLOCKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BANKSWITCHLOCKKEY` reader - Bank Switching Lock"]
pub type BANKSWITCHLOCKKEY_R = crate::FieldReader<u16, BANKSWITCHLOCKKEY_A>;
impl BANKSWITCHLOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BANKSWITCHLOCKKEY_A> {
        match self.bits {
            0 => Some(BANKSWITCHLOCKKEY_A::UNLOCKED),
            1 => Some(BANKSWITCHLOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::LOCKED
    }
}
#[doc = "Field `BANKSWITCHLOCKKEY` writer - Bank Switching Lock"]
pub type BANKSWITCHLOCKKEY_W<'a> =
    crate::FieldWriter<'a, u32, BANKSWITCHLOCK_SPEC, u16, BANKSWITCHLOCKKEY_A, 16, 0>;
impl<'a> BANKSWITCHLOCKKEY_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(BANKSWITCHLOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&self) -> BANKSWITCHLOCKKEY_R {
        BANKSWITCHLOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&mut self) -> BANKSWITCHLOCKKEY_W {
        BANKSWITCHLOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bank Switching Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bankswitchlock](index.html) module"]
pub struct BANKSWITCHLOCK_SPEC;
impl crate::RegisterSpec for BANKSWITCHLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bankswitchlock::R](R) reader structure"]
impl crate::Readable for BANKSWITCHLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bankswitchlock::W](W) writer structure"]
impl crate::Writable for BANKSWITCHLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BANKSWITCHLOCK to value 0x01"]
impl crate::Resettable for BANKSWITCHLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
