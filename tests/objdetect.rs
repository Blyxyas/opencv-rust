use std::path::PathBuf;

use opencv::{
    core::Mat,
    imgcodecs,
    objdetect,
    Result,
    types::VectorOfPoint,
};

#[test]
fn qr_code() -> Result<()> {
    let qr_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/qr.png");

    {
        let mut detector = objdetect::QRCodeDetector::new()?;
        let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
        let mut pts = VectorOfPoint::new();
        let mut straight = Mat::new()?;
        let res = detector.detect_and_decode(&src, &mut pts, &mut straight)?;
        assert_eq!(res, "https://crates.io/crates/opencv");
        assert_eq!(4, pts.len());
        assert!(!straight.empty()?);
    }

    {
        let mut detector = objdetect::QRCodeDetector::new()?;
        let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
        let mut pts = VectorOfPoint::new();
        let res = detector.detect(&src, &mut pts)?;
        assert!(res);
        assert_eq!(4, pts.len());
        let mut straight = Mat::new()?;
        let res = detector.decode(&src, &pts, &mut straight)?;
        assert_eq!(res, "https://crates.io/crates/opencv");
        assert!(!straight.empty()?);
    }

    {
        let src = imgcodecs::imread(qr_path.to_str().unwrap(), imgcodecs::IMREAD_COLOR)?;
        let mut pts = VectorOfPoint::new();
        let res = objdetect::detect_qr_code(&src, &mut pts, 0.2, 0.1)?;
        assert!(res);
        assert_eq!(4, pts.len());
        let mut out = String::new();
        let mut straight = Mat::new()?;
        let res = objdetect::decode_qr_code(&src, &pts, &mut out, &mut straight)?;
        assert!(res);
        assert_eq!(out, "https://crates.io/crates/opencv");
        assert!(!straight.empty()?);
    }
    Ok(())
}
