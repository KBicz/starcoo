use chrono;
use std::env;
use ansi_term::Style;
use std::process::exit;
use std::{thread, time};
use std::process::Command;
use std::ops::{Bound, RangeBounds};

pub const PI: f64 = 3.14159265358979323846264338327950288f64;
fn deg2rad(deg: f64) -> f64 { return deg*PI/180f64; }
fn rad2deg(rad: f64) -> f64 { return rad*180f64/PI; }

fn helpf()
{    
    println!("\n  Program starcoo for MacOS, Linux and Windows written by K. Bicz, ver. of Aug 1, 2021.");
    println!("  Show live information about star coordinates.");
    println!("\n  Usage: starcoo <-star=Str || -delta=f64 -alpha=f64> \n");
    println!("         option  -star : star name (only with installed simpos without ra & dec, format = YZ_CMi).");
    println!("                 -ra   : right ascension of the star (in degrees).");
    println!("                 -dec  : declination of the star (in degrees).");
    println!("                 -lat  : latitude of the observation place (in degrees).");
    println!("                 -lon  : longitude of the observation place (in degrees).");
    println!("                 -ref  : refresh rate (in Hz).");
    println!("");
    exit(0);
}

fn is_leap_year(year: i32) -> bool { (year % 4 == 0 && year % 100 != 0) && (year % 400 == 0) }

