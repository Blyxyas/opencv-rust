extern "C" {
	void cv_VectorOfBarcodeType_delete(std::vector<cv::barcode::BarcodeType>* instance) {
		delete instance;
	}

	std::vector<cv::barcode::BarcodeType>* cv_VectorOfBarcodeType_new() {
		return new std::vector<cv::barcode::BarcodeType>();
	}

	size_t cv_VectorOfBarcodeType_len(const std::vector<cv::barcode::BarcodeType>* instance) {
		return instance->size();
	}

	bool cv_VectorOfBarcodeType_is_empty(const std::vector<cv::barcode::BarcodeType>* instance) {
		return instance->empty();
	}

	size_t cv_VectorOfBarcodeType_capacity(const std::vector<cv::barcode::BarcodeType>* instance) {
		return instance->capacity();
	}

	void cv_VectorOfBarcodeType_resize(std::vector<cv::barcode::BarcodeType>* instance, size_t new_size) {
		instance->resize(new_size);
	}

	void cv_VectorOfBarcodeType_shrink_to_fit(std::vector<cv::barcode::BarcodeType>* instance) {
		instance->shrink_to_fit();
	}

	void cv_VectorOfBarcodeType_reserve(std::vector<cv::barcode::BarcodeType>* instance, size_t additional) {
		instance->reserve(instance->size() + additional);
	}

	void cv_VectorOfBarcodeType_remove(std::vector<cv::barcode::BarcodeType>* instance, size_t index) {
		instance->erase(instance->begin() + index);
	}

	void cv_VectorOfBarcodeType_swap(std::vector<cv::barcode::BarcodeType>* instance, size_t index1, size_t index2) {
		std::swap((*instance)[index1], (*instance)[index2]);
	}

	void cv_VectorOfBarcodeType_clear(std::vector<cv::barcode::BarcodeType>* instance) {
		instance->clear();
	}

	void cv_VectorOfBarcodeType_push(std::vector<cv::barcode::BarcodeType>* instance, cv::barcode::BarcodeType val) {
		instance->push_back(val);
	}

	void cv_VectorOfBarcodeType_insert(std::vector<cv::barcode::BarcodeType>* instance, size_t index, cv::barcode::BarcodeType val) {
		instance->insert(instance->begin() + index, val);
	}

	void cv_VectorOfBarcodeType_get(const std::vector<cv::barcode::BarcodeType>* instance, size_t index, cv::barcode::BarcodeType* ocvrs_return) {
		*ocvrs_return = (*instance)[index];
	}

	void cv_VectorOfBarcodeType_set(std::vector<cv::barcode::BarcodeType>* instance, size_t index, cv::barcode::BarcodeType val) {
		(*instance)[index] = val;
	}

	const cv::barcode::BarcodeType* cv_VectorOfBarcodeType_data(const std::vector<cv::barcode::BarcodeType>* instance) {
		return instance->data();
	}
	
	cv::barcode::BarcodeType* cv_VectorOfBarcodeType_data_mut(std::vector<cv::barcode::BarcodeType>* instance) {
		return instance->data();
	}
	
	std::vector<cv::barcode::BarcodeType>* cv_VectorOfBarcodeType_clone(const std::vector<cv::barcode::BarcodeType>* instance) {
		return new std::vector<cv::barcode::BarcodeType>(*instance);
	}
	
	std::vector<cv::barcode::BarcodeType>* cv_VectorOfBarcodeType_from_slice(const cv::barcode::BarcodeType* data, size_t len) {
		return new std::vector<cv::barcode::BarcodeType>(data, data + len);
	}
}


