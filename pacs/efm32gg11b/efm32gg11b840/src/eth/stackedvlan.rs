#[doc = "Register `STACKEDVLAN` reader"]
pub struct R(crate::R<STACKEDVLAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STACKEDVLAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STACKEDVLAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STACKEDVLAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STACKEDVLAN` writer"]
pub struct W(crate::W<STACKEDVLAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STACKEDVLAN_SPEC>;
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
impl From<crate::W<STACKEDVLAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STACKEDVLAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - User defined VLAN_TYPE field"]
pub type MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH` writer - User defined VLAN_TYPE field"]
pub type MATCH_W<'a> = crate::FieldWriter<'a, u32, STACKEDVLAN_SPEC, u16, u16, 16, 0>;
#[doc = "Field `ENBPROCESSING` reader - Enable stacked VLAN processing mode"]
pub type ENBPROCESSING_R = crate::BitReader<bool>;
#[doc = "Field `ENBPROCESSING` writer - Enable stacked VLAN processing mode"]
pub type ENBPROCESSING_W<'a> = crate::BitWriter<'a, u32, STACKEDVLAN_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&self) -> ENBPROCESSING_R {
        ENBPROCESSING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W::new(self)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&mut self) -> ENBPROCESSING_W {
        ENBPROCESSING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stackedvlan](index.html) module"]
pub struct STACKEDVLAN_SPEC;
impl crate::RegisterSpec for STACKEDVLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stackedvlan::R](R) reader structure"]
impl crate::Readable for STACKEDVLAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stackedvlan::W](W) writer structure"]
impl crate::Writable for STACKEDVLAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STACKEDVLAN to value 0"]
impl crate::Resettable for STACKEDVLAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
