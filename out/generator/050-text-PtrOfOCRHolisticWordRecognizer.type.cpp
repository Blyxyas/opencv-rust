extern "C" {
	void cv_PtrOfOCRHolisticWordRecognizer_delete(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
		delete instance;
	}

	const cv::text::OCRHolisticWordRecognizer* cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr(const cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
		return instance->get();
	}

	cv::text::OCRHolisticWordRecognizer* cv_PtrOfOCRHolisticWordRecognizer_get_inner_ptr_mut(cv::Ptr<cv::text::OCRHolisticWordRecognizer>* instance) {
		return instance->get();
	}
}

