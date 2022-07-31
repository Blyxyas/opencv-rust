extern "C" {
	cv::Ptr<cv::MercatorWarper>* cv_PtrOfMercatorWarper_new(cv::MercatorWarper* val) {
		return new cv::Ptr<cv::MercatorWarper>(val);
	}
	
	void cv_PtrOfMercatorWarper_delete(cv::Ptr<cv::MercatorWarper>* instance) {
		delete instance;
	}

	const cv::MercatorWarper* cv_PtrOfMercatorWarper_get_inner_ptr(const cv::Ptr<cv::MercatorWarper>* instance) {
		return instance->get();
	}

	cv::MercatorWarper* cv_PtrOfMercatorWarper_get_inner_ptr_mut(cv::Ptr<cv::MercatorWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfMercatorWarper_to_PtrOfWarperCreator(cv::Ptr<cv::MercatorWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

