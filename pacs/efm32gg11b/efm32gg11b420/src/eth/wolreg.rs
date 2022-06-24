#[doc = "Register `WOLREG` reader"]
pub struct R(crate::R<WOLREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOLREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOLREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOLREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOLREG` writer"]
pub struct W(crate::W<WOLREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOLREG_SPEC>;
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
impl From<crate::W<WOLREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOLREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_W<'a> = crate::FieldWriter<'a, u32, WOLREG_SPEC, u16, u16, 16, 0>;
#[doc = "Field `WOLMASK0` reader - Wake on LAN magic packet event enable"]
pub type WOLMASK0_R = crate::BitReader<bool>;
#[doc = "Field `WOLMASK0` writer - Wake on LAN magic packet event enable"]
pub type WOLMASK0_W<'a> = crate::BitWriter<'a, u32, WOLREG_SPEC, bool, 16>;
#[doc = "Field `WOLMASK1` reader - Wake on LAN ARP request event enable"]
pub type WOLMASK1_R = crate::BitReader<bool>;
#[doc = "Field `WOLMASK1` writer - Wake on LAN ARP request event enable"]
pub type WOLMASK1_W<'a> = crate::BitWriter<'a, u32, WOLREG_SPEC, bool, 17>;
#[doc = "Field `WOLMASK2` reader - Wake on LAN specific address register 1 event enable"]
pub type WOLMASK2_R = crate::BitReader<bool>;
#[doc = "Field `WOLMASK2` writer - Wake on LAN specific address register 1 event enable"]
pub type WOLMASK2_W<'a> = crate::BitWriter<'a, u32, WOLREG_SPEC, bool, 18>;
#[doc = "Field `WOLMASK3` reader - Wake on LAN multicast hash event enable"]
pub type WOLMASK3_R = crate::BitReader<bool>;
#[doc = "Field `WOLMASK3` writer - Wake on LAN multicast hash event enable"]
pub type WOLMASK3_W<'a> = crate::BitWriter<'a, u32, WOLREG_SPEC, bool, 19>;
impl R {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&self) -> WOLMASK0_R {
        WOLMASK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&self) -> WOLMASK1_R {
        WOLMASK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&self) -> WOLMASK2_R {
        WOLMASK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&self) -> WOLMASK3_R {
        WOLMASK3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W::new(self)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable"]
    #[inline(always)]
    pub fn wolmask0(&mut self) -> WOLMASK0_W {
        WOLMASK0_W::new(self)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable"]
    #[inline(always)]
    pub fn wolmask1(&mut self) -> WOLMASK1_W {
        WOLMASK1_W::new(self)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable"]
    #[inline(always)]
    pub fn wolmask2(&mut self) -> WOLMASK2_W {
        WOLMASK2_W::new(self)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable"]
    #[inline(always)]
    pub fn wolmask3(&mut self) -> WOLMASK3_W {
        WOLMASK3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake on LAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wolreg](index.html) module"]
pub struct WOLREG_SPEC;
impl crate::RegisterSpec for WOLREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wolreg::R](R) reader structure"]
impl crate::Readable for WOLREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wolreg::W](W) writer structure"]
impl crate::Writable for WOLREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WOLREG to value 0"]
impl crate::Resettable for WOLREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
