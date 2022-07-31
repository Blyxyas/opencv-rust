extern "C" {
	void cv_PtrOfFisherFaceRecognizer_delete(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
		delete instance;
	}

	const cv::face::FisherFaceRecognizer* cv_PtrOfFisherFaceRecognizer_get_inner_ptr(const cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
		return instance->get();
	}

	cv::face::FisherFaceRecognizer* cv_PtrOfFisherFaceRecognizer_get_inner_ptr_mut(cv::Ptr<cv::face::FisherFaceRecognizer>* instance) {
		return instance->get();
	}
}

