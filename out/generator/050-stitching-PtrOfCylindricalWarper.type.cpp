extern "C" {
	cv::Ptr<cv::CylindricalWarper>* cv_PtrOfCylindricalWarper_new(cv::CylindricalWarper* val) {
		return new cv::Ptr<cv::CylindricalWarper>(val);
	}
	
	void cv_PtrOfCylindricalWarper_delete(cv::Ptr<cv::CylindricalWarper>* instance) {
		delete instance;
	}

	const cv::CylindricalWarper* cv_PtrOfCylindricalWarper_get_inner_ptr(const cv::Ptr<cv::CylindricalWarper>* instance) {
		return instance->get();
	}

	cv::CylindricalWarper* cv_PtrOfCylindricalWarper_get_inner_ptr_mut(cv::Ptr<cv::CylindricalWarper>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::WarperCreator>* cv_PtrOfCylindricalWarper_to_PtrOfWarperCreator(cv::Ptr<cv::CylindricalWarper>* instance) {
		return new cv::Ptr<cv::WarperCreator>(instance->dynamicCast<cv::WarperCreator>());
	}
}

