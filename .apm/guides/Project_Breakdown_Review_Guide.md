# APM 0.5.3 - Hướng dẫn Rà soát Phân rã Dự án
Hướng dẫn này định nghĩa cách Setup Agent tiến hành rà soát có mục tiêu, do người dùng chọn của Kế hoạch Triển khai để phát hiện và sửa các vấn đề chất lượng nhiệm vụ quan trọng trước khi nâng cao. Sử dụng ngữ cảnh mới từ việc tạo Kế hoạch Triển khai, agent đề xuất các lĩnh vực cụ thể để rà soát có hệ thống và để người dùng chọn section nào nhận phân tích chi tiết.

---

## 1. Tổng quan Giao thức Rà soát

### Mục đích Rà soát
Tiến hành rà soát có hệ thống trên các phần Kế hoạch Triển khai do người dùng chọn để xác định và sửa các vấn đề chất lượng nhiệm vụ quan trọng:
- Vi phạm gói nhiệm vụ (nhiều hoạt động riêng biệt trong một nhiệm vụ)
- Lỗi phân loại (chỉ định sai single-step vs multi-step)
- Mẫu template matching (định dạng cứng nhắc qua các nhiệm vụ)
- Thất bại tuân thủ yêu cầu người dùng (thiếu yêu cầu Tổng hợp Ngữ cảnh)
- Lỗi phạm vi thực thi nhiệm vụ (giả định nền tảng bên ngoài)

### Phương pháp Rà soát Dựa trên Ngữ cảnh
**Đề xuất Agent → Lựa chọn Người dùng → Rà soát Có hệ thống Có mục tiêu → Sửa chữa Toàn diện**

**Quy trình Rà soát:**
1. **Đề xuất Thông minh**: Agent phân tích ngữ cảnh Kế hoạch Triển khai mới để đề xuất các lĩnh vực rà soát
2. **Lựa chọn Người dùng**: Người dùng chọn nhiệm vụ/giai đoạn nào nhận rà soát có hệ thống
3. **Phân tích Có hệ thống**: Áp dụng phương pháp kiểm tra đầy đủ chỉ cho các lĩnh vực được chọn
4. **Sửa chữa Toàn diện**: Sửa tất cả vấn đề trong các lĩnh vực được chọn, đảm bảo tuân thủ nghiêm ngặt định dạng đã thiết lập
5. **Rà soát Người dùng Cuối cùng**: Trình bày kế hoạch cập nhật hoàn chỉnh để phê duyệt

**Hiệu quả**: Sức mạnh rà soát có hệ thống đầy đủ chỉ được áp dụng nơi có giá trị nhất

---

## 2. Đề xuất Lĩnh vực Rà soát Thông minh

### 2.1. Phân tích Ngữ cảnh cho Tạo Đề xuất
**Tận dụng ngữ cảnh tạo Kế hoạch Triển khai mới để xác định mục tiêu rà soát có giá trị cao:**

**Nhận thức Ngữ cảnh Ngay lập tức:**
- **Nhiệm vụ Multi-Step Phức tạp**: Nhiệm vụ với 6+ bước có thể cần tách
- **Phạm vi Công nghệ**: Nhiệm vụ bao gồm nhiều domain hoặc lĩnh vực kỹ năng
- **Mục Đường dẫn Quan trọng**: Nhiệm vụ với nhiều dependency hoặc bàn giao xuyên-agent
- **Lĩnh vực Yêu cầu Người dùng**: Section chứa các yếu tố Tổng hợp Ngữ cảnh được nhấn mạnh
- **Điểm Tích hợp Bên ngoài**: Nhiệm vụ liên quan đến deploy, cấu hình, hoặc điều phối nền tảng

### 2.2. Danh mục Đề xuất
**Đề xuất lĩnh vực rà soát dựa trên mẫu được phát hiện:**

**Lĩnh vực Độ phức tạp Cao:**
- Giai đoạn với nhiều nhiệm vụ 6+ bước
- Nhiệm vụ trải rộng các domain công nghệ khác nhau
- Section với dependency xuyên-agent dày đặc

**Lĩnh vực Đường dẫn Quan trọng:**
- Nhiệm vụ chặn nhiều nhiệm vụ khác
- Điểm bàn giao xuyên-agent
- Nhiệm vụ tích hợp nền tảng bên ngoài