trait StringUtils 
{
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str 
{
    fn substring(&self, start: usize, len: usize) -> &str 
    {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop 
        {
            if char_pos == start { break; }
            if let Some(c) = it.next() 
            {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop 
        {
            if char_pos == len { break; }
            if let Some(c) = it.next() 
            {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str 
    {
        let start = match range.start_bound() 
        {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() 
        {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

fn julday() -> f64
{
    let utc = chrono::offset::Utc::now().to_string();
    let l: f64 = utc.substring(0, 4).parse().unwrap();
    let m: f64 = utc.substring(5, 2).parse().unwrap();
    let d: f64 = utc.substring(8, 2).parse().unwrap();

    let hour: f64 = utc.substring(11,2).parse().unwrap();
    let minute: f64 = utc.substring(14,2).parse().unwrap();
    let seconds: f64 = utc.substring(17,utc.len()-17-4).parse().unwrap();

    let l1: i64 =  (l as i64) + 4716 - (((14f64-m)/12f64) as i64);
    let m1: i64 = ((m as i64) + 9).rem_euclid(12);
    let g: i64 = ((0.75f64 * (((l1 as f64+184f64)/100f64) as i64) as f64 ) as i64)-38;

    return (((365.25*(l1 as f64)) as i64) + ((30.6*(m1 as f64)+0.4) as i64) + (d as i64) - g - 1402) as f64 - 0.5f64 +  hour/24f64 + minute/(60f64*24f64)+seconds/(24f64*3600f64);
}

fn theta0(jd: f64, t: f64) -> f64
{
    return 280.46061837 + 360.98564736629 * (jd - 2451545.0) + (0.000387933 * t * t) - (t * t * t / 38710000.0);
}

fn reduceangle(mut angle: f64) -> f64
{
    while angle < 0f64 { angle += 360f64; }
    let ang = angle/360f64;
    return (ang - (ang as i64) as f64)*360f64 ;
}

fn spaces(coo: f64) -> String
{
    let mut spaces: &str = "";

    if coo >= 0f64
    {
        if coo >= 100f64 {spaces = " ";}
        else if coo < 100f64 && coo >= 10f64 {spaces = "  ";}
        else if coo < 10f64 {spaces = "   ";}
    }
    else
    {
        if coo.abs() >= 100f64 {spaces = "";}
        else if coo.abs() < 100f64 && coo.abs() >= 10f64 {spaces = " ";}
        else if coo.abs() < 10f64 {spaces = "  ";}
    }
    
    spaces.to_string()
}

fn hhmmss(coo: f64) -> (i64,i64,f64)
{
    let coo: f64 = coo.abs();
    let (hh, mm, ss): (i64, i64, f64);

    hh = (coo/15f64) as i64;
    mm = ((coo/15f64 - hh as f64)*60f64) as i64;
    ss = coo/15f64*3600f64 - (hh as f64)*3600f64 - (mm as f64)*60f64; 

    (hh,mm,(ss*10f64).round()/10f64)
}

fn converttime(t0:i64, t1:i64, t2:f64) -> (String, String, String)
{
    let mut result: (String, String, String) = (format![""],format![""],format![""]);

    if t0 < 10 { result.0 = format!["0{}",t0]; }
    else { result.0 = format!["{}",t0]; }
    if t1 < 10 { result.1 = format!["0{}",t1]; }
    else { result.1 = format!["{}",t1]; }
    if t2 < 10f64 { result.2 = format!["0{}",t2]; }
    else { result.2 = format!["{}",t2]; }

    if result.2.len() == 2 {result.2 = format!["{}.0",result.2]}

    result
}

fn airmass(alt: f64) -> f64 { 1f64/(deg2rad(alt)+244.0/(165.0+47.0*alt.powf(1.1))).sin() }

fn sunpos(lat: f64, lon: f64)
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

    if is_leap_year(year as i32)
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
    decl = rad2deg(0.006918 - 0.399912*gamma.cos() + 0.070257*gamma.sin() - 0.006758*(2.0*gamma).cos() + 0.000907*(2.0*gamma).sin() - 0.002697*(3.0*gamma).cos() + 0.00148*(3.0*gamma).sin());
    time_offset = eqtime + 4.0*lon - 60.0*tz;
    tst = hour*60.0 + minute + seconds/60.0 + time_offset;
    ha = tst/4.0-180.0;
    alt = (deg2rad(lat).sin()*deg2rad(decl).sin() + deg2rad(lat).cos()*deg2rad(decl).cos()*deg2rad(ha).cos()).asin();
    ha = rad2deg(  ( deg2rad(90.833).cos()/(deg2rad(lat).cos()*deg2rad(decl).cos()) - deg2rad(lat).tan()*deg2rad(decl).tan() ).acos()  );
    sunrise = hhmmss((720.0-4.0*(lon+ha)-eqtime)/60.0*15.0);
    sunset = hhmmss((720.0-4.0*(lon-ha)-eqtime)/60.0*15.0);

    ltsr = converttime(sunrise.0+tz as i64,sunrise.1,sunrise.2);
    ltss = converttime(sunset.0+tz as i64,sunset.1,sunset.2);
    utcsr = converttime(sunrise.0,sunrise.1,sunrise.2);
    utcss = converttime(sunset.0,sunset.1,sunset.2);

    println!("\n Alt. of the Sun : {}{:.6} deg ( Civ. = -6 deg, Naut. = -12 deg, Astr. = -18 deg )",spaces(alt),rad2deg(alt));
    println!(" Sunrise         :   {}:{}:{} UTC   |  {}:{}:{} {}",utcsr.0,utcsr.1,utcsr.2,ltsr.0,ltsr.1,ltsr.2,tzs);
    println!(" Sunset          :   {}:{}:{} UTC   |  {}:{}:{} {}",utcss.0,utcss.1,utcss.2,ltss.0,ltss.1,ltss.2,tzs);
}

fn skycoo(star: &str, ra: f64, dec: f64, lat: f64, lon: f64, refr: f64)
{
    let mut hhat: f64;
    let (mut hat, mut rat) : ((i64,i64,f64),(i64,i64,f64));
    let (mut alt, mut ha, mut az, mut jd, mut t, mut theta, mut gmstra): (f64, f64, f64, f64, f64, f64, f64);

    loop
    {
        jd = julday();
        t = (jd - 2451545.0)/36525f64;
        theta = theta0(jd, t);
        gmstra = reduceangle(theta);
        ha = gmstra + lon - ra;
        alt = (deg2rad(lat).sin()*deg2rad(dec).sin() + deg2rad(lat).cos()*deg2rad(dec).cos()*deg2rad(ha).cos()).asin();
        az = (-deg2rad(ha).sin()*deg2rad(dec).cos()/deg2rad(alt).cos()).asin();
        hat = hhmmss(ha); rat = hhmmss(ra);

        alt = rad2deg(alt); az = rad2deg(az)+360.0; 
        if az > 360.0 {az -= 360.0;}

        println!("\n Star: {}\n",Style::new().bold().paint(star.replace("_"," ")).to_string());
        println!(" Julian Day      :  {:.8}",jd);
        println!(" Local Time      :  {}",chrono::offset::Local::now());
        println!(" Universal Time  :  {}\n",chrono::offset::Utc::now());
        println!(" Right Ascension :  {}{:.6} deg   |{}{}h {:2.}m{}{:.1}s",spaces(ra),ra,spaces(rat.0 as f64),rat.0,rat.1,spaces(-rat.2.floor()),rat.2);
        hhat = -hat.2.floor(); if hhat == 0.0 {hhat -= 1.0;}
        println!(" Hour Angle      :  {}{:.6} deg   |{}{}h {:2.}m{}{:.1}s",spaces(ha),ha,spaces(hat.0 as f64),hat.0,hat.1,spaces(hhat),hat.2);
        println!(" Declination     :  {}{:.6} deg",spaces(dec),dec);
        println!(" Azimuth         :  {}{:.6} deg",spaces(az),az);
        println!(" Altitude        :  {}{:.6} deg",spaces(alt),alt);
        println!(" Airmass         :  {}{:.6}",spaces(airmass(alt.abs())),airmass(alt.abs()));

        sunpos(lat,lon);

        println!();
        if refr == 0.0 {break;}
        else 
        {
            thread::sleep(time::Duration::from_millis((1f64/refr*1000f64).round() as u64));
            if env::consts::OS.eq("macos") { std::process::Command::new("printf").arg(" \'\\33c").status().unwrap(); }
            else if env::consts::OS.eq("linux") { std::process::Command::new("clear").status().unwrap();}
            else if env::consts::OS.eq("windows") {  std::process::Command::new("cls").status().unwrap();}
            else { print!("\x1B[2J\x1B[1;1H");}
        }
    }
}

fn main() 
{
    let mut v: Vec<&str>;
    let mut lon: f64 = 16.657820;
    let argc: usize = env::args().len();
    let mut simpos: String = "simpos".to_string();
    let argv: Vec<String> = env::args().collect();
    let (mut starctrl, mut ractrl, mut dectrl, mut refr): (bool, bool, bool, f64) = (false, false, false, 0f64);
    let (mut star, mut ra, mut dec, mut lat): (&str, f64, f64, f64) = ("YZ_CMi", 116.167385f64, 3.552465f64, 51.474249);

    if argc == 1 {helpf();}
    else
    {
        for i in 0..argc
        {
            if argv[i].contains("-star=") {v = argv[i].split("=").collect(); star = v[1]; starctrl = true;}
            else if argv[i].contains("-ra=") {v = argv[i].split("=").collect(); ra = v[1].parse().unwrap(); ractrl = true;} 
            else if argv[i].contains("-dec=") {v = argv[i].split("=").collect(); dec = v[1].parse().unwrap(); dectrl = true;} 
            else if argv[i].contains("-lat=") {v = argv[i].split("=").collect(); lat = v[1].parse().unwrap();} 
            else if argv[i].contains("-lon=") {v = argv[i].split("=").collect(); lon = v[1].parse().unwrap();} 
            else if argv[i].contains("-refr=") {v = argv[i].split("=").collect(); refr = v[1].parse().unwrap();}
            else if argv[i].eq("--help") || argv[i].eq("-h")  {helpf();} 
        }
    }

    if !starctrl && ractrl && dectrl {star = "Unknown";}
    else if !starctrl && !ractrl && !dectrl {helpf();}
    else if starctrl && (!ractrl || !dectrl) 
    {
        if env::consts::OS.eq("windows") {simpos = "simpos.exe".to_string();}
        let output = Command::new(simpos).arg("-d").arg(star.to_string()).output().expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        let output: Vec<&str> = output.split(" ").collect();
        ra = output[0].parse().unwrap(); dec = output[1].parse().unwrap();
    }

    skycoo(star,ra,dec,lat,lon,refr);
}