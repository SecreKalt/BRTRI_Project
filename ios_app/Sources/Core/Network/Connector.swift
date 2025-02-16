import Foundation

class Connector {
    private var urlSession: URLSession
    private var routerURL: URL
    private var retryCount: Int = 0
    private let maxRetries: Int = 3

    init(routerURL: URL) {
        self.routerURL = routerURL
        self.urlSession = URLSession(configuration: .default)
    }

    func sendScannedData(_ data: Data) {
        var request = URLRequest(url: routerURL)
        request.httpMethod = "POST"
        request.httpBody = data
        request.setValue("application/octet-stream", forHTTPHeaderField: "Content-Type")

        let task = urlSession.dataTask(with: request) { [weak self] data, response, error in
            if let error = error {
                self?.handleError(error)
                return
            }

            guard let httpResponse = response as? HTTPURLResponse, httpResponse.statusCode == 200 else {
                self?.handleError(NSError(domain: "Connector", code: 1, userInfo: [NSLocalizedDescriptionKey: "Invalid response"]))
                return
            }

            // Handle successful response if needed
        }

        task.resume()
    }

    private func handleError(_ error: Error) {
        print("Error sending data: \(error.localizedDescription)")
        retryCount += 1
        if retryCount <= maxRetries {
            print("Retrying... (\(retryCount)/\(maxRetries))")
            // Retry logic here
        } else {
            print("Max retries reached. Failed to send data.")
        }
    }
}
