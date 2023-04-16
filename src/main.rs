mod strut;
mod timef;
mod angles;
mod sunposition;

use chrono;
use std::env;
use ansi_term::Style;
use std::process::exit;
use std::{thread, time};
use crate::strut::spaces;
use std::process::Command;

fn helpf()
{    
    println!("\n  Program starcoo for MacOS, Linux and Windows written by K. Bicz, ver. of Aug 1, 2021.");
    println!("  Show live information about star coordinates.");
    println!("\n  Usage: starcoo <-star=Str || -delta=f64 -alpha=f64> [-lat=i64] [-lon=i64] [-ref=i64]\n");
    println!("         option  -star : star name (only with installed simpos without ra & dec, format = YZ_CMi).");
    println!("                 -ra   : right ascension of the star (in degrees).");
    println!("                 -dec  : declination of the star (in degrees).");
    println!("                 -lat  : latitude of the observation place (in degrees).");
    println!("                 -lon  : longitude of the observation place (in degrees).");
    println!("                 -ref  : refresh rate (in Hz).");
    println!("");
    exit(0);
}

fn airmass(alt: f64) -> f64 { 1f64/(alt.to_radians()+244.0/(165.0+47.0*alt.powf(1.1))).sin() }

fn skycoo(star: &str, ra: f64, dec: f64, lat: f64, lon: f64, refr: f64)
{
    let mut hhat: f64;
    let (mut hat, mut rat) : ((i64,i64,f64),(i64,i64,f64));
    let (mut alt, mut ha, mut az, mut jd, mut t, mut theta, mut gmstra): (f64, f64, f64, f64, f64, f64, f64);

    loop
    {
        jd = timef::julday();
        t = (jd - 2451545.0)/36525f64;
        theta = angles::theta0(jd, t);
        gmstra = angles::reduceangle(theta);
        ha = gmstra + lon - ra;
        alt = (lat.to_radians().sin()*dec.to_radians().sin() + lat.to_radians().cos()*dec.to_radians().cos()*ha.to_radians().cos()).asin();
        az = (-ha.to_radians().sin()*dec.to_radians().cos()/alt.to_radians().cos()).asin();
        hat = timef::hhmmss(ha); rat = timef::hhmmss(ra);

        alt = alt.to_degrees(); az = az.to_degrees()+360.0; 
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

        sunposition::sunpos(lat,lon);

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
            else if argv[i].contains("-ref=") {v = argv[i].split("=").collect(); refr = v[1].parse().unwrap();}
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