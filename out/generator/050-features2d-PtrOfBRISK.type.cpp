extern "C" {
	cv::Ptr<cv::BRISK>* cv_PtrOfBRISK_new(cv::BRISK* val) {
		return new cv::Ptr<cv::BRISK>(val);
	}
	
	void cv_PtrOfBRISK_delete(cv::Ptr<cv::BRISK>* instance) {
		delete instance;
	}

	const cv::BRISK* cv_PtrOfBRISK_get_inner_ptr(const cv::Ptr<cv::BRISK>* instance) {
		return instance->get();
	}

	cv::BRISK* cv_PtrOfBRISK_get_inner_ptr_mut(cv::Ptr<cv::BRISK>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfBRISK_to_PtrOfFeature2D(cv::Ptr<cv::BRISK>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

