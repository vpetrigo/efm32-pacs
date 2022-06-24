#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORCEEN` reader - Force Enable"]
pub type FORCEEN_R = crate::BitReader<bool>;
#[doc = "Field `FORCEEN` writer - Force Enable"]
pub type FORCEEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `DISONDEMAND` reader - Disable On-demand Mode"]
pub type DISONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `DISONDEMAND` writer - Disable On-demand Mode"]
pub type DISONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `KEEPWARM` reader - Keep Warm"]
pub type KEEPWARM_R = crate::BitReader<bool>;
#[doc = "Field `KEEPWARM` writer - Keep Warm"]
pub type KEEPWARM_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Force XI Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEXI2GNDANA_A {
    #[doc = "0: Disabled (not pulled)"]
    DISABLE = 0,
    #[doc = "1: Enabled (pulled)"]
    ENABLE = 1,
}
impl From<FORCEXI2GNDANA_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEXI2GNDANA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXI2GNDANA` reader - Force XI Pin to Ground"]
pub type FORCEXI2GNDANA_R = crate::BitReader<FORCEXI2GNDANA_A>;
impl FORCEXI2GNDANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEXI2GNDANA_A {
        match self.bits {
            false => FORCEXI2GNDANA_A::DISABLE,
            true => FORCEXI2GNDANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCEXI2GNDANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCEXI2GNDANA_A::ENABLE
    }
}
#[doc = "Field `FORCEXI2GNDANA` writer - Force XI Pin to Ground"]
pub type FORCEXI2GNDANA_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, FORCEXI2GNDANA_A, 4>;
impl<'a> FORCEXI2GNDANA_W<'a> {
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCEXI2GNDANA_A::DISABLE)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCEXI2GNDANA_A::ENABLE)
    }
}
#[doc = "Force XO Pin to Ground\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEXO2GNDANA_A {
    #[doc = "0: Disabled (not pulled)"]
    DISABLE = 0,
    #[doc = "1: Enabled (pulled)"]
    ENABLE = 1,
}
impl From<FORCEXO2GNDANA_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEXO2GNDANA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCEXO2GNDANA` reader - Force XO Pin to Ground"]
pub type FORCEXO2GNDANA_R = crate::BitReader<FORCEXO2GNDANA_A>;
impl FORCEXO2GNDANA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEXO2GNDANA_A {
        match self.bits {
            false => FORCEXO2GNDANA_A::DISABLE,
            true => FORCEXO2GNDANA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FORCEXO2GNDANA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FORCEXO2GNDANA_A::ENABLE
    }
}
#[doc = "Field `FORCEXO2GNDANA` writer - Force XO Pin to Ground"]
pub type FORCEXO2GNDANA_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, FORCEXO2GNDANA_A, 5>;
impl<'a> FORCEXO2GNDANA_W<'a> {
    #[doc = "Disabled (not pulled)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FORCEXO2GNDANA_A::DISABLE)
    }
    #[doc = "Enabled (pulled)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FORCEXO2GNDANA_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> FORCEEN_R {
        FORCEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable On-demand Mode"]
    #[inline(always)]
    pub fn disondemand(&self) -> DISONDEMAND_R {
        DISONDEMAND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KEEPWARM_R {
        KEEPWARM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&self) -> FORCEXI2GNDANA_R {
        FORCEXI2GNDANA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&self) -> FORCEXO2GNDANA_R {
        FORCEXO2GNDANA_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> FORCEEN_W {
        FORCEEN_W::new(self)
    }
    #[doc = "Bit 1 - Disable On-demand Mode"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DISONDEMAND_W {
        DISONDEMAND_W::new(self)
    }
    #[doc = "Bit 2 - Keep Warm"]
    #[inline(always)]
    pub fn keepwarm(&mut self) -> KEEPWARM_W {
        KEEPWARM_W::new(self)
    }
    #[doc = "Bit 4 - Force XI Pin to Ground"]
    #[inline(always)]
    pub fn forcexi2gndana(&mut self) -> FORCEXI2GNDANA_W {
        FORCEXI2GNDANA_W::new(self)
    }
    #[doc = "Bit 5 - Force XO Pin to Ground"]
    #[inline(always)]
    pub fn forcexo2gndana(&mut self) -> FORCEXO2GNDANA_W {
        FORCEXO2GNDANA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x02"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
