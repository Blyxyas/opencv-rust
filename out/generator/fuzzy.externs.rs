extern "C" {
	// FT02D_FL_process(cv::InputArray, const int, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:106
	pub fn cv_ft_FT02D_FL_process_const__InputArrayR_const_int_const__OutputArrayR(matrix: *const c_void, radius: i32, output: *const c_void, ocvrs_return: *mut Result_void);
	// FT02D_FL_process_float(cv::InputArray, const int, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:115
	pub fn cv_ft_FT02D_FL_process_float_const__InputArrayR_const_int_const__OutputArrayR(matrix: *const c_void, radius: i32, output: *const c_void, ocvrs_return: *mut Result_void);
	// FT02D_components(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:64
	pub fn cv_ft_FT02D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix: *const c_void, kernel: *const c_void, components: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// FT02D_inverseFT(cv::InputArray, cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:75
	pub fn cv_ft_FT02D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components: *const c_void, kernel: *const c_void, output: *const c_void, width: i32, height: i32, ocvrs_return: *mut Result_void);
	// FT02D_iteration(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:97
	pub fn cv_ft_FT02D_iteration_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_const__OutputArrayR_bool(matrix: *const c_void, kernel: *const c_void, output: *const c_void, mask: *const c_void, mask_output: *const c_void, first_stop: bool, ocvrs_return: *mut Result<i32>);
	// FT02D_process(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F0_math.hpp:85
	pub fn cv_ft_FT02D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix: *const c_void, kernel: *const c_void, output: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// FT12D_components(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:63
	pub fn cv_ft_FT12D_components_const__InputArrayR_const__InputArrayR_const__OutputArrayR(matrix: *const c_void, kernel: *const c_void, components: *const c_void, ocvrs_return: *mut Result_void);
	// FT12D_createPolynomMatrixHorizontal(int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:94
	pub fn cv_ft_FT12D_createPolynomMatrixHorizontal_int_const__OutputArrayR_const_int(radius: i32, matrix: *const c_void, chn: i32, ocvrs_return: *mut Result_void);
	// FT12D_createPolynomMatrixVertical(int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:85
	pub fn cv_ft_FT12D_createPolynomMatrixVertical_int_const__OutputArrayR_const_int(radius: i32, matrix: *const c_void, chn: i32, ocvrs_return: *mut Result_void);
	// FT12D_inverseFT(cv::InputArray, cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:118
	pub fn cv_ft_FT12D_inverseFT_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int(components: *const c_void, kernel: *const c_void, output: *const c_void, width: i32, height: i32, ocvrs_return: *mut Result_void);
	// FT12D_polynomial(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:76
	pub fn cv_ft_FT12D_polynomial_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(matrix: *const c_void, kernel: *const c_void, c00: *const c_void, c10: *const c_void, c01: *const c_void, components: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// FT12D_process(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/fuzzy/fuzzy_F1_math.hpp:107
	pub fn cv_ft_FT12D_process_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(matrix: *const c_void, kernel: *const c_void, output: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// createKernel(cv::InputArray, cv::InputArray, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:64
	pub fn cv_ft_createKernel_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const_int(a: *const c_void, b: *const c_void, kernel: *const c_void, chn: i32, ocvrs_return: *mut Result_void);
	// createKernel(int, int, cv::OutputArray, const int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:75
	pub fn cv_ft_createKernel_int_int_const__OutputArrayR_const_int(function: i32, radius: i32, kernel: *const c_void, chn: i32, ocvrs_return: *mut Result_void);
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:103
	pub fn cv_ft_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(image: *const c_void, kernel: *const c_void, output: *const c_void, ocvrs_return: *mut Result_void);
	// inpaint(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/fuzzy/fuzzy_image.hpp:94
	pub fn cv_ft_inpaint_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_int(image: *const c_void, mask: *const c_void, output: *const c_void, radius: i32, function: i32, algorithm: i32, ocvrs_return: *mut Result_void);
}
