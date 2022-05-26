use intder::Intder;
use rust_anpass::Dvec;
use spectro::Spectro;

use crate::config::Config;
use crate::coord_type::CoordType;
use crate::coord_type::SIC;

#[test]
fn full() {
    let config = Config::load("testfiles/test.toml");
    let coord = SIC::new(Intder::load_file("testfiles/intder.in"));
    let spectro = Spectro::load("testfiles/spectro.in");
    let summ = coord.run(&config, &spectro);
    approx::assert_abs_diff_eq!(
        Dvec::from(summ.corr),
        Dvec::from(vec![
            2784.0, 2764.3, 1775.7, 1177.1, 1040.6, 960.1, 920.0, 927.0, 905.3,
        ]),
        epsilon = 1.0
    )
}
