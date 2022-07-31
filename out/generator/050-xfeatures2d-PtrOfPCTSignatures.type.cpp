extern "C" {
	void cv_PtrOfPCTSignatures_delete(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		delete instance;
	}

	const cv::xfeatures2d::PCTSignatures* cv_PtrOfPCTSignatures_get_inner_ptr(const cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		return instance->get();
	}

	cv::xfeatures2d::PCTSignatures* cv_PtrOfPCTSignatures_get_inner_ptr_mut(cv::Ptr<cv::xfeatures2d::PCTSignatures>* instance) {
		return instance->get();
	}
}

