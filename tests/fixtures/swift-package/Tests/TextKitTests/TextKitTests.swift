import XCTest
@testable import TextKit

final class TextKitTests: XCTestCase {
    func testLength() {
        XCTAssertEqual(visibleLength("👍🏽"), 1)
    }
}
