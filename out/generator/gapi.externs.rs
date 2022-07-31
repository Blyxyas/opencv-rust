extern "C" {
	// descr_of(const cv::Mat &) /usr/include/opencv2/gapi/gmat.hpp:268
	pub fn cv_descr_of_const_MatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// descr_of(const cv::MediaFrame &) /usr/include/opencv2/gapi/gframe.hpp:107
	pub fn cv_descr_of_const_MediaFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// descr_of(const cv::RMat &) /usr/include/opencv2/gapi/gmat.hpp:265
	pub fn cv_descr_of_const_RMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// descr_of(const cv::Scalar &) /usr/include/opencv2/gapi/gscalar.hpp:134
	pub fn cv_descr_of_const_ScalarR(scalar: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// descr_of(const cv::UMat &) /usr/include/opencv2/gapi/gmat.hpp:256
	pub fn cv_descr_of_const_UMatR(mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BGR2Gray(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1415
	pub fn cv_gapi_BGR2Gray_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BGR2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1448
	pub fn cv_gapi_BGR2I420_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BGR2LUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1510
	pub fn cv_gapi_BGR2LUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BGR2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1552
	pub fn cv_gapi_BGR2YUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BayerGR2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1639
	pub fn cv_gapi_BayerGR2RGB_const_GMatR(src_gr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Canny(const cv::GMat &, double, double, int, bool) /usr/include/opencv2/gapi/imgproc.hpp:1026
	pub fn cv_gapi_Canny_const_GMatR_double_double_int_bool(image: *const c_void, threshold1: f64, threshold2: f64, aperture_size: i32, l2gradient: bool, ocvrs_return: *mut Result<*mut c_void>);
	// I4202BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1480
	pub fn cv_gapi_I4202BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// I4202RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1496
	pub fn cv_gapi_I4202RGB_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// LUT(const cv::GMat &, const cv::Mat &) /usr/include/opencv2/gapi/core.hpp:1699
	pub fn cv_gapi_LUT_const_GMatR_const_MatR(src: *const c_void, lut: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// LUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1524
	pub fn cv_gapi_LUV2BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Laplacian(const cv::GMat &, int, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:967
	pub fn cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(src: *const c_void, ddepth: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// NV12toBGR(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1625
	pub fn cv_gapi_NV12toBGR_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// NV12toBGRp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1703
	pub fn cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// NV12toGray(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1610
	pub fn cv_gapi_NV12toGray_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// NV12toRGB(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1595
	pub fn cv_gapi_NV12toRGB_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// NV12toRGBp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1685
	pub fn cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(src_y: *const c_void, src_uv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2Gray(const cv::GMat &, float, float, float) /usr/include/opencv2/gapi/imgproc.hpp:1402
	pub fn cv_gapi_RGB2Gray_const_GMatR_float_float_float(src: *const c_void, r_y: f32, g_y: f32, b_y: f32, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2HSV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1653
	pub fn cv_gapi_RGB2HSV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1464
	pub fn cv_gapi_RGB2I420_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2Lab(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1566
	pub fn cv_gapi_RGB2Lab_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2YUV422(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1667
	pub fn cv_gapi_RGB2YUV422_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// RGB2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1432
	pub fn cv_gapi_RGB2YUV_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Sobel(const cv::GMat &, int, int, int, int, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:886
	pub fn cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(src: *const c_void, ddepth: i32, dx: i32, dy: i32, ksize: i32, scale: f64, delta: f64, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// YUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1538
	pub fn cv_gapi_YUV2BGR_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// YUV2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1580
	pub fn cv_gapi_YUV2RGB_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// absDiffC(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1254
	pub fn cv_gapi_absDiffC_const_GMatR_const_GScalarR(src: *const c_void, c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// absDiff(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1237
	pub fn cv_gapi_absDiff_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// addC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:638
	pub fn cv_gapi_addC_const_GScalarR_const_GMatR_int(c: *const c_void, src1: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// addWeighted(const cv::GMat &, double, const cv::GMat &, double, double, int) /usr/include/opencv2/gapi/core.hpp:1302
	pub fn cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// bilateralFilter(const cv::GMat &, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:1001
	pub fn cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(src: *const c_void, d: i32, sigma_color: f64, sigma_space: f64, border_type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_and(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1089
	pub fn cv_gapi_bitwise_and_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_and(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1095
	pub fn cv_gapi_bitwise_and_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_not(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1165
	pub fn cv_gapi_bitwise_not_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_or(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1113
	pub fn cv_gapi_bitwise_or_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_or(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1119
	pub fn cv_gapi_bitwise_or_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_xor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1138
	pub fn cv_gapi_bitwise_xor_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// bitwise_xor(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1144
	pub fn cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// blur(const cv::GMat &, const cv::Size &, const cv::Point &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:657
	pub fn cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(src: *const c_void, ksize: *const core::Size, anchor: *const core::Point, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// boxFilter(const cv::GMat &, int, const cv::Size &, const cv::Point &, bool, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:630
	pub fn cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(src: *const c_void, dtype: i32, ksize: *const core::Size, anchor: *const core::Point, normalize: bool, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// cmpEQ(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1041
	pub fn cv_gapi_cmpEQ_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpEQ(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1045
	pub fn cv_gapi_cmpEQ_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpGE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:989
	pub fn cv_gapi_cmpGE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpGE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:993
	pub fn cv_gapi_cmpGE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpGT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:937
	pub fn cv_gapi_cmpGT_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpGT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:941
	pub fn cv_gapi_cmpGT_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpLE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1015
	pub fn cv_gapi_cmpLE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpLE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1019
	pub fn cv_gapi_cmpLE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpLT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:963
	pub fn cv_gapi_cmpLT_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpLT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:967
	pub fn cv_gapi_cmpLT_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpNE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1067
	pub fn cv_gapi_cmpNE_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cmpNE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1071
	pub fn cv_gapi_cmpNE_const_GMatR_const_GScalarR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// combine(const cv::GKernelPackage &, const cv::GKernelPackage &) /usr/include/opencv2/gapi/gkernel.hpp:417
	pub fn cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// concatHor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1632
	pub fn cv_gapi_concatHor_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// concatHor(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1640
	pub fn cv_gapi_concatHor_const_vector_GMat_R(v: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// concatVert(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1672
	pub fn cv_gapi_concatVert_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// concatVert(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1680
	pub fn cv_gapi_concatVert_const_vector_GMat_R(v: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// convertTo(const cv::GMat &, int, double, double) /usr/include/opencv2/gapi/core.hpp:1716
	pub fn cv_gapi_convertTo_const_GMatR_int_double_double(src: *const c_void, rdepth: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result<*mut c_void>);
	// copy(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:88
	pub fn cv_gapi_copy_const_GFrameR(in_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// crop(const cv::GMat &, const cv::Rect &) /usr/include/opencv2/gapi/core.hpp:1604
	pub fn cv_gapi_crop_const_GMatR_const_RectR(src: *const c_void, rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// dilate3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:804
	pub fn cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(src: *const c_void, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// dilate(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:780
	pub fn cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// divC(const cv::GMat &, const cv::GScalar &, double, int) /usr/include/opencv2/gapi/core.hpp:788
	pub fn cv_gapi_divC_const_GMatR_const_GScalarR_double_int(src: *const c_void, divisor: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// divRC(const cv::GScalar &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:809
	pub fn cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(divident: *const c_void, src: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// div(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:767
	pub fn cv_gapi_div_const_GMatR_const_GMatR_double_int(src1: *const c_void, src2: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// equalizeHist(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1101
	pub fn cv_gapi_equalizeHist_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// erode3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:753
	pub fn cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(src: *const c_void, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// erode(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:733
	pub fn cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(src: *const c_void, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// filter2D(const cv::GMat &, int, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:596
	pub fn cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src: *const c_void, ddepth: i32, kernel: *const c_void, anchor: *const core::Point, delta: *const core::Scalar, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// flip(const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:1590
	pub fn cv_gapi_flip_const_GMatR_int(src: *const c_void, flip_code: i32, ocvrs_return: *mut Result<*mut c_void>);
	// gaussianBlur(const cv::GMat &, const cv::Size &, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:690
	pub fn cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(src: *const c_void, ksize: *const core::Size, sigma_x: f64, sigma_y: f64, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// inRange(const cv::GMat &, const cv::GScalar &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1441
	pub fn cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(src: *const c_void, thresh_low: *const c_void, thresh_up: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// mask(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:822
	pub fn cv_gapi_mask_const_GMatR_const_GMatR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// max(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1220
	pub fn cv_gapi_max_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// merge3(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1484
	pub fn cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, src3: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// merge4(const cv::GMat &, const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1465
	pub fn cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// min(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1203
	pub fn cv_gapi_min_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// morphologyEx(const cv::GMat &, const cv::MorphTypes, const cv::Mat &, const cv::Point &, const int, const cv::BorderTypes, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:834
	pub fn cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(src: *const c_void, op: crate::imgproc::MorphTypes, kernel: *const c_void, anchor: *const core::Point, iterations: i32, border_type: core::BorderTypes, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// mulC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:742
	pub fn cv_gapi_mulC_const_GMatR_const_GScalarR_int(src: *const c_void, multiplier: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mulC(const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:740
	pub fn cv_gapi_mulC_const_GMatR_double_int(src: *const c_void, multiplier: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mulC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:744
	pub fn cv_gapi_mulC_const_GScalarR_const_GMatR_int(multiplier: *const c_void, src: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mul(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:722
	pub fn cv_gapi_mul_const_GMatR_const_GMatR_double_int(src1: *const c_void, src2: *const c_void, scale: f64, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// normInf(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1370
	pub fn cv_gapi_normInf_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// normL1(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1325
	pub fn cv_gapi_normL1_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// normL2(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1347
	pub fn cv_gapi_normL2_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// normalize(const cv::GMat &, double, double, int, int) /usr/include/opencv2/gapi/core.hpp:1738
	pub fn cv_gapi_normalize_const_GMatR_double_double_int_int(src: *const c_void, alpha: f64, beta: f64, norm_type: i32, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// descr_of(const cv::gapi::own::Mat &) /usr/include/opencv2/gapi/gmat.hpp:262
	pub fn cv_gapi_own_descr_of_const_MatR(mat: *const crate::gapi::Own_Mat, ocvrs_return: *mut Result<*mut c_void>);
	// phase(const cv::GMat &, const cv::GMat &, bool) /usr/include/opencv2/gapi/core.hpp:899
	pub fn cv_gapi_phase_const_GMatR_const_GMatR_bool(x: *const c_void, y: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result<*mut c_void>);
	// remap(const cv::GMat &, const cv::Mat &, const cv::Mat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1551
	pub fn cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(src: *const c_void, map1: *const c_void, map2: *const c_void, interpolation: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// resizeP(const cv::GMatP &, const cv::Size &, int) /usr/include/opencv2/gapi/imgproc.hpp:1763
	pub fn cv_gapi_resizeP_const_GMatPR_const_SizeR_int(src: *const c_void, dsize: *const core::Size, interpolation: i32, ocvrs_return: *mut Result<*mut c_void>);
	// select(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1181
	pub fn cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(src1: *const c_void, src2: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// sepFilter(const cv::GMat &, int, const cv::Mat &, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:559
	pub fn cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(src: *const c_void, ddepth: i32, kernel_x: *const c_void, kernel_y: *const c_void, anchor: *const core::Point, delta: *const core::Scalar, border_type: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// sqrt(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:910
	pub fn cv_gapi_sqrt_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// BGR(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:43
	pub fn cv_gapi_streaming_BGR_const_GFrameR(in_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// UV(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:63
	pub fn cv_gapi_streaming_UV_const_GFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Y(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:53
	pub fn cv_gapi_streaming_Y_const_GFrameR(frame: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// desync(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/desync.hpp:79
	pub fn cv_gapi_streaming_desync_const_GFrameR(f: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// desync(const cv::GMat &) /usr/include/opencv2/gapi/streaming/desync.hpp:78
	pub fn cv_gapi_streaming_desync_const_GMatR(g: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// kernels() /usr/include/opencv2/gapi/streaming/format.hpp:16
	pub fn cv_gapi_streaming_kernels(ocvrs_return: *mut Result<*mut c_void>);
	// subC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:682
	pub fn cv_gapi_subC_const_GMatR_const_GScalarR_int(src: *const c_void, c: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// subRC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:701
	pub fn cv_gapi_subRC_const_GScalarR_const_GMatR_int(c: *const c_void, src: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// sub(const cv::GMat &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:663
	pub fn cv_gapi_sub_const_GMatR_const_GMatR_int(src1: *const c_void, src2: *const c_void, ddepth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// sum(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1266
	pub fn cv_gapi_sum_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// threshold(const cv::GMat &, const cv::GScalar &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:1419
	pub fn cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(src: *const c_void, thresh: *const c_void, maxval: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// transpose(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1876
	pub fn cv_gapi_transpose_const_GMatR(src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// warpAffine(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1787
	pub fn cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// warpPerspective(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1762
	pub fn cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(src: *const c_void, m: *const c_void, dsize: *const core::Size, flags: i32, border_mode: i32, border_value: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:16
	pub fn cv_operatorA_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:18
	pub fn cv_operatorA_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:19
	pub fn cv_operatorA_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:33
	pub fn cv_operatorD_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:31
	pub fn cv_operatorD_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:32
	pub fn cv_operatorD_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:52
	pub fn cv_operatorEQ_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:59
	pub fn cv_operatorEQ_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:66
	pub fn cv_operatorEQ_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:21
	pub fn cv_operatorS_const_GMatR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:23
	pub fn cv_operatorS_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:24
	pub fn cv_operatorS_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:28
	pub fn cv_operatorX_const_GMatR_const_GScalarR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::GMat &, float) /usr/include/opencv2/gapi/operators.hpp:26
	pub fn cv_operatorX_const_GMatR_float(lhs: *const c_void, rhs: f32, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:29
	pub fn cv_operatorX_const_GScalarR_const_GMatR(lhs: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(float, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:27
	pub fn cv_operatorX_float_const_GMatR(lhs: f32, rhs: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// validate_input_arg(const cv::GRunArg &) /usr/include/opencv2/gapi/gproto.hpp:154
	pub fn cv_validate_input_arg_const_GRunArgR(arg: *const c_void, ocvrs_return: *mut Result_void);
	// validate_input_args(const cv::GRunArgs &) /usr/include/opencv2/gapi/gproto.hpp:155
	pub fn cv_validate_input_args_const_GRunArgsR(args: *const c_void, ocvrs_return: *mut Result_void);
	// kind /usr/include/opencv2/gapi/garg.hpp:87
	pub fn cv_GArg_getPropKind_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::ArgKind);
	// kind /usr/include/opencv2/gapi/garg.hpp:87
	pub fn cv_GArg_setPropKind_ArgKind(instance: *mut c_void, val: crate::gapi::ArgKind);
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	pub fn cv_GArg_getPropOpaque_kind_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::OpaqueKind);
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	pub fn cv_GArg_setPropOpaque_kind_OpaqueKind(instance: *mut c_void, val: crate::gapi::OpaqueKind);
	// GArg() /usr/include/opencv2/gapi/garg.hpp:49
	pub fn cv_GArg_GArg(ocvrs_return: *mut Result<*mut c_void>);
	// GCall(const cv::GKernel &) /usr/include/opencv2/gapi/gcall.hpp:31
	pub fn cv_GCall_GCall_const_GKernelR(k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// yield(int) /usr/include/opencv2/gapi/gcall.hpp:42
	pub fn cv_GCall_yield_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
	// yieldP(int) /usr/include/opencv2/gapi/gcall.hpp:43
	pub fn cv_GCall_yieldP_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
	// yieldScalar(int) /usr/include/opencv2/gapi/gcall.hpp:44
	pub fn cv_GCall_yieldScalar_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
	// yieldFrame(int) /usr/include/opencv2/gapi/gcall.hpp:45
	pub fn cv_GCall_yieldFrame_int(instance: *mut c_void, output: i32, ocvrs_return: *mut Result<*mut c_void>);
	// kernel() /usr/include/opencv2/gapi/gcall.hpp:63
	pub fn cv_GCall_kernel(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// params() /usr/include/opencv2/gapi/gcall.hpp:64
	pub fn cv_GCall_params(instance: *mut c_void, ocvrs_return: *mut Result<crate::gapi::any>);
	// setArgs(std::vector<GArg> &&) /usr/include/opencv2/gapi/gcall.hpp:66
	pub fn cv_GCall_setArgs_vector_GArg_R(instance: *mut c_void, args: *mut c_void, ocvrs_return: *mut Result_void);
	// GCompiled() /usr/include/opencv2/gapi/gcompiled.hpp:75
	pub fn cv_GCompiled_GCompiled(ocvrs_return: *mut Result<*mut c_void>);
	// operator bool() /usr/include/opencv2/gapi/gcompiled.hpp:165
	pub fn cv_GCompiled_operator_bool_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// canReshape() /usr/include/opencv2/gapi/gcompiled.hpp:196
	pub fn cv_GCompiled_canReshape_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// prepareForNewStream() /usr/include/opencv2/gapi/gcompiled.hpp:222
	pub fn cv_GCompiled_prepareForNewStream(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	pub fn cv_GFrameDesc_getPropFmt_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::MediaFormat);
	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	pub fn cv_GFrameDesc_setPropFmt_MediaFormat(instance: *mut c_void, val: crate::gapi::MediaFormat);
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	pub fn cv_GFrameDesc_getPropSize_const(instance: *const c_void, ocvrs_return: *mut core::Size);
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	pub fn cv_GFrameDesc_setPropSize_Size(instance: *mut c_void, val: *const core::Size);
	// operator==(const cv::GFrameDesc &) /usr/include/opencv2/gapi/gframe.hpp:101
	pub fn cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(instance: *const c_void, unnamed: *const c_void, ocvrs_return: *mut Result<bool>);
	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	pub fn cv_GKernel_getPropName_const(instance: *const c_void) -> *mut c_void;
	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	pub fn cv_GKernel_setPropName_string(instance: *mut c_void, val: *mut c_char);
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	pub fn cv_GKernel_getPropTag_const(instance: *const c_void) -> *mut c_void;
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	pub fn cv_GKernel_setPropTag_string(instance: *mut c_void, val: *mut c_char);
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	pub fn cv_GKernel_getPropOutShapes_const(instance: *const c_void) -> *mut c_void;
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	pub fn cv_GKernel_setPropOutShapes_GShapes(instance: *mut c_void, val: *mut c_void);
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	pub fn cv_GKernel_getPropInKinds_const(instance: *const c_void) -> *mut c_void;
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	pub fn cv_GKernel_setPropInKinds_GKinds(instance: *mut c_void, val: *mut c_void);
	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	pub fn cv_GKernelImpl_getPropOpaque_const(instance: *const c_void, ocvrs_return: *mut crate::gapi::any);
	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	pub fn cv_GKernelImpl_setPropOpaque_any(instance: *mut c_void, val: crate::gapi::any);
	// GRunArg() /usr/include/opencv2/gapi/garg.hpp:129
	pub fn cv_GRunArg_GRunArg(ocvrs_return: *mut Result<*mut c_void>);
	// GRunArg(const cv::GRunArg &) /usr/include/opencv2/gapi/garg.hpp:130
	pub fn cv_GRunArg_GRunArg_const_GRunArgR(arg: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GRunArg(cv::GRunArg &&) /usr/include/opencv2/gapi/garg.hpp:131
	pub fn cv_GRunArg_GRunArg_GRunArgR(arg: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	pub fn cv_GTransform_getPropDescription_const(instance: *const c_void) -> *mut c_void;
	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	pub fn cv_GTransform_setPropDescription_string(instance: *mut c_void, val: *mut c_char);
	// MediaFrame() /usr/include/opencv2/gapi/media.hpp:70
	pub fn cv_MediaFrame_MediaFrame(ocvrs_return: *mut Result<*mut c_void>);
	// access(cv::MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:103
	pub fn cv_MediaFrame_access_const_Access(instance: *const c_void, mode: crate::gapi::MediaFrame_Access, ocvrs_return: *mut Result<*mut c_void>);
	// desc() /usr/include/opencv2/gapi/media.hpp:110
	pub fn cv_MediaFrame_desc_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// blobParams() /usr/include/opencv2/gapi/media.hpp:115
	pub fn cv_MediaFrame_blobParams_const(instance: *const c_void, ocvrs_return: *mut Result<crate::gapi::any>);
	// meta() /usr/include/opencv2/gapi/media.hpp:239
	pub fn cv_MediaFrame_IAdapter_meta_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// access(MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:240
	pub fn cv_MediaFrame_IAdapter_access_Access(instance: *mut c_void, unnamed: crate::gapi::MediaFrame_Access, ocvrs_return: *mut Result<*mut c_void>);
	// blobParams() /usr/include/opencv2/gapi/media.hpp:243
	pub fn cv_MediaFrame_IAdapter_blobParams_const(instance: *const c_void, ocvrs_return: *mut Result<crate::gapi::any>);
	// View(cv::MediaFrame::View &&) /usr/include/opencv2/gapi/media.hpp:211
	pub fn cv_MediaFrame_View_View_ViewR(unnamed: *mut c_void) -> *mut c_void;
	// RMat() /usr/include/opencv2/gapi/rmat.hpp:126
	pub fn cv_RMat_RMat() -> *mut c_void;
	// desc() /usr/include/opencv2/gapi/rmat.hpp:128
	pub fn cv_RMat_desc_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// access(cv::RMat::Access) /usr/include/opencv2/gapi/rmat.hpp:135
	pub fn cv_RMat_access_const_Access(instance: *const c_void, a: crate::gapi::RMat_Access, ocvrs_return: *mut Result<*mut c_void>);
	// desc() /usr/include/opencv2/gapi/rmat.hpp:109
	pub fn cv_RMat_IAdapter_desc_const(instance: *const dyn crate::gapi::RMat_IAdapter, ocvrs_return: *mut Result<*mut c_void>);
	// access(cv::RMat::Access) /usr/include/opencv2/gapi/rmat.hpp:113
	pub fn cv_RMat_IAdapter_access_Access(instance: *const dyn crate::gapi::RMat_IAdapter, unnamed: crate::gapi::RMat_Access, ocvrs_return: *mut Result<*mut c_void>);
	// View() /usr/include/opencv2/gapi/rmat.hpp:62
	pub fn cv_RMat_View_View() -> *mut c_void;
	// View(cv::RMat::View &&) /usr/include/opencv2/gapi/rmat.hpp:68
	pub fn cv_RMat_View_View_ViewR(unnamed: *mut c_void) -> *mut c_void;
	// size() /usr/include/opencv2/gapi/rmat.hpp:72
	pub fn cv_RMat_View_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// dims() /usr/include/opencv2/gapi/rmat.hpp:73
	pub fn cv_RMat_View_dims_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cols() /usr/include/opencv2/gapi/rmat.hpp:74
	pub fn cv_RMat_View_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// rows() /usr/include/opencv2/gapi/rmat.hpp:75
	pub fn cv_RMat_View_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// type() /usr/include/opencv2/gapi/rmat.hpp:76
	pub fn cv_RMat_View_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// depth() /usr/include/opencv2/gapi/rmat.hpp:77
	pub fn cv_RMat_View_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// chan() /usr/include/opencv2/gapi/rmat.hpp:78
	pub fn cv_RMat_View_chan_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// elemSize() /usr/include/opencv2/gapi/rmat.hpp:79
	pub fn cv_RMat_View_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// step(size_t) /usr/include/opencv2/gapi/rmat.hpp:93
	pub fn cv_RMat_View_step_const_size_t(instance: *const c_void, i: size_t, ocvrs_return: *mut Result<size_t>);
	// steps() /usr/include/opencv2/gapi/rmat.hpp:94
	pub fn cv_RMat_View_steps_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GBackend() /usr/include/opencv2/gapi/gkernel.hpp:382
	pub fn cv_gapi_GBackend_GBackend(ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::gapi::GBackend &) /usr/include/opencv2/gapi/gkernel.hpp:389
	pub fn cv_gapi_GBackend_operatorEQ_const_const_GBackendR(instance: *const c_void, rhs: *const c_void, ocvrs_return: *mut Result<bool>);
	// val /usr/include/opencv2/gapi/own/scalar.hpp:35
	pub fn cv_gapi_own_Scalar_getPropVal(instance: *mut c_void) -> *mut [f64; 4];
	// Scalar() /usr/include/opencv2/gapi/own/scalar.hpp:23
	pub fn cv_gapi_own_Scalar_Scalar() -> *mut c_void;
	// Scalar(double) /usr/include/opencv2/gapi/own/scalar.hpp:24
	pub fn cv_gapi_own_Scalar_Scalar_double(v0: f64, ocvrs_return: *mut Result<*mut c_void>);
	// Scalar(double, double, double, double) /usr/include/opencv2/gapi/own/scalar.hpp:25
	pub fn cv_gapi_own_Scalar_Scalar_double_double_double_double(v0: f64, v1: f64, v2: f64, v3: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:30
	pub fn cv_gapi_own_Scalar_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<f64>);
	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:31
	pub fn cv_gapi_own_Scalar_operator___int(instance: *mut c_void, i: i32, ocvrs_return: *mut Result<f64>);
	// all(double) /usr/include/opencv2/gapi/own/scalar.hpp:33
	pub fn cv_gapi_own_Scalar_all_double(v0: f64, ocvrs_return: *mut Result<*mut c_void>);
	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	pub fn cv_gapi_use_only_getPropPkg_const(instance: *const c_void) -> *mut c_void;
	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	pub fn cv_gapi_use_only_setPropPkg_GKernelPackage(instance: *mut c_void, val: *mut c_void);
}
