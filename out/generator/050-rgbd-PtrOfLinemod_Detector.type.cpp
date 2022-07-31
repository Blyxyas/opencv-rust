extern "C" {
	cv::Ptr<cv::linemod::Detector>* cv_PtrOfLinemod_Detector_new(cv::linemod::Detector* val) {
		return new cv::Ptr<cv::linemod::Detector>(val);
	}
	
	void cv_PtrOfLinemod_Detector_delete(cv::Ptr<cv::linemod::Detector>* instance) {
		delete instance;
	}

	const cv::linemod::Detector* cv_PtrOfLinemod_Detector_get_inner_ptr(const cv::Ptr<cv::linemod::Detector>* instance) {
		return instance->get();
	}

	cv::linemod::Detector* cv_PtrOfLinemod_Detector_get_inner_ptr_mut(cv::Ptr<cv::linemod::Detector>* instance) {
		return instance->get();
	}
}

