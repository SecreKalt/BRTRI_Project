import Foundation
import Network

class Stream {
    private var connection: NWConnection?
    private var isStreaming: Bool = false
    private var buffer: Data = Data()
    private let queue = DispatchQueue(label: "StreamQueue")

    func startStreaming(to url: URL) {
        guard !isStreaming else { return }
        isStreaming = true
        connection = NWConnection(host: NWEndpoint.Host(url.host ?? ""), port: NWEndpoint.Port("\(url.port ?? 80)")!, using: .tcp)
        connection?.stateUpdateHandler = { [weak self] state in
            switch state {
            case .ready:
                print("Connection ready")
                self?.sendBufferedData()
            case .failed(let error):
                print("Connection failed: \(error)")
                self?.stopStreaming()
            default:
                break
            }
        }
        connection?.start(queue: queue)
    }

    func stopStreaming() {
        guard isStreaming else { return }
        isStreaming = false
        connection?.cancel()
        connection = nil
    }

    func streamData(_ data: Data) {
        guard isStreaming else { return }
        buffer.append(data)
        sendBufferedData()
    }

    private func sendBufferedData() {
        guard isStreaming, let connection = connection else { return }
        guard !buffer.isEmpty else { return }

        connection.send(content: buffer, completion: .contentProcessed({ [weak self] error in
            if let error = error {
                print("Error sending data: \(error)")
                self?.stopStreaming()
                return
            }
            self?.buffer.removeAll()
        }))
    }
}
