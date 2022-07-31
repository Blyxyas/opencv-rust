extern "C" {
	cv::Ptr<cv::linemod::ColorGradient>* cv_PtrOfLinemod_ColorGradient_new(cv::linemod::ColorGradient* val) {
		return new cv::Ptr<cv::linemod::ColorGradient>(val);
	}
	
	void cv_PtrOfLinemod_ColorGradient_delete(cv::Ptr<cv::linemod::ColorGradient>* instance) {
		delete instance;
	}

	const cv::linemod::ColorGradient* cv_PtrOfLinemod_ColorGradient_get_inner_ptr(const cv::Ptr<cv::linemod::ColorGradient>* instance) {
		return instance->get();
	}

	cv::linemod::ColorGradient* cv_PtrOfLinemod_ColorGradient_get_inner_ptr_mut(cv::Ptr<cv::linemod::ColorGradient>* instance) {
		return instance->get();
	}
}

