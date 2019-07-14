use velodyne::{PointSource, FullPoint, packet::{PcapSource}};

fn to_mm(val: f32) -> i32 {
    (val*1000.) as i32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pcap_path = std::env::args().nth(1).expect("provide path to pcap file");
    let source = PcapSource::new(pcap_path, false, false)?;
    let mut source = PointSource::hdl64_init(source)?;
    loop {
        let mut buf = vec![];
        let (t, meta) = match source.process_points(|p: FullPoint| buf.push(p))? {
            Some(m) => m,
            None => break,
        };
        for p in buf.iter() {
            println!("{}\t{}\t{:.4}\t{:.4}\t{:.4}\t{}\t{}",
                t, meta.azimuth,
                to_mm(p.xyz[0]), to_mm(p.xyz[1]), to_mm(p.xyz[2]),
                p.intensity, p.laser_id,
            );
        }
        buf.clear();
    }
    Ok(())
}
