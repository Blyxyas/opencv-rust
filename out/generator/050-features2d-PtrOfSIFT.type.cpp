extern "C" {
	cv::Ptr<cv::SIFT>* cv_PtrOfSIFT_new(cv::SIFT* val) {
		return new cv::Ptr<cv::SIFT>(val);
	}
	
	void cv_PtrOfSIFT_delete(cv::Ptr<cv::SIFT>* instance) {
		delete instance;
	}

	const cv::SIFT* cv_PtrOfSIFT_get_inner_ptr(const cv::Ptr<cv::SIFT>* instance) {
		return instance->get();
	}

	cv::SIFT* cv_PtrOfSIFT_get_inner_ptr_mut(cv::Ptr<cv::SIFT>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfSIFT_to_PtrOfFeature2D(cv::Ptr<cv::SIFT>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

