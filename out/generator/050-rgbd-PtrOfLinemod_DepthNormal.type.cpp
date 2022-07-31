extern "C" {
	cv::Ptr<cv::linemod::DepthNormal>* cv_PtrOfLinemod_DepthNormal_new(cv::linemod::DepthNormal* val) {
		return new cv::Ptr<cv::linemod::DepthNormal>(val);
	}
	
	void cv_PtrOfLinemod_DepthNormal_delete(cv::Ptr<cv::linemod::DepthNormal>* instance) {
		delete instance;
	}

	const cv::linemod::DepthNormal* cv_PtrOfLinemod_DepthNormal_get_inner_ptr(const cv::Ptr<cv::linemod::DepthNormal>* instance) {
		return instance->get();
	}

	cv::linemod::DepthNormal* cv_PtrOfLinemod_DepthNormal_get_inner_ptr_mut(cv::Ptr<cv::linemod::DepthNormal>* instance) {
		return instance->get();
	}
}

