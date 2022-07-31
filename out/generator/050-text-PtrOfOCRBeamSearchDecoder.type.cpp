extern "C" {
	cv::Ptr<cv::text::OCRBeamSearchDecoder>* cv_PtrOfOCRBeamSearchDecoder_new(cv::text::OCRBeamSearchDecoder* val) {
		return new cv::Ptr<cv::text::OCRBeamSearchDecoder>(val);
	}
	
	void cv_PtrOfOCRBeamSearchDecoder_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
		delete instance;
	}

	const cv::text::OCRBeamSearchDecoder* cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr(const cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
		return instance->get();
	}

	cv::text::OCRBeamSearchDecoder* cv_PtrOfOCRBeamSearchDecoder_get_inner_ptr_mut(cv::Ptr<cv::text::OCRBeamSearchDecoder>* instance) {
		return instance->get();
	}
}

