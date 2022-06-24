#[doc = "Register `SYSWAKETIME` reader"]
pub struct R(crate::R<SYSWAKETIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSWAKETIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSWAKETIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSWAKETIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSWAKETIME` writer"]
pub struct W(crate::W<SYSWAKETIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSWAKETIME_SPEC>;
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
impl From<crate::W<SYSWAKETIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSWAKETIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSWAKETIME` reader - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SYSWAKETIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYSWAKETIME` writer - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SYSWAKETIME_W<'a> = crate::FieldWriter<'a, u32, SYSWAKETIME_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&self) -> SYSWAKETIME_R {
        SYSWAKETIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&mut self) -> SYSWAKETIME_W {
        SYSWAKETIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System wake time\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syswaketime](index.html) module"]
pub struct SYSWAKETIME_SPEC;
impl crate::RegisterSpec for SYSWAKETIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syswaketime::R](R) reader structure"]
impl crate::Readable for SYSWAKETIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syswaketime::W](W) writer structure"]
impl crate::Writable for SYSWAKETIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSWAKETIME to value 0"]
impl crate::Resettable for SYSWAKETIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