**Lĩnh vực Yêu cầu Người dùng:**
- Section triển khai yêu cầu Tổng hợp Ngữ cảnh được nhấn mạnh
- Nhiệm vụ liên quan đến sở thích hoặc ràng buộc cụ thể của người dùng

**Lĩnh vực Quan ngại Mẫu:**
- Nhóm nhiệm vụ với định dạng giống hệt nhau
- Section có thể có vấn đề template matching

### 2.3. Định dạng Trình bày Đề xuất
**Trình bày đề xuất rõ ràng, có thể hành động cho người dùng:**

**Cấu trúc Định dạng:**
```markdown
## Đề xuất Rà soát Có hệ thống

Dựa trên Kế hoạch Triển khai tôi vừa tạo, tôi đề xuất rà soát có hệ thống cho:

**Lĩnh vực Độ phức tạp Cao:**
- **[Phase/Task ID]** ([chỉ báo độ phức tạp: số lượng multi-step, phạm vi domain, v.v.])
- **[Phase/Task ID]** ([lý do độ phức tạp cụ thể])

**Lĩnh vực Đường dẫn Quan trọng:**
- **[Phase/Task ID]** ([mô tả dependency và tác động])
- **[Phase/Task ID]** ([yêu cầu điều phối bên ngoài])

**Tích hợp Yêu cầu Người dùng:**
- **[Phase/Task ID]** ([yêu cầu Tổng hợp Ngữ cảnh cụ thể được triển khai])

**Quan ngại Mẫu:**
- **[Task Range]** ([vấn đề template matching hoặc định dạng được xác định])

**Đề xuất:** Tập trung rà soát có hệ thống vào [lựa chọn có giá trị cao nhất] để có tác động tối đa.

**Lựa chọn của Bạn:** Chọn bất kỳ tổ hợp nào của các đề xuất trên, hoặc chỉ định nhiệm vụ/giai đoạn khác bạn muốn được rà soát. Tôi sẽ áp dụng phân tích có hệ thống đầy đủ chỉ cho các lĩnh vực bạn chọn.
```

**Hướng dẫn Đề xuất:**
- Giới hạn đề xuất tối đa 4-6 mục để ra quyết định rõ ràng
- Cung cấp lý do cụ thể cho mỗi đề xuất
- Làm nổi bật 1-2 ưu tiên hàng đầu để hướng dẫn người dùng
- Luôn cung cấp sự linh hoạt cho người dùng sửa đổi lựa chọn

---

## 3. Quy trình Lựa chọn Người dùng

### 3.1. Tùy chọn Lựa chọn
**Lựa chọn linh hoạt cho phép người dùng kiểm soát:**

**Định dạng Lựa chọn Người dùng Có thể Chọn:**
- **Lựa chọn Giai đoạn Đầy đủ**: "Rà soát [Phase X]" (tất cả nhiệm vụ trong giai đoạn được chỉ định)
- **Nhiều Giai đoạn**: "Rà soát [Phases X và Y]" (nhiều giai đoạn hoàn chỉnh)
- **Nhiệm vụ Riêng lẻ**: "Rà soát [Task X.Y] và [Task Z.A]" (lựa chọn nhiệm vụ cụ thể)
- **Phạm vi Nhiệm vụ**: "Rà soát [Tasks X.Y-X.Z]" (nhóm nhiệm vụ tuần tự)
- **Tổ hợp Hỗn hợp**: "Rà soát [Phase X] và [Task Y.Z]" (giai đoạn cộng nhiệm vụ riêng lẻ)
- **Cách tiếp cận Loại trừ**: "Rà soát mọi thứ ngoại trừ [định danh Phase/Task]" (toàn diện trừ loại trừ)

**Khả năng Lựa chọn Bổ sung:**
- Người dùng có thể thêm nhiệm vụ không có trong đề xuất agent
- Người dùng có thể yêu cầu tập trung vào khía cạnh cụ thể (phân loại, gói, tích hợp yêu cầu)
- Người dùng có thể sửa đổi đề xuất agent bằng cách thêm hoặc xóa mục

### 3.2. Xác nhận Lựa chọn
**Xác nhận rõ ràng phạm vi rà soát trước khi tiến hành:**

