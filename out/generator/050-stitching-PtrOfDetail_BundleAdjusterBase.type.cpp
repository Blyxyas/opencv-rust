extern "C" {
	void cv_PtrOfDetail_BundleAdjusterBase_delete(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterBase* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterBase* cv_PtrOfDetail_BundleAdjusterBase_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_BundleAdjusterBase_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterBase>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

