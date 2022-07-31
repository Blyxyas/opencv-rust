extern "C" {
	cv::Ptr<cv::detail::BundleAdjusterRay>* cv_PtrOfDetail_BundleAdjusterRay_new(cv::detail::BundleAdjusterRay* val) {
		return new cv::Ptr<cv::detail::BundleAdjusterRay>(val);
	}
	
	void cv_PtrOfDetail_BundleAdjusterRay_delete(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		delete instance;
	}

	const cv::detail::BundleAdjusterRay* cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr(const cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return instance->get();
	}

	cv::detail::BundleAdjusterRay* cv_PtrOfDetail_BundleAdjusterRay_get_inner_ptr_mut(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::BundleAdjusterBase>* cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_BundleAdjusterBase(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return new cv::Ptr<cv::detail::BundleAdjusterBase>(instance->dynamicCast<cv::detail::BundleAdjusterBase>());
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_BundleAdjusterRay_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::BundleAdjusterRay>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

