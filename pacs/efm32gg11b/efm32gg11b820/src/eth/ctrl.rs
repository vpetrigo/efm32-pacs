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
#[doc = "TSU Clock selection value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSUCLKSEL_A {
    #[doc = "0: No TSU clock source selected"]
    NOCLOCK = 0,
    #[doc = "1: Select system clock as TSU Clock"]
    PLL = 1,
    #[doc = "2: Select ethernet RX Clock as TSU Clock"]
    RXCLK = 2,
    #[doc = "3: Select ref clock as TSU Clock"]
    REFCLK = 3,
    #[doc = "4: Select tsu external pin as TSU Clock"]
    TSUEXTCLK = 4,
}
impl From<TSUCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSUCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSUCLKSEL` reader - TSU Clock selection value"]
pub type TSUCLKSEL_R = crate::FieldReader<u8, TSUCLKSEL_A>;
impl TSUCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSUCLKSEL_A> {
        match self.bits {
            0 => Some(TSUCLKSEL_A::NOCLOCK),
            1 => Some(TSUCLKSEL_A::PLL),
            2 => Some(TSUCLKSEL_A::RXCLK),
            3 => Some(TSUCLKSEL_A::REFCLK),
            4 => Some(TSUCLKSEL_A::TSUEXTCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOCLOCK`"]
    #[inline(always)]
    pub fn is_noclock(&self) -> bool {
        *self == TSUCLKSEL_A::NOCLOCK
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == TSUCLKSEL_A::PLL
    }
    #[doc = "Checks if the value of the field is `RXCLK`"]
    #[inline(always)]
    pub fn is_rxclk(&self) -> bool {
        *self == TSUCLKSEL_A::RXCLK
    }
    #[doc = "Checks if the value of the field is `REFCLK`"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == TSUCLKSEL_A::REFCLK
    }
    #[doc = "Checks if the value of the field is `TSUEXTCLK`"]
    #[inline(always)]
    pub fn is_tsuextclk(&self) -> bool {
        *self == TSUCLKSEL_A::TSUEXTCLK
    }
}
#[doc = "Field `TSUCLKSEL` writer - TSU Clock selection value"]
pub type TSUCLKSEL_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, TSUCLKSEL_A, 3, 0>;
impl<'a> TSUCLKSEL_W<'a> {
    #[doc = "No TSU clock source selected"]
    #[inline(always)]
    pub fn noclock(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::NOCLOCK)
    }
    #[doc = "Select system clock as TSU Clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::PLL)
    }
    #[doc = "Select ethernet RX Clock as TSU Clock"]
    #[inline(always)]
    pub fn rxclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::RXCLK)
    }
    #[doc = "Select ref clock as TSU Clock"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::REFCLK)
    }
    #[doc = "Select tsu external pin as TSU Clock"]
    #[inline(always)]
    pub fn tsuextclk(self) -> &'a mut W {
        self.variant(TSUCLKSEL_A::TSUEXTCLK)
    }
}
#[doc = "Field `TSUPRESC` reader - Clock division factor of TSUPRESC+1"]
pub type TSUPRESC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSUPRESC` writer - Clock division factor of TSUPRESC+1"]
pub type TSUPRESC_W<'a> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, 4>;
#[doc = "Field `MIISEL` reader - MII select signal"]
pub type MIISEL_R = crate::BitReader<bool>;
#[doc = "Field `MIISEL` writer - MII select signal"]
pub type MIISEL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 8>;
#[doc = "Field `GBLCLKEN` reader - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GBLCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `GBLCLKEN` writer - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
pub type GBLCLKEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 9>;
#[doc = "Field `TXREFCLKSEL` reader - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TXREFCLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `TXREFCLKSEL` writer - REFCLK source select for RMII_TXD and RMII_TX_EN"]
pub type TXREFCLKSEL_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 10>;
impl R {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&self) -> TSUCLKSEL_R {
        TSUCLKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&self) -> TSUPRESC_R {
        TSUPRESC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&self) -> MIISEL_R {
        MIISEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&self) -> GBLCLKEN_R {
        GBLCLKEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&self) -> TXREFCLKSEL_R {
        TXREFCLKSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TSU Clock selection value"]
    #[inline(always)]
    pub fn tsuclksel(&mut self) -> TSUCLKSEL_W {
        TSUCLKSEL_W::new(self)
    }
    #[doc = "Bits 4:7 - Clock division factor of TSUPRESC+1"]
    #[inline(always)]
    pub fn tsupresc(&mut self) -> TSUPRESC_W {
        TSUPRESC_W::new(self)
    }
    #[doc = "Bit 8 - MII select signal"]
    #[inline(always)]
    pub fn miisel(&mut self) -> MIISEL_W {
        MIISEL_W::new(self)
    }
    #[doc = "Bit 9 - Global Clock Enable signal for Ethernet clocks tsu_clk, tx_clk, rx_clk and ref_clk"]
    #[inline(always)]
    pub fn gblclken(&mut self) -> GBLCLKEN_W {
        GBLCLKEN_W::new(self)
    }
    #[doc = "Bit 10 - REFCLK source select for RMII_TXD and RMII_TX_EN"]
    #[inline(always)]
    pub fn txrefclksel(&mut self) -> TXREFCLKSEL_W {
        TXREFCLKSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