**Định dạng Xác nhận:**
```markdown
**Đã Chọn cho Rà soát Có hệ thống:**
- [Lựa chọn Phase/Task với số lượng nhiệm vụ]
- [Lựa chọn nhiệm vụ riêng lẻ]
- [Bất kỳ lĩnh vực tập trung đặc biệt nào được yêu cầu]

**Tổng cộng:** [X] nhiệm vụ nhận phân tích có hệ thống đầy đủ
**Tiến hành rà soát có hệ thống các lĩnh vực được chọn...**
```

**Yêu cầu Xác nhận:**
- Liệt kê tất cả giai đoạn và nhiệm vụ riêng lẻ được chọn
- Cung cấp tổng số nhiệm vụ để rõ ràng phạm vi
- Xác nhận bất kỳ lĩnh vực tập trung hoặc ràng buộc đặc biệt nào
- Nhận phê duyệt rõ ràng từ người dùng trước khi tiến hành

---

## 4. Phân tích Có hệ thống (Chỉ Lĩnh vực Được chọn)

### 4.1. Phương pháp Rà soát Quan trọng
**Thách thức các quyết định trước đó sử dụng đặt câu hỏi phân tích để xác định cải tiến thực sự:**

**QUAN TRỌNG**: Setup Agent vừa tạo các nhiệm vụ này sử dụng lý luận cụ thể. Rà soát có hệ thống phải thách thức lý luận đó một cách phân tích để tìm cơ hội cải tiến thực sự, không chỉ đơn giản xác nhận các quyết định trước.

### 4.2. Khung Kiểm tra Phân tích
**Cho mỗi nhiệm vụ được chọn, áp dụng đặt câu hỏi phân tích có cấu trúc:**

**Task [X.Y]: [Tên Nhiệm vụ] - Rà soát Có hệ thống**

**Phân tích Phạm vi:**
- **Quyết định Hiện tại**: "Cho nhiệm vụ này, tôi đã chọn [quyết định phạm vi]. Tại sao đây không phải là [cách tiếp cận phạm vi thay thế]?"
- **Đánh giá Độ phức tạp**: "Nhiệm vụ này có [X] bước/thành phần. Tôi có thể chia thành 2 hoặc nhiều nhiệm vụ tập trung không? Lợi ích/hạn chế sẽ là gì?"
- **Đánh giá Domain**: "Tôi đã giao này cho [Agent]. [Agent Thay thế] có phù hợp hơn không? Kiến thức domain cụ thể nào điều này yêu cầu?"

**Phân tích Phân loại:**
- **Định dạng Hiện tại**: "Tôi đã chọn định dạng [single-step/multi-step]. Yếu tố cụ thể nào hỗ trợ/thách thức phân loại này?"
- **Điểm Xác nhận**: "Nhiệm vụ này có cần điểm xác nhận người dùng không? Implementation Agent có thể bị kẹt ở đâu mà không có hướng dẫn?"
- **Hiệu quả Quy trình Làm việc**: "Điều này có hiệu quả hơn như [phân loại thay thế] không? Xác nhận nào thực sự cần thiết?"

**Khả thi Triển khai:**
- **Khả năng Agent**: "Giả định cụ thể nào tôi đang đưa ra về khả năng Implementation Agent? Giả định nào có thể không chính xác?"
- **Yêu cầu Ngữ cảnh**: "Nếu Implementation Agent nhận nhiệm vụ này với ngữ cảnh tối thiểu, họ sẽ cần làm rõ điều gì?"
- **Thách thức Thực thi**: "Điểm thất bại có khả năng nhất trong quá trình thực thi nhiệm vụ là gì? Đặc tả nhiệm vụ có thể giải quyết những điều này như thế nào?"
- **Trường Meta**: "Các trường 'Objective', 'Output', và 'Guidance' có cung cấp hướng dẫn rõ ràng, ngắn gọn cho Manager Agent không?"

