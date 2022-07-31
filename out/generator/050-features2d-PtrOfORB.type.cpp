extern "C" {
	void cv_PtrOfORB_delete(cv::Ptr<cv::ORB>* instance) {
		delete instance;
	}

	const cv::ORB* cv_PtrOfORB_get_inner_ptr(const cv::Ptr<cv::ORB>* instance) {
		return instance->get();
	}

	cv::ORB* cv_PtrOfORB_get_inner_ptr_mut(cv::Ptr<cv::ORB>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfORB_to_PtrOfFeature2D(cv::Ptr<cv::ORB>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

