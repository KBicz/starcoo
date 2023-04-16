pub fn theta0(jd: f64, t: f64) -> f64
{
    return 280.46061837 + 360.98564736629 * (jd - 2451545.0) + (0.000387933 * t * t) - (t * t * t / 38710000.0);
}

pub fn reduceangle(mut angle: f64) -> f64
{
    while angle < 0f64 { angle += 360f64; }
    let ang = angle/360f64;
    return (ang - (ang as i64) as f64)*360f64 ;
}
