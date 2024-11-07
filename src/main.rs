use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::{Dimensions, Point};
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_8X13};
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::primitives::{Circle, Primitive, PrimitiveStyleBuilder};
use embedded_graphics::text::{Alignment, Baseline, LineHeight, Text, TextStyleBuilder};
use embedded_graphics::Drawable;
use embedded_hal::delay::DelayNs;
use epd_waveshare::color::TriColor;
use epd_waveshare::epd7in5b_v3::{Display7in5, Epd7in5};
use epd_waveshare::prelude::WaveshareDisplay;
use rppal::gpio::Gpio;
use rppal::hal::Delay;
use rppal::spi::{SimpleHalSpiDevice, Spi};
use std::error::Error;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.

fn main() -> Result<(), Box<dyn Error>> {
    println!("EDP WaveShare via RUST!");

    let spi = Spi::new(
        rppal::spi::Bus::Spi0,
        rppal::spi::SlaveSelect::Ss0,
        4_000_000,
        rppal::spi::Mode::Mode0,
    )
    .expect("could not setup spi");
    let mut spi_device = SimpleHalSpiDevice::new(spi);
    let gpio = Gpio::new()?;
    // BCM 24 === Physical pin 18
    let busy = gpio.get(24)?.into_input();

    let mut dc = gpio.get(25)?.into_output();
    dc.set_high();

    let mut rst = gpio.get(17)?.into_output();
    rst.set_high();

    let mut delay = Delay::new();

    let mut epd7in5 = Epd7in5::new(&mut spi_device, busy, dc, rst, &mut delay, None)
        .expect("Failed to setup Epd7in5");
    let mut display = Display7in5::default();

    println!("Clear the display");
    display.clear(TriColor::White).ok();
    epd7in5.update_and_display_frame(&mut spi_device, display.buffer(), &mut delay)?;
    // delay.delay_ms(5000);

    let bounding_box = display.bounding_box();
    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_10X20)
        .text_color(TriColor::Chromatic)
        .build();
    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();
    let center_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();
    Text::with_text_style(
        "From the screen to the ring, to the pen, to the king
Where's my crown? That's my bling
Always drama when I ring
See, I believe that if I see it in my heart
Smash through the ceiling 'cause I'm reachin' for the stars
Woah-oh-oh
This is how the story goes
Woah-oh-oh
I guess this is how the story goes
I'm in the thick of it, everybody knows
They know me where it snows, I skied in and they froze (woo)
I don't know no nothin' 'bout no ice, I'm just cold
Forty somethin' milli' subs or so, I've been told",
        bounding_box.top_left,
        character_style,
        left_aligned,
    )
    .draw(&mut display)?;
    Text::with_text_style(
        "Dani Dini :O RASPBERRY PI written in rust wow",
        bounding_box.center(),
        character_style,
        center_aligned,
    )
    .draw(&mut display)?;

    println!("Draw Circle");
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(TriColor::Black)
        .stroke_width(5)
        .build();
    let _ = Circle::with_center(Point::new(300, 64), 80)
        .into_styled(style)
        .draw(&mut display);

    epd7in5.update_and_display_frame(&mut spi_device, display.buffer(), &mut delay)?;
    // delay.delay_ms(5000);
    epd7in5.sleep(&mut spi_device, &mut delay)?;
    Ok(())
}
