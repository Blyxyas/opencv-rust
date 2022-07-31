extern "C" {
	void cv_PtrOfEigenFaceRecognizer_delete(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
		delete instance;
	}

	const cv::face::EigenFaceRecognizer* cv_PtrOfEigenFaceRecognizer_get_inner_ptr(const cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
		return instance->get();
	}

	cv::face::EigenFaceRecognizer* cv_PtrOfEigenFaceRecognizer_get_inner_ptr_mut(cv::Ptr<cv::face::EigenFaceRecognizer>* instance) {
		return instance->get();
	}
}

