extern "C" {
	void cv_PtrOfOCRTesseract_delete(cv::Ptr<cv::text::OCRTesseract>* instance) {
		delete instance;
	}

	const cv::text::OCRTesseract* cv_PtrOfOCRTesseract_get_inner_ptr(const cv::Ptr<cv::text::OCRTesseract>* instance) {
		return instance->get();
	}

	cv::text::OCRTesseract* cv_PtrOfOCRTesseract_get_inner_ptr_mut(cv::Ptr<cv::text::OCRTesseract>* instance) {
		return instance->get();
	}
}