**Tích hợp Yêu cầu:**
- **Căn chỉnh Tổng hợp Ngữ cảnh**: "Yêu cầu Tổng hợp Ngữ cảnh nào áp dụng cho nhiệm vụ này? Chúng được tích hợp rõ ràng hay giả định?"
- **Điều phối Người dùng**: "Hành động bên ngoài nào nhiệm vụ này yêu cầu? Các bước điều phối người dùng có được chỉ định rõ ràng không?"
- **Rõ ràng Đầu ra**: "Đầu ra nhiệm vụ có đủ cụ thể để Implementation Agent tiếp theo tích hợp không? Điều gì có thể mơ hồ?"

**Cách tiếp cận Thay thế:**
- **Tổ chức Khác**: "Công việc này có thể được cấu trúc như [cách tiếp cận thay thế] không? Lợi thế sẽ là gì?"
- **Tối ưu hóa Dependency**: "Các dependency cho nhiệm vụ này có tối ưu không? Tái tổ chức có thể giảm chi phí điều phối không?"

### 4.3. Thực thi Phân tích Có hệ thống
**Áp dụng khung phân tích cho mỗi nhiệm vụ được chọn:**

**Task [X.Y]: [Tên Nhiệm vụ] - Kết quả Phân tích**

1. **Kết quả Phân tích Phạm vi**:
   - Cân nhắc Phạm vi Thay thế: [Phân tích và quyết định]
   - Đánh giá Tách Nhiệm vụ: [Lợi ích/hạn chế được đánh giá, quyết định với lý do]
   - Xem xét Phân công Agent: [Phân tích phù hợp domain và xác nhận/thay đổi]

2. **Kết quả Phân tích Phân loại**:
   - Lý giải Định dạng: [Yếu tố hỗ trợ phân loại hiện tại hoặc cần thay đổi]
   - Đánh giá Điểm Xác nhận: [Nhu cầu xác nhận người dùng được phân tích]
   - Đánh giá Hiệu quả: [Cơ hội tối ưu hóa quy trình làm việc được xác định/xác nhận]

3. **Kết quả Khả thi Triển khai**:
   - Xem xét Giả định Khả năng: [Giả định được xác nhận hoặc sửa chữa được xác định]
   - Phân tích Yêu cầu Ngữ cảnh: [Làm rõ cần thiết hoặc đủ được xác nhận]
   - Giảm thiểu Điểm Thất bại: [Vấn đề tiềm năng được xác định và giải quyết]

4. **Kết quả Tích hợp Yêu cầu**:
   - Tích hợp Tổng hợp Ngữ cảnh: [Yêu cầu được thêm rõ ràng hoặc tích hợp được xác nhận]
   - Rõ ràng Điều phối Người dùng: [Bước hành động bên ngoài được làm rõ hoặc xác nhận]
   - Xem xét Đặc tả Đầu ra: [Mơ hồ được giải quyết hoặc rõ ràng được xác nhận]

5. **Kết quả Cách tiếp cận Thay thế**:
   - Thay thế Cấu trúc: [Cách tiếp cận thay thế được cân nhắc, hiện tại được lý giải hoặc thay đổi]
   - Tối ưu hóa Dependency: [Cải tiến điều phối được xác định hoặc hiện tại được xác nhận]

**Đánh giá Tổng thể**: [Cải tiến được triển khai / Cách tiếp cận hiện tại được xác nhận với lý do cụ thể]

### 4.4. Yêu cầu Nâng cao Chất lượng
**Đảm bảo thách thức mang tính xây dựng các quyết định trước:**

**Tiêu chuẩn Phân tích:**
- Mỗi nhiệm vụ được chọn phải được kiểm tra từ nhiều góc độ phân tích dựa trên §4.2 và §4.3
- Quyết định hiện tại phải được lý giải rõ ràng khi duy trì
- Cách tiếp cận thay thế phải được cân nhắc thực sự, không bị loại bỏ

**Phân tích Dựa trên Bằng chứng:**
- "Ban đầu tôi đã chọn cách tiếp cận X dựa trên lý do Y. Khi xem xét, cân nhắc Z gợi ý cải tiến A"
- "Trong khi cấu trúc hiện tại có vẻ vững chắc, phân tích khả thi triển khai tiết lộ cơ hội tối ưu hóa B"
- "Xem xét đặc tả nhiệm vụ xác nhận đủ nhưng xác định nâng cao C cho rõ ràng Implementation Agent"
- "Lựa chọn hiện tại đúng vì các yếu tố X, Y, và Z; phân tích thay thế cho thấy không có cách tiếp cận nào khác sẽ cung cấp lợi ích bổ sung trong ngữ cảnh này"

