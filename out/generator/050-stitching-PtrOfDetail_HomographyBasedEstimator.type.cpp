extern "C" {
	cv::Ptr<cv::detail::HomographyBasedEstimator>* cv_PtrOfDetail_HomographyBasedEstimator_new(cv::detail::HomographyBasedEstimator* val) {
		return new cv::Ptr<cv::detail::HomographyBasedEstimator>(val);
	}
	
	void cv_PtrOfDetail_HomographyBasedEstimator_delete(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
		delete instance;
	}

	const cv::detail::HomographyBasedEstimator* cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr(const cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
		return instance->get();
	}

	cv::detail::HomographyBasedEstimator* cv_PtrOfDetail_HomographyBasedEstimator_get_inner_ptr_mut(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::detail::Estimator>* cv_PtrOfDetail_HomographyBasedEstimator_to_PtrOfDetail_Estimator(cv::Ptr<cv::detail::HomographyBasedEstimator>* instance) {
		return new cv::Ptr<cv::detail::Estimator>(instance->dynamicCast<cv::detail::Estimator>());
	}
}

