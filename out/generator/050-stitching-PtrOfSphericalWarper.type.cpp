extern "C" {
	cv::Ptr<cv::SphericalWarper>* cv_PtrOfSphericalWarper_new(cv::SphericalWarper* val) {
		return new cv::Ptr<cv::SphericalWarper>(val);
	}
	
	void cv_PtrOfSphericalWarper_delete(cv::Ptr<cv::SphericalWarper>* instance) {
		delete instance;
	}

	const cv::SphericalWarper* cv_PtrOfSphericalWarper_get_inner_ptr(const cv::Ptr<cv::SphericalWarper>* instance) {
		return instance->get();
	}

	cv::SphericalWarper* cv_PtrOfSphericalWarper_get_inner_ptr_mut(cv::Ptr<cv::SphericalWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfSphericalWarper_to_PtrOfWarperCreator(cv::Ptr<cv::SphericalWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