**Quy trình Thách thức Xây dựng:**
- Đặt câu hỏi mỗi quyết định quan trọng được đưa ra trong quá trình tạo nhiệm vụ ban đầu
- Xem xét góc nhìn Implementation Agent xuyên suốt phân tích
- Xác định cơ hội cải tiến cụ thể thay vì phê bình chung
- Duy trì tập trung vào thành công và rõ ràng thực thi nhiệm vụ

### 4.5. Tài liệu Vấn đề
**Theo dõi tất cả cải tiến được xác định trong các lĩnh vực được chọn:**

**Định dạng Tài liệu:**
```markdown
**Cải tiến Được xác định trong Lĩnh vực Được chọn:**
- [Task ID]: [Loại cải tiến] ([nâng cao được áp dụng])
- [Task ID]: [Tối ưu hóa được xác định] ([sửa đổi được thực hiện])
- [Task Range]: [Cải tiến mẫu] ([nâng cao có hệ thống được áp dụng])
```

**Yêu cầu Tài liệu:**
- Liệt kê mỗi nhiệm vụ với cải tiến được xác định trong rà soát có hệ thống
- Chỉ định loại cải tiến (tối ưu hóa phạm vi, tinh chỉnh phân loại, tích hợp yêu cầu, v.v.)
- Ghi lại nâng cao cụ thể được áp dụng
- Nhóm các cải tiến tương tự để rõ ràng
- Ghi chú nhiệm vụ mà cách tiếp cận hiện tại được xác nhận thông qua phân tích

---

## 5. Sửa chữa Toàn diện & Áp dụng Mẫu

### 5.1. Sửa chữa Lĩnh vực Được chọn
**Áp dụng tất cả sửa chữa được xác định cho nhiệm vụ được chọn:**

- Sửa vi phạm gói thông qua tách nhiệm vụ
- Sửa lỗi phân loại
- Thêm yêu cầu người dùng thiếu
- Giải quyết vấn đề template matching
- Làm rõ ranh giới phạm vi thực thi

### 5.2. Áp dụng Mẫu cho Lĩnh vực Chưa Rà soát
**Áp dụng mẫu đã học để cải thiện toàn bộ kế hoạch:**

**Nếu Mẫu Được tìm thấy trong Lĩnh vực Được chọn:**
- **Mẫu gói**: Quét lĩnh vực chưa rà soát cho chỉ báo gói tương tự
- **Mẫu phân loại**: Kiểm tra nhiệm vụ chưa rà soát với đặc điểm tương tự
- **Template matching**: Thay đổi định dạng qua các nhiệm vụ tương tự chưa rà soát
- **Yêu cầu thiếu**: Thêm yêu cầu cho nhiệm vụ chưa rà soát trong domain tương tự

**Áp dụng Bảo thủ:**
- Chỉ áp dụng mẫu rõ ràng, hiển nhiên cho lĩnh vực chưa rà soát
- Tránh thay đổi mở rộng cho section chưa rà soát
- Tập trung vào áp dụng bài học từ rà soát có hệ thống

### 5.3. Cập nhật Kế hoạch Toàn diện
**Cập nhật toàn bộ Kế hoạch Triển khai với tất cả thay đổi:**

1. **Áp dụng sửa chữa rà soát có hệ thống** cho lĩnh vực được chọn
2. **Áp dụng cải tiến dựa trên mẫu** cho lĩnh vực chưa rà soát
3. **Duy trì nhất quán** qua toàn bộ kế hoạch
4. **Cập nhật đánh số nhiệm vụ** và dependency khi cần

---

## 6. Rà soát Người dùng Cuối cùng

### 6.1. Trình bày Tóm tắt Rà soát
**Tóm tắt rõ ràng tất cả thay đổi được thực hiện:**

