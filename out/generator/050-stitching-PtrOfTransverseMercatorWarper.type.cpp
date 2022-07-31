extern "C" {
	cv::Ptr<cv::TransverseMercatorWarper>* cv_PtrOfTransverseMercatorWarper_new(cv::TransverseMercatorWarper* val) {
		return new cv::Ptr<cv::TransverseMercatorWarper>(val);
	}
	
	void cv_PtrOfTransverseMercatorWarper_delete(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		delete instance;
	}

	const cv::TransverseMercatorWarper* cv_PtrOfTransverseMercatorWarper_get_inner_ptr(const cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return instance->get();
	}

	cv::TransverseMercatorWarper* cv_PtrOfTransverseMercatorWarper_get_inner_ptr_mut(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfTransverseMercatorWarper_to_PtrOfWarperCreator(cv::Ptr<cv::TransverseMercatorWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

