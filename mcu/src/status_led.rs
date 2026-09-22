#[derive(Copy, Clone)]
pub enum Status {
    None,
    Init,
    Okay,
    Error,
}

impl Into<(bool,bool,bool)> for Status {
    fn into(self) -> (bool,bool,bool) {
        match self {
            Status::None => (false, false, false), // black
            Status::Init => (false, false, true), // blue
            Status::Okay => (false, true, false), // green
            Status::Error => (true, false, false), // red
        }
    }
}

static COLOR: embassy_sync::signal::Signal<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, Status> = embassy_sync::signal::Signal::new();

#[embassy_executor::task]
pub async fn status_led(mut pin_red: embassy_rp::gpio::Output<'static>, mut pin_green: embassy_rp::gpio::Output<'static>, mut pin_blue: embassy_rp::gpio::Output<'static>) {
    COLOR.signal(Status::Init);

    { // init
        let col = Status::Init;
        let (red, green, blue) = col.into();

        if red {
            pin_red.set_high();
        } else {
            pin_red.set_low();
        }

        if green {
            pin_green.set_high();
        } else {
            pin_green.set_low();
        }

        if blue {
            pin_blue.set_high();
        } else {
            pin_blue.set_low();
        }
    }

    loop { // update
        let col = COLOR.wait().await;
        let (red, green, blue) = col.into();

        if red {
            pin_red.set_high();
        } else {
            pin_red.set_low();
        }

        if green {
            pin_green.set_high();
        } else {
            pin_green.set_low();
        }

        if blue {
            pin_blue.set_high();
        } else {
            pin_blue.set_low();
        }
    }
}