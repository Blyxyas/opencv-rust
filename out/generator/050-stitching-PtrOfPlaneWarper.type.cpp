extern "C" {
	cv::Ptr<cv::PlaneWarper>* cv_PtrOfPlaneWarper_new(cv::PlaneWarper* val) {
		return new cv::Ptr<cv::PlaneWarper>(val);
	}
	
	void cv_PtrOfPlaneWarper_delete(cv::Ptr<cv::PlaneWarper>* instance) {
		delete instance;
	}

	const cv::PlaneWarper* cv_PtrOfPlaneWarper_get_inner_ptr(const cv::Ptr<cv::PlaneWarper>* instance) {
		return instance->get();
	}

	cv::PlaneWarper* cv_PtrOfPlaneWarper_get_inner_ptr_mut(cv::Ptr<cv::PlaneWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfPlaneWarper_to_PtrOfWarperCreator(cv::Ptr<cv::PlaneWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

