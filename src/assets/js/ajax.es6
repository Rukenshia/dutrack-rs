class AjaxError extends Error {
	constructor(status, statusText) {
		let message = `${status}: ${statusText}`
		super(message);
		this.name = this.constructor.name;
		this.message = message;
		this.status = status;
		this.statusText = statusText;
		Error.captureStackTrace(this, this.constructor.name)
	}
}

function applyOptions(xhr, options) {
	options.headers = options.headers || [];
	for (let header in options.headers) {
		xhr.setRequestHeader(header, options.headers[header]);
	}
}

function createHandler(xhr, success, fail) {
	return () => {
		if (xhr.readyState !== XMLHttpRequest.DONE) {
			return;
		}
		if (xhr.status === 200) {
			success(xhr.response);
		} else {
			fail(xhr.status, xhr.statusText);
		}
	};
}

function request(method, url, body, options) {
	options = options || {};
	return new Promise((resolve, reject) => {
		let xhr = new XMLHttpRequest();
		xhr.onreadystatechange = createHandler(
			xhr,
			(res) => {
				resolve(res);
			},
			(status, statusText) => {
				reject(new AjaxError(status, statusText));
			});
		xhr.open(method, url, true);
		applyOptions(xhr, options);
		xhr.onerror = function() {
			reject(new AjaxError(-1, 'Network error'));
		};
		xhr.send(body);
	});
}

window.http = {
	async get(url, options) {
		return request('GET', url, null, options);
	},
	async post(url, body, options) {
		return request('POST', url, body, options);
	},
	async del(url, body, options) {
		return request('DELETE', url, body, options);
	},
	async put(url, body, options) {
		return request('PUT', url, body, options);
	},
};
