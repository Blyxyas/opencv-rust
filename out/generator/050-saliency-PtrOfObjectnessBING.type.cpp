extern "C" {
	cv::Ptr<cv::saliency::ObjectnessBING>* cv_PtrOfObjectnessBING_new(cv::saliency::ObjectnessBING* val) {
		return new cv::Ptr<cv::saliency::ObjectnessBING>(val);
	}
	
	void cv_PtrOfObjectnessBING_delete(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
		delete instance;
	}

	const cv::saliency::ObjectnessBING* cv_PtrOfObjectnessBING_get_inner_ptr(const cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
		return instance->get();
	}

	cv::saliency::ObjectnessBING* cv_PtrOfObjectnessBING_get_inner_ptr_mut(cv::Ptr<cv::saliency::ObjectnessBING>* instance) {
		return instance->get();
	}
}

