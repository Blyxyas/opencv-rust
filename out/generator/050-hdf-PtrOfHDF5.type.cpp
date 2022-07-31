extern "C" {
	void cv_PtrOfHDF5_delete(cv::Ptr<cv::hdf::HDF5>* instance) {
		delete instance;
	}

	const cv::hdf::HDF5* cv_PtrOfHDF5_get_inner_ptr(const cv::Ptr<cv::hdf::HDF5>* instance) {
		return instance->get();
	}

	cv::hdf::HDF5* cv_PtrOfHDF5_get_inner_ptr_mut(cv::Ptr<cv::hdf::HDF5>* instance) {
		return instance->get();
	}
}

