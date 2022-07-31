extern "C" {
	cv::Ptr<cv::img_hash::RadialVarianceHash>* cv_PtrOfRadialVarianceHash_new(cv::img_hash::RadialVarianceHash* val) {
		return new cv::Ptr<cv::img_hash::RadialVarianceHash>(val);
	}
	
	void cv_PtrOfRadialVarianceHash_delete(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
		delete instance;
	}

	const cv::img_hash::RadialVarianceHash* cv_PtrOfRadialVarianceHash_get_inner_ptr(const cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
		return instance->get();
	}

	cv::img_hash::RadialVarianceHash* cv_PtrOfRadialVarianceHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::RadialVarianceHash>* instance) {
		return instance->get();
	}
}