**Định dạng Tóm tắt:**
```markdown
## Rà soát Hoàn thành - Tóm tắt Thay đổi

**Rà soát Có hệ thống Được Áp dụng Cho:**
- [Lựa chọn Phase/Task] - Tìm thấy và sửa: [tóm tắt vấn đề với số lượng]
- [Nhiệm vụ riêng lẻ] - Tìm thấy và sửa: [vấn đề cụ thể]
- [Bất kỳ lĩnh vực nào không tìm thấy vấn đề]

**Cải tiến Dựa trên Mẫu Được Áp dụng:**
- [Mô tả mẫu được tìm thấy và áp dụng cho lĩnh vực chưa rà soát]
- [Số lượng và loại cải tiến được thực hiện dựa trên phát hiện rà soát có hệ thống]

**Tổng Thay đổi:**
- [X] nhiệm vụ được tách ([gốc] → [phân rã nhiệm vụ mới])
- [X] nhiệm vụ được phân loại lại ([thay đổi phân loại được thực hiện])
- [X] nhiệm vụ được nâng cao với [loại nâng cao]
- [X] nhiệm vụ được định dạng lại cho [cải tiến định dạng]

**Sẵn sàng cho Giai đoạn Nâng cao**
```

**Yêu cầu Tóm tắt:**
- Phân biệt rõ ràng giữa sửa chữa rà soát có hệ thống và cải tiến dựa trên mẫu
- Cung cấp số lượng và loại thay đổi cụ thể được thực hiện
- Liệt kê bất kỳ tách nhiệm vụ nào với định danh trước/sau
- Xác nhận sẵn sàng cho giai đoạn tiếp theo

### 6.2. Quy trình Phê duyệt Cuối cùng
**Người dùng rà soát và phê duyệt:**

1. **Trình bày Kế hoạch Triển khai cập nhật** với tất cả thay đổi
2. **Làm nổi bật sửa đổi lớn** để người dùng chú ý
3. **Yêu cầu phê duyệt rõ ràng** để tiến sang Manager Bootstrap Prompt Creation
4. **Giải quyết bất kỳ quan ngại nào của người dùng** hoặc thay đổi bổ sung
5. **Xác nhận hoàn thành** khi người dùng phê duyệt

---

## 7. Hoàn thiện
**Chuẩn bị cho Bootstrap Prompt Creation:**
- Đảm bảo `Implementation_Plan.md` ở trạng thái cuối cùng, sạch.
- Xác nhận tất cả header nhiệm vụ, phân công agent, và tag dependency được định dạng đúng.

**Bootstrap Prompt Generation:**
- Chuyển điều khiển về logic .claude/commands/apm-1-initiate-setup.md.
- **Khôi phục Ngữ cảnh:** Khi tạo Bootstrap Prompt, bạn phải sử dụng **TEMPLATE CHÍNH XÁC** từ .claude/commands/apm-1-initiate-setup.md. Nếu template bị suy giảm hoặc thiếu từ cửa sổ ngữ cảnh của bạn, **ĐỌC .claude/commands/apm-1-initiate-setup.md** để truy xuất trước khi tạo artifact cuối cùng.

---

## Quy tắc Ngôn ngữ – BẮT BUỘC

### Nguyên tắc cốt lõi
- **BẮT BUỘC**: Phản hồi bằng **tiếng Việt**.
- **GIẢI THÍCH THUẬT NGỮ**: Mọi thuật ngữ tiếng Anh phải kèm mô tả tiếng Việt.

### Cú pháp chuẩn
`**<English Term>** (mô tả tiếng Việt – chức năng/mục đích)`

### Code Comments / Documentation / Logs
- **Mặc định**: Comments, logs, docs, docstrings viết bằng tiếng Việt.
- **Song ngữ tại điểm quan trọng**: Dòng đầu tiếng Việt, dòng sau tiếng Anh.
- **Structured logging**: Keys bằng tiếng Anh (machine parsing), messages bằng tiếng Việt.
- **Ngoại lệ**: Khi library/standard yêu cầu tiếng Anh, thêm ghi chú tiếng Việt.

### Ví dụ
- **Systematic Review** (Rà soát Hệ thống – kiểm tra kế hoạch chi tiết)
- **Task Packing Violation** (Vi phạm Gói Task – nhiều hoạt động trong một task)
- **Classification Error** (Lỗi Phân loại – sai single-step/multi-step)
- **Pattern Application** (Áp dụng Mẫu – lan truyền cải tiến ra toàn plan)
- **User Selection** (Lựa chọn User – người dùng chọn vùng review)

---

**Kết thúc Hướng dẫn**