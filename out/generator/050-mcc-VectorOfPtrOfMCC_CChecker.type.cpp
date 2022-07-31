extern "C" {
	void cv_VectorOfPtrOfMCC_CChecker_delete(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		delete instance;
	}

	std::vector<cv::Ptr<cv::mcc::CChecker>>* cv_VectorOfPtrOfMCC_CChecker_new() {
		return new std::vector<cv::Ptr<cv::mcc::CChecker>>();
	}

	size_t cv_VectorOfPtrOfMCC_CChecker_len(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->size();
	}

	bool cv_VectorOfPtrOfMCC_CChecker_is_empty(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfPtrOfMCC_CChecker_capacity(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfPtrOfMCC_CChecker_resize(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfPtrOfMCC_CChecker_shrink_to_fit(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfPtrOfMCC_CChecker_reserve(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfPtrOfMCC_CChecker_remove(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfPtrOfMCC_CChecker_swap(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfPtrOfMCC_CChecker_clear(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance) {
		instance->clear();
	}

	void cv_VectorOfPtrOfMCC_CChecker_push(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, cv::Ptr<cv::mcc::CChecker>* val) {
		instance->push_back(*val);
	}

	void cv_VectorOfPtrOfMCC_CChecker_insert(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>* val) {
		instance->insert(instance->begin() + index, *val);
	}

	void cv_VectorOfPtrOfMCC_CChecker_get(const std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>** ocvrs_return) {
		*ocvrs_return = new cv::Ptr<cv::mcc::CChecker>((*instance)[index]);
	}

	void cv_VectorOfPtrOfMCC_CChecker_set(std::vector<cv::Ptr<cv::mcc::CChecker>>* instance, size_t index, cv::Ptr<cv::mcc::CChecker>* val) {
		(*instance)[index] = *val;
	}

}


