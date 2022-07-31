extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterReproj>* cv_PtrOfDetail_BundleAdjusterReproj_new(cv::detail::BundleAdjusterReproj* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterReproj>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterReproj_delete(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterReproj* cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterReproj* cv_PtrOfDetail_BundleAdjusterReproj_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_BundleAdjusterReproj_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterReproj>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

