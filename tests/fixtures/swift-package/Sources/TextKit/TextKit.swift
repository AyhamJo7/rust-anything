/// Why a score did not parse.
public enum ScoreError: Error {
    case empty
    case notANumber
    case outOfRange(Int)
}

/// Adds two counters. Swift stops the program when the sum overflows `Int`.
public func addCounts(_ a: Int, _ b: Int) -> Int {
    return a + b
}

/// The characters a reader sees: `count` counts grapheme clusters, so "👍🏽" is 1.
public func visibleLength(_ text: String) -> Int {
    return text.count
}

/// A score between 0 and `limit`.
public func parseScore(_ text: String, max limit: Int = 100) throws -> Int {
    if text.isEmpty {
        throw ScoreError.empty
    }
    guard let n = Int(text) else {
        throw ScoreError.notANumber
    }
    if n < 0 || n > limit {
        throw ScoreError.outOfRange(n)
    }
    return n
}

/// The mean of `values`, or nil when there are none.
public func average(of values: [Double]) -> Double? {
    if values.isEmpty {
        return nil
    }
    var sum = 0.0
    for v in values {
        sum += v
    }
    return sum / Double(values.count)
}

/// How often each lower-cased word occurs.
public func wordCounts(_ text: String, ignoring stop: [String]) -> [String: Int] {
    var counts: [String: Int] = [:]
    for word in text.split(separator: " ") {
        let key = word.lowercased()
        if !stop.contains(key) {
            counts[key, default: 0] += 1
        }
    }
    return counts
}

/// Takes a closure, so it stays in Swift.
public func applyTwice(_ f: (Int) -> Int, to x: Int) -> Int {
    return f(f(x))
}

private func unused() -> Int {
    return 0
}
