use crate::timef;
use crate::strut::{StringUtils,spaces};

pub const PI: f64 = 3.14159265358979323846264338327950288f64;

pub fn sunpos(lat: f64, lon: f64)
{
    let gamma: f64;
    let mut ha: f64;
    let time_offset: f64;
    let mut dofty: f64 = 0f64;
    let (tst, alt): (f64, f64);
    let sunset: (i64, i64, f64);
    let sunrise: (i64, i64, f64);
    let (eqtime, decl) : (f64, f64);
    let ltsr: (String, String, String);
    let ltss: (String, String, String);
    let utcsr: (String, String, String);
    let utcss: (String, String, String);
    let lt = chrono::offset::Local::now().to_string();
    let (year, month, day): (f64, f64, f64) = (lt.substring(0, 4).parse().unwrap(), lt.substring(5, 2).parse().unwrap(),lt.substring(8, 2).parse().unwrap());
    let hour: f64 = lt.substring(11,2).parse().unwrap();
    let minute: f64 = lt.substring(14,2).parse().unwrap();
    let seconds: f64 = lt.substring(17,lt.len()-17-7).parse().unwrap();
    let mut days = vec![31.0,28.0,31.0,30.0,31.0,30.0,31.0,31.0,30.0,31.0,30.0,31.0];
    let tz: f64 = lt.substring(lt.len()-6, 3).parse().unwrap();
    let tzs: String = lt.substring(lt.len()-6, 6).parse().unwrap();
    dofty += day;

    if timef::is_leap_year(year as i32)
    {
        days[1] += 1.0;
        if month != 1.0
        {
           for i in 0..(month as usize-1) { dofty += days[i];}
        }
        gamma = 2.0*PI/366.0*(dofty-1.0+((hour+minute/60.0+seconds/3600.0)-12.0)/24.0);
    }
    else
    {
        if month != 1.0
        {
           for i in 0..(month as usize-1) { dofty += days[i];}
        }
        gamma = 2.0*PI/365.0*(dofty-1.0+((hour+minute/60.0+seconds/3600.0)-12.0)/24.0);
    }

    eqtime =  229.18*(0.000075 + 0.001868*gamma.cos() - 0.032077*gamma.sin() - 0.014615*(2.0*gamma).cos() - 0.040849*(2.0*gamma).sin() );
    decl = (0.006918 - 0.399912*gamma.cos() + 0.070257*gamma.sin() - 0.006758*(2.0*gamma).cos() + 0.000907*(2.0*gamma).sin() - 0.002697*(3.0*gamma).cos() + 0.00148*(3.0*gamma).sin()).to_degrees();
    time_offset = eqtime + 4.0*lon - 60.0*tz;
    tst = hour*60.0 + minute + seconds/60.0 + time_offset;
    ha = tst/4.0-180.0;
    alt = (lat.to_radians().sin()*decl.to_radians().sin() + lat.to_radians().cos()*decl.to_radians().cos()*ha.to_radians().cos()).asin();
    ha = ( ( 90.833f64.to_radians().cos()/(lat.to_radians().cos()*decl.to_radians().cos()) - lat.to_radians().tan()* decl.to_radians().tan() ).acos() ).to_degrees();
    sunrise = timef::hhmmss((720.0-4.0*(lon+ha)-eqtime)/60.0*15.0);
    sunset = timef::hhmmss((720.0-4.0*(lon-ha)-eqtime)/60.0*15.0);

    ltsr = timef::converttime(sunrise.0+tz as i64,sunrise.1,sunrise.2);
    ltss = timef::converttime(sunset.0+tz as i64,sunset.1,sunset.2);
    utcsr = timef::converttime(sunrise.0,sunrise.1,sunrise.2);
    utcss = timef::converttime(sunset.0,sunset.1,sunset.2);

    println!("\n Alt. of the Sun : {}{:.6} deg ( Civ. = -6 deg, Naut. = -12 deg, Astr. = -18 deg )",spaces(alt),alt.to_degrees());
    println!(" Sunrise         :   {}:{}:{} UTC   |  {}:{}:{} {}",utcsr.0,utcsr.1,utcsr.2,ltsr.0,ltsr.1,ltsr.2,tzs);
    println!(" Sunset          :   {}:{}:{} UTC   |  {}:{}:{} {}",utcss.0,utcss.1,utcss.2,ltss.0,ltss.1,ltss.2,tzs);
}