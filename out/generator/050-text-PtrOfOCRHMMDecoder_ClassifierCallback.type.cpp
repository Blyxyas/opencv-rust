extern "C" {
	cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* cv_PtrOfOCRHMMDecoder_ClassifierCallback_new(cv::text::OCRHMMDecoder::ClassifierCallback* val) {
		return new cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>(val);
	}
	
	void cv_PtrOfOCRHMMDecoder_ClassifierCallback_delete(cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* instance) {
		delete instance;
	}

	const cv::text::OCRHMMDecoder::ClassifierCallback* cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr(const cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* instance) {
		return instance->get();
	}

	cv::text::OCRHMMDecoder::ClassifierCallback* cv_PtrOfOCRHMMDecoder_ClassifierCallback_get_inner_ptr_mut(cv::Ptr<cv::text::OCRHMMDecoder::ClassifierCallback>* instance) {
		return instance->get();
	}
}

