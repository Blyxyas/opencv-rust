extern "C" {
	cv::Ptr<cv::text::OCRHMMDecoder>* cv_PtrOfOCRHMMDecoder_new(cv::text::OCRHMMDecoder* val) {
		return new cv::Ptr<cv::text::OCRHMMDecoder>(val);
	}
	
	void cv_PtrOfOCRHMMDecoder_delete(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
		delete instance;
	}

	const cv::text::OCRHMMDecoder* cv_PtrOfOCRHMMDecoder_get_inner_ptr(const cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
		return instance->get();
	}

	cv::text::OCRHMMDecoder* cv_PtrOfOCRHMMDecoder_get_inner_ptr_mut(cv::Ptr<cv::text::OCRHMMDecoder>* instance) {
		return instance->get();
	}
}

