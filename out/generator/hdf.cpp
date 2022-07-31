#include "hdf.hpp"
#include "hdf_types.hpp"

extern "C" {
	// open(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:802
	void cv_hdf_open_const_StringR(const char* HDF5Filename, Result<cv::Ptr<cv::hdf::HDF5>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::hdf::HDF5> ret = cv::hdf::open(std::string(HDF5Filename));
			Ok(new cv::Ptr<cv::hdf::HDF5>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::hdf::HDF5>*>))
	}
	
	// close() /usr/include/opencv2/hdf/hdf5.hpp:73
	void cv_hdf_HDF5_close(cv::hdf::HDF5* instance, Result_void* ocvrs_return) {
		try {
			instance->close();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// grcreate(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:95
	void cv_hdf_HDF5_grcreate_const_StringR(cv::hdf::HDF5* instance, const char* grlabel, Result_void* ocvrs_return) {
		try {
			instance->grcreate(std::string(grlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// hlexists(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:104
	void cv_hdf_HDF5_hlexists_const_const_StringR(const cv::hdf::HDF5* instance, const char* label, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hlexists(std::string(label));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// atexists(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:114
	void cv_hdf_HDF5_atexists_const_const_StringR(const cv::hdf::HDF5* instance, const char* atlabel, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->atexists(std::string(atlabel));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// atdelete(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:126
	void cv_hdf_HDF5_atdelete_const_StringR(cv::hdf::HDF5* instance, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atdelete(std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atwrite(const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:144
	void cv_hdf_HDF5_atwrite_const_int_const_StringR(cv::hdf::HDF5* instance, const int value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atwrite(value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atread(int *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:161
	void cv_hdf_HDF5_atread_intX_const_StringR(cv::hdf::HDF5* instance, int* value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atread(value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atwrite(const double, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:164
	void cv_hdf_HDF5_atwrite_const_double_const_StringR(cv::hdf::HDF5* instance, const double value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atwrite(value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atread(double *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:167
	void cv_hdf_HDF5_atread_doubleX_const_StringR(cv::hdf::HDF5* instance, double* value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atread(value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atwrite(const cv::String &, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:170
	void cv_hdf_HDF5_atwrite_const_StringR_const_StringR(cv::hdf::HDF5* instance, const char* value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atwrite(std::string(value), std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atread(cv::String *, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:173
	void cv_hdf_HDF5_atread_StringX_const_StringR(cv::hdf::HDF5* instance, void** value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			std::string value_out;
			instance->atread(&value_out, std::string(atlabel));
			*value = ocvrs_create_string(value_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atwrite(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:187
	void cv_hdf_HDF5_atwrite_const__InputArrayR_const_StringR(cv::hdf::HDF5* instance, const cv::_InputArray* value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atwrite(*value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// atread(cv::OutputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:200
	void cv_hdf_HDF5_atread_const__OutputArrayR_const_StringR(cv::hdf::HDF5* instance, const cv::_OutputArray* value, const char* atlabel, Result_void* ocvrs_return) {
		try {
			instance->atread(*value, std::string(atlabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int, const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:203
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, Result_void* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, std::string(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int, const int, const cv::String &, const int) /usr/include/opencv2/hdf/hdf5.hpp:206
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, const int compresslevel, Result_void* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, std::string(dslabel), compresslevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int, const int, const cv::String &, const int, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:209
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_vector_int_R(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, const int compresslevel, const std::vector<int>* dims_chunks, Result_void* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, std::string(dslabel), compresslevel, *dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int, const int, const cv::String &, const int, const int *) /usr/include/opencv2/hdf/hdf5.hpp:278
	void cv_hdf_HDF5_dscreate_const_const_int_const_int_const_int_const_StringR_const_int_const_intX(const cv::hdf::HDF5* instance, const int rows, const int cols, const int type, const char* dslabel, const int compresslevel, const int* dims_chunks, Result_void* ocvrs_return) {
		try {
			instance->dscreate(rows, cols, type, std::string(dslabel), compresslevel, dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int *, const int, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:282
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, Result_void* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, std::string(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int *, const int, const cv::String &, const int) /usr/include/opencv2/hdf/hdf5.hpp:285
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, const int compresslevel, Result_void* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, std::string(dslabel), compresslevel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const vector<int> &, const int, const cv::String &, const int, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:288
	void cv_hdf_HDF5_dscreate_const_const_vector_int_R_const_int_const_StringR_const_int_const_vector_int_R(const cv::hdf::HDF5* instance, const std::vector<int>* sizes, const int type, const char* dslabel, const int compresslevel, const std::vector<int>* dims_chunks, Result_void* ocvrs_return) {
		try {
			instance->dscreate(*sizes, type, std::string(dslabel), compresslevel, *dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dscreate(const int, const int *, const int, const cv::String &, const int, const int *) /usr/include/opencv2/hdf/hdf5.hpp:362
	void cv_hdf_HDF5_dscreate_const_const_int_const_intX_const_int_const_StringR_const_int_const_intX(const cv::hdf::HDF5* instance, const int n_dims, const int* sizes, const int type, const char* dslabel, const int compresslevel, const int* dims_chunks, Result_void* ocvrs_return) {
		try {
			instance->dscreate(n_dims, sizes, type, std::string(dslabel), compresslevel, dims_chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsgetsize(const cv::String &, int) /usr/include/opencv2/hdf/hdf5.hpp:380
	void cv_hdf_HDF5_dsgetsize_const_const_StringR_int(const cv::hdf::HDF5* instance, const char* dslabel, int dims_flag, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->dsgetsize(std::string(dslabel), dims_flag);
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// dsgettype(const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:391
	void cv_hdf_HDF5_dsgettype_const_const_StringR(const cv::hdf::HDF5* instance, const char* dslabel, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dsgettype(std::string(dslabel));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// dswrite(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:394
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, Result_void* ocvrs_return) {
		try {
			instance->dswrite(*Array, std::string(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dswrite(cv::InputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:396
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const int* dims_offset, Result_void* ocvrs_return) {
		try {
			instance->dswrite(*Array, std::string(dslabel), dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dswrite(cv::InputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:399
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dswrite(*Array, std::string(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dswrite(cv::InputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:464
	void cv_hdf_HDF5_dswrite_const_const__InputArrayR_const_StringR_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const int* dims_offset, const int* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dswrite(*Array, std::string(dslabel), dims_offset, dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsinsert(cv::InputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:468
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, Result_void* ocvrs_return) {
		try {
			instance->dsinsert(*Array, std::string(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsinsert(cv::InputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:470
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const int* dims_offset, Result_void* ocvrs_return) {
		try {
			instance->dsinsert(*Array, std::string(dslabel), dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsinsert(cv::InputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:473
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dsinsert(*Array, std::string(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsinsert(cv::InputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:523
	void cv_hdf_HDF5_dsinsert_const_const__InputArrayR_const_StringR_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_InputArray* Array, const char* dslabel, const int* dims_offset, const int* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dsinsert(*Array, std::string(dslabel), dims_offset, dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsread(cv::OutputArray, const cv::String &) /usr/include/opencv2/hdf/hdf5.hpp:528
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, Result_void* ocvrs_return) {
		try {
			instance->dsread(*Array, std::string(dslabel));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsread(cv::OutputArray, const cv::String &, const int *) /usr/include/opencv2/hdf/hdf5.hpp:530
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, const int* dims_offset, Result_void* ocvrs_return) {
		try {
			instance->dsread(*Array, std::string(dslabel), dims_offset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsread(cv::OutputArray, const cv::String &, const vector<int> &, const vector<int> &) /usr/include/opencv2/hdf/hdf5.hpp:533
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_vector_int_R_const_vector_int_R(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, const std::vector<int>* dims_offset, const std::vector<int>* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dsread(*Array, std::string(dslabel), *dims_offset, *dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dsread(cv::OutputArray, const cv::String &, const int *, const int *) /usr/include/opencv2/hdf/hdf5.hpp:575
	void cv_hdf_HDF5_dsread_const_const__OutputArrayR_const_StringR_const_intX_const_intX(const cv::hdf::HDF5* instance, const cv::_OutputArray* Array, const char* dslabel, const int* dims_offset, const int* dims_counts, Result_void* ocvrs_return) {
		try {
			instance->dsread(*Array, std::string(dslabel), dims_offset, dims_counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// kpgetsize(const cv::String &, int) /usr/include/opencv2/hdf/hdf5.hpp:591
	void cv_hdf_HDF5_kpgetsize_const_const_StringR_int(const cv::hdf::HDF5* instance, const char* kplabel, int dims_flag, Result<int>* ocvrs_return) {
		try {
			int ret = instance->kpgetsize(std::string(kplabel), dims_flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// kpcreate(const int, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:628
	void cv_hdf_HDF5_kpcreate_const_const_int_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const int size, const char* kplabel, const int compresslevel, const int chunks, Result_void* ocvrs_return) {
		try {
			instance->kpcreate(size, std::string(kplabel), compresslevel, chunks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// kpwrite(const vector<cv::KeyPoint>, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:681
	void cv_hdf_HDF5_kpwrite_const_const_vector_KeyPoint__const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, Result_void* ocvrs_return) {
		try {
			instance->kpwrite(*keypoints, std::string(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// kpinsert(const vector<cv::KeyPoint>, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:716
	void cv_hdf_HDF5_kpinsert_const_const_vector_KeyPoint__const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, const std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, Result_void* ocvrs_return) {
		try {
			instance->kpinsert(*keypoints, std::string(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// kpread(vector<cv::KeyPoint> &, const cv::String &, const int, const int) /usr/include/opencv2/hdf/hdf5.hpp:754
	void cv_hdf_HDF5_kpread_const_vector_KeyPoint_R_const_StringR_const_int_const_int(const cv::hdf::HDF5* instance, std::vector<cv::KeyPoint>* keypoints, const char* kplabel, const int offset, const int counts, Result_void* ocvrs_return) {
		try {
			instance->kpread(*keypoints, std::string(kplabel), offset, counts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
