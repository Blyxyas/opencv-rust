extern "C" {
	cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_new(cv::text::OCRBeamSearchDecoder::ClassifierCallback* val) {
		return new cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>(val);
	}
	
	void cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_delete(cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* instance) {
		delete instance;
	}

	const cv::text::OCRBeamSearchDecoder::ClassifierCallback* cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr(const cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* instance) {
		return instance->get();
	}

	cv::text::OCRBeamSearchDecoder::ClassifierCallback* cv_PtrOfOCRBeamSearchDecoder_ClassifierCallback_get_inner_ptr_mut(cv::Ptr<cv::text::OCRBeamSearchDecoder::ClassifierCallback>* instance) {
		return instance->get();
	}
}

