#[doc = "Register `TESTLOCK` reader"]
pub struct R(crate::R<TESTLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TESTLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TESTLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TESTLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TESTLOCK` writer"]
pub struct W(crate::W<TESTLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TESTLOCK_SPEC>;
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
impl From<crate::W<TESTLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TESTLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Configuration Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum LOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LOCKKEY` reader - Configuration Lock Key"]
pub type LOCKKEY_R = crate::FieldReader<u16, LOCKKEY_A>;
impl LOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKKEY_A> {
        match self.bits {
            0 => Some(LOCKKEY_A::UNLOCKED),
            1 => Some(LOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY_A::LOCKED
    }
}
#[doc = "Field `LOCKKEY` writer - Configuration Lock Key"]
pub type LOCKKEY_W<'a> = crate::FieldWriter<'a, u32, TESTLOCK_SPEC, u16, LOCKKEY_A, 16, 0>;
impl<'a> LOCKKEY_W<'a> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&mut self) -> LOCKKEY_W {
        LOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [testlock](index.html) module"]
pub struct TESTLOCK_SPEC;
impl crate::RegisterSpec for TESTLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [testlock::R](R) reader structure"]
impl crate::Readable for TESTLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [testlock::W](W) writer structure"]
impl crate::Writable for TESTLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TESTLOCK to value 0"]
impl crate::Resettable for TESTLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
