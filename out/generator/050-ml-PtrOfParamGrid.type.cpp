extern "C" {
	cv::Ptr<cv::ml::ParamGrid>* cv_PtrOfParamGrid_new(cv::ml::ParamGrid* val) {
		return new cv::Ptr<cv::ml::ParamGrid>(val);
	}
	
	void cv_PtrOfParamGrid_delete(cv::Ptr<cv::ml::ParamGrid>* instance) {
		delete instance;
	}

	const cv::ml::ParamGrid* cv_PtrOfParamGrid_get_inner_ptr(const cv::Ptr<cv::ml::ParamGrid>* instance) {
		return instance->get();
	}

	cv::ml::ParamGrid* cv_PtrOfParamGrid_get_inner_ptr_mut(cv::Ptr<cv::ml::ParamGrid>* instance) {
		return instance->get();
	}
}

