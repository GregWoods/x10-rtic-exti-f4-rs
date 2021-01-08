#![no_std]
#![no_main]

use cortex_m::asm::delay;
use panic_halt as _;
use rtic::app;
use rtic;
use rtic_core::prelude::TupleExt02;

use stm32f4xx_hal::{
    delay::Delay, 
    gpio::gpioa::{PA5}, 
    gpio::gpioc::{PC13}, 
    gpio::{
        Edge, ExtiPin, Input, Output, PullUp, PushPull
    }, 
    prelude::*, 
    rcc::{Clocks, Rcc}, 
    stm32::{self, GPIOA, GPIOC}
};

use rtt_target::{rprintln, rtt_init_print};



#[rtic::app(device = stm32f4xx_hal::stm32, peripherals = true, dispatchers = [SSI0, QEI0])]
mod app
{

    use rtt_target::{rprintln, rtt_init_print};
    use stm32f4xx_hal::{gpio::{Edge, ExtiPin, GpioExt, Input, Output, PullUp, PushPull, gpioa::PA5, gpioc::PC13}, rcc::RccExt, stm32};

    #[resources]
    struct Resources 
    {
        exti: stm32::EXTI,
        usr_led: PA5<Output<PushPull>>,
        usr_btn: PC13<Input<PullUp>>,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources 
    {
        // RTT handler
        rtt_init_print!();
        rprintln!("==== Start! ====");

        // Alias peripherals
        let mut dp: stm32::Peripherals = ctx.device;

        //essential for external interrupts it seems
        dp.RCC.apb2enr.write(|w| w.syscfgen().enabled());

        let rcc = dp.RCC.constrain();
        let clocks = crate::setup_clocks(rcc);

        //LED setup
        let gpioa = dp.GPIOA.split();
        let usr_led = gpioa.pa5.into_push_pull_output();

        let mut exti = dp.EXTI;

        /*
        //usr_btn  and exti setup
        let gpioc = dp.GPIOC.split();
        let usr_btn = gpioc.pc13.into_pull_up_input();
        dp.SYSCFG.exticr4.write(|w| unsafe { w.exti13().bits(0x010)}); //portC
        exti.imr.write(|w| { w.mr13().set_bit() });
        exti.rtsr.write(|w| { w.tr13().set_bit() });
        exti.pr.write(|w| w.pr13().set_bit()); // Clear interrupt
        */
        exti.imr.write(|w| unsafe {w.bits(0b0010000000000000)});    //bit13 set, reset the rest

        // Create a usr_btn input with an interrupt - using the HAL
        let gpioc = dp.GPIOC.split();
        let mut usr_btn = gpioc.pc13.into_pull_up_input();
        let mut syscfg = dp.SYSCFG; //.constrain();
        usr_btn.make_interrupt_source(&mut syscfg);
        usr_btn.enable_interrupt(&mut exti);
        usr_btn.trigger_on_edge(&mut exti, Edge::FALLING);
        
        init::LateResources {
            exti,
            usr_led,
            usr_btn,
        }
    }


    #[idle(resources = [usr_btn, usr_led])]
    fn idle(cx: idle::Context) -> ! 
    {
        let mut usr_led = cx.resources.usr_led;
        let mut usr_btn = cx.resources.usr_btn;
        
        usr_led.lock(|usr_led| {
            use stm32f4xx_hal::prelude::_embedded_hal_digital_v2_ToggleableOutputPin;
            let tmp_led = *usr_led;
            tmp_led.toggle();
        });

        //Multi-lock SYNTAX... broken!
        (usr_led, usr_btn).lock(|usr_led, usr_btn| {

            //let mut delay = Delay::new(cp.SYST, clocks);
            loop {
                *usr_led.toggle().ok();
                //if btn. == 1 {
                //    rprintln!("usr_btn pressed!");
                //}
                //cortex_m::asm::wfi();   //is this needed?
                //delay.delay_ms(100u32);
            }
        });    
    }


    #[task(binds = EXTI15_10, resources = [exti, usr_led, usr_btn])]
    fn exti_15_10_interrupt(cx: exti_15_10_interrupt::Context) {
        let mut exti = cx.resources.exti;
        let mut usr_led = cx.resources.usr_led;
        let mut usr_btn = cx.resources.usr_btn;        
        (exti, usr_led, usr_btn).lock(|exti, usr_led, usr_btn| {
            //refer to the resources as *exti, *usr_led, *usr_btn

            //check which interrupt fired
            let val = *exti.pr.read().bits();
            match val {
                0x2000 => {
                    //0x2000 = bit 13 set
                }
                _ => rprintln!("another value of EXTI15_10 ;)"),
            }

            //cx.resources.exti.pr.write(|w| w.pr13().set_bit()); // Clear interrupt
            *usr_btn.clear_interrupt_pending_bit();
            *usr_led.toggle().ok();

        });
        
        
    }



}

fn setup_clocks(rcc: Rcc) -> Clocks {
    return rcc
        .cfgr
        .hclk(48.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .pclk2(24.mhz())
        .freeze();
}


