extern "C" {
	void cv_PtrOfLBPHFaceRecognizer_delete(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
		delete instance;
	}

	const cv::face::LBPHFaceRecognizer* cv_PtrOfLBPHFaceRecognizer_get_inner_ptr(const cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
		return instance->get();
	}

	cv::face::LBPHFaceRecognizer* cv_PtrOfLBPHFaceRecognizer_get_inner_ptr_mut(cv::Ptr<cv::face::LBPHFaceRecognizer>* instance) {
		return instance->get();
	}
}

