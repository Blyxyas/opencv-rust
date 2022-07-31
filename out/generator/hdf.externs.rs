extern "C" {
	// open(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:802
	pub fn cv_hdf_open_const_StringR(hdf5_filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// close() /usr/include/opencv2/hdf/hdf5.hpp:73
	pub fn cv_hdf_HDF5_close(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// grcreate(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:95
	pub fn cv_hdf_HDF5_grcreate_const_StringR(instance: *mut c_void, grlabel: *const c_char, ocvrs_return: *mut Result_void);
	// hlexists(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:104
	pub fn cv_hdf_HDF5_hlexists_const_const_StringR(instance: *const c_void, label: *const c_char, ocvrs_return: *mut Result<bool>);
	// atexists(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:114
	pub fn cv_hdf_HDF5_atexists_const_const_StringR(instance: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result<bool>);
	// atdelete(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:126
	pub fn cv_hdf_HDF5_atdelete_const_StringR(instance: *mut c_void, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atwrite(const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:144
	pub fn cv_hdf_HDF5_atwrite_const_int_const_StringR(instance: *mut c_void, value: i32, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atread(int *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:161
	pub fn cv_hdf_HDF5_atread_intX_const_StringR(instance: *mut c_void, value: *mut i32, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atwrite(const double, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:164
	pub fn cv_hdf_HDF5_atwrite_const_double_const_StringR(instance: *mut c_void, value: f64, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atread(double *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:167
	pub fn cv_hdf_HDF5_atread_doubleX_const_StringR(instance: *mut c_void, value: *mut f64, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atwrite(const cv::String &, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:170
	pub fn cv_hdf_HDF5_atwrite_const_StringR_const_StringR(instance: *mut c_void, value: *const c_char, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atread(cv::String *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:173
	pub fn cv_hdf_HDF5_atread_StringX_const_StringR(instance: *mut c_void, value: *mut *mut c_void, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atwrite(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:187
	pub fn cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(instance: *mut c_void, value: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// atread(cv::OutputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:200
	pub fn cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(instance: *mut c_void, value: *const c_void, atlabel: *const c_char, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int, const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:203
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int, const int, const cv::String &, const int) /usr/include/opencv2/hdf/hdf5.hpp:206
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, compresslevel: i32, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int, const int, const cv::String &, const int, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:209
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vector_int_R(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const c_void, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int, const int, const cv::String &, const int, const int *) /usr/include/opencv2/hdf/hdf5.hpp:278
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_intX(instance: *const c_void, rows: i32, cols: i32, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const i32, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int *, const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:282
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int *, const int, const cv::String &, const int) /usr/include/opencv2/hdf/hdf5.hpp:285
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, compresslevel: i32, ocvrs_return: *mut Result_void);
	// dscreate(const vector<int> &, const int, const cv::String &, const int, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:288
	pub fn cv_hdf_HDF5_dscreate_const_const_vector_int_R_const_int_const_StringR_const_int_const_vector_int_R(instance: *const c_void, sizes: *const c_void, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const c_void, ocvrs_return: *mut Result_void);
	// dscreate(const int, const int *, const int, const cv::String &, const int, const int *) /usr/include/opencv2/hdf/hdf5.hpp:362
	pub fn cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(instance: *const c_void, n_dims: i32, sizes: *const i32, typ: i32, dslabel: *const c_char, compresslevel: i32, dims_chunks: *const i32, ocvrs_return: *mut Result_void);
	// dsgetsize(const cv::String &, int) /usr/include/opencv2/hdf/hdf5.hpp:380
	pub fn cv_hdf_HDF5_dsgetsize_const_const_StringR_int(instance: *const c_void, dslabel: *const c_char, dims_flag: i32, ocvrs_return: *mut Result<*mut c_void>);
	// dsgettype(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:391
	pub fn cv_hdf_HDF5_dsgettype_const_const_StringR(instance: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result<i32>);
	// dswrite(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:394
	pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result_void);
	// dswrite(cv::InputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:396
	pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, ocvrs_return: *mut Result_void);
	// dswrite(cv::InputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:399
	pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result_void);
	// dswrite(cv::InputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:464
	pub fn cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, dims_counts: *const i32, ocvrs_return: *mut Result_void);
	// dsinsert(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:468
	pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result_void);
	// dsinsert(cv::InputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:470
	pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, ocvrs_return: *mut Result_void);
	// dsinsert(cv::InputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:473
	pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result_void);
	// dsinsert(cv::InputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:523
	pub fn cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, dims_counts: *const i32, ocvrs_return: *mut Result_void);
	// dsread(cv::OutputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:528
	pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(instance: *const c_void, array: *const c_void, dslabel: *const c_char, ocvrs_return: *mut Result_void);
	// dsread(cv::OutputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:530
	pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, ocvrs_return: *mut Result_void);
	// dsread(cv::OutputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:533
	pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const c_void, dims_counts: *const c_void, ocvrs_return: *mut Result_void);
	// dsread(cv::OutputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:575
	pub fn cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX_const_intX(instance: *const c_void, array: *const c_void, dslabel: *const c_char, dims_offset: *const i32, dims_counts: *const i32, ocvrs_return: *mut Result_void);
	// kpgetsize(const cv::String &, int) /usr/include/opencv2/hdf/hdf5.hpp:591
	pub fn cv_hdf_HDF5_kpgetsize_const_const_StringR_int(instance: *const c_void, kplabel: *const c_char, dims_flag: i32, ocvrs_return: *mut Result<i32>);
	// kpcreate(const int, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:628
	pub fn cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(instance: *const c_void, size: i32, kplabel: *const c_char, compresslevel: i32, chunks: i32, ocvrs_return: *mut Result_void);
	// kpwrite(const vector<cv::KeyPoint>, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:681
	pub fn cv_hdf_HDF5_kpwrite_const_const_vector_KeyPoint__const_StringR_const_int_const_int(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result_void);
	// kpinsert(const vector<cv::KeyPoint>, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:716
	pub fn cv_hdf_HDF5_kpinsert_const_const_vector_KeyPoint__const_StringR_const_int_const_int(instance: *const c_void, keypoints: *const c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result_void);
	// kpread(vector<cv::KeyPoint> &, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:754
	pub fn cv_hdf_HDF5_kpread_const_vector_KeyPoint_R_const_StringR_const_int_const_int(instance: *const c_void, keypoints: *mut c_void, kplabel: *const c_char, offset: i32, counts: i32, ocvrs_return: *mut Result_void);
}
