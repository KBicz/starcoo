# starcoo
Program to show actual position of the selected star and the Sun on the sky for the given location on the Earth. Recommended to have installed wcstools. With wcstools you can run the code by giving only the star name. If wcstools are not installed the user must provide both declination and right ascension of the star. The default localization is set to Astronomical Observatory of the University of Wrocław, Białków, Poland. Compile it using command "cargo build --release".

    Usage: starcoo <-star=Str || -delta=f64 -alpha=f64> [-lat=f64] [-lon=f64] [-refr=f64]

         option  -star : star name (only with installed simpos without ra & dec, format = YZ_CMi).
                 -ra   : right ascension of the star (in degrees).
                 -dec  : declination of the star (in degrees).
                 -lat  : latitude of the observation place (in degrees).
                 -lon  : longitude of the observation place (in degrees).
                 -ref  : refresh rate (in Hz).
