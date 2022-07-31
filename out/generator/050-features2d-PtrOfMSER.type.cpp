extern "C" {
	void cv_PtrOfMSER_delete(cv::Ptr<cv::MSER>* instance) {
		delete instance;
	}

	const cv::MSER* cv_PtrOfMSER_get_inner_ptr(const cv::Ptr<cv::MSER>* instance) {
		return instance->get();
	}

	cv::MSER* cv_PtrOfMSER_get_inner_ptr_mut(cv::Ptr<cv::MSER>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfMSER_to_PtrOfFeature2D(cv::Ptr<cv::MSER>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

