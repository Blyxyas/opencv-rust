extern "C" {
	cv::Ptr<cv::AffineWarper>* cv_PtrOfAffineWarper_new(cv::AffineWarper* val) {
		return new cv::Ptr<cv::AffineWarper>(val);
	}
	
	void cv_PtrOfAffineWarper_delete(cv::Ptr<cv::AffineWarper>* instance) {
		delete instance;
	}

	const cv::AffineWarper* cv_PtrOfAffineWarper_get_inner_ptr(const cv::Ptr<cv::AffineWarper>* instance) {
		return instance->get();
	}

	cv::AffineWarper* cv_PtrOfAffineWarper_get_inner_ptr_mut(cv::Ptr<cv::AffineWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfAffineWarper_to_PtrOfWarperCreator(cv::Ptr<cv::AffineWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

