use crate::strut::StringUtils;

pub fn is_leap_year(year: i32) -> bool { (year % 4 == 0 && year % 100 != 0) && (year % 400 == 0) }

pub fn julday() -> f64
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

pub fn hhmmss(coo: f64) -> (i64,i64,f64)
{
    let coo: f64 = coo.abs();
    let (hh, mm, ss): (i64, i64, f64);

    hh = (coo/15f64) as i64;
    mm = ((coo/15f64 - hh as f64)*60f64) as i64;
    ss = coo/15f64*3600f64 - (hh as f64)*3600f64 - (mm as f64)*60f64; 

    (hh,mm,(ss*10f64).round()/10f64)
}

pub fn converttime(t0:i64, t1:i64, t2:f64) -> (String, String, String)
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