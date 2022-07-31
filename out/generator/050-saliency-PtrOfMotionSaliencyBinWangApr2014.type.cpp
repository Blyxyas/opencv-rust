extern "C" {
	cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* cv_PtrOfMotionSaliencyBinWangApr2014_new(cv::saliency::MotionSaliencyBinWangApr2014* val) {
		return new cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>(val);
	}
	
	void cv_PtrOfMotionSaliencyBinWangApr2014_delete(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
		delete instance;
	}

	const cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr(const cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
		return instance->get();
	}

	cv::saliency::MotionSaliencyBinWangApr2014* cv_PtrOfMotionSaliencyBinWangApr2014_get_inner_ptr_mut(cv::Ptr<cv::saliency::MotionSaliencyBinWangApr2014>* instance) {
		return instance->get();
	}
}

