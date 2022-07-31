extern "C" {
	cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* cv_PtrOfStaticSaliencyFineGrained_new(cv::saliency::StaticSaliencyFineGrained* val) {
		return new cv::Ptr<cv::saliency::StaticSaliencyFineGrained>(val);
	}
	
	void cv_PtrOfStaticSaliencyFineGrained_delete(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
		delete instance;
	}

	const cv::saliency::StaticSaliencyFineGrained* cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr(const cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
		return instance->get();
	}

	cv::saliency::StaticSaliencyFineGrained* cv_PtrOfStaticSaliencyFineGrained_get_inner_ptr_mut(cv::Ptr<cv::saliency::StaticSaliencyFineGrained>* instance) {
		return instance->get();
	}
}

