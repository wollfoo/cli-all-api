# APM 0.5.3 - Hướng dẫn Memory Log
Hướng dẫn này định nghĩa cách Implementation Agent ghi lại công việc của họ cho Manager Agent và Người dùng. **Memory Log** (Nhật ký Bộ nhớ – ghi lại kết quả thực thi task) ghi lại ngữ cảnh cấp nhiệm vụ sử dụng định dạng **Dynamic-MD** (Định dạng Markdown Động).

Cả Manager và Implementation Agent phải đọc hướng dẫn này trong quá trình khởi tạo phiên. Implementation Agent tham chiếu nó khi ghi log; Manager Agent sử dụng nó khi xem xét log.

## 1. Tổng quan Hệ thống Memory
Tóm tắt biến thể Hệ thống Memory Dynamic-MD, bố cục lưu trữ và định dạng log:

- Lưu trữ: Thư mục `.apm/Memory/` với các thư mục con `Phase_XX_<slug>/`
- Định dạng: Một file `Task_XX_<slug>.md` cho mỗi nhiệm vụ (Markdown)

Memory Log được điền bởi Implementation Agent sau mỗi lần thực thi nhiệm vụ hoặc khi xảy ra blocker. Manager Agent xem xét log để theo dõi tiến độ và lập kế hoạch các bước tiếp theo.

## 2. Định dạng Memory Log Dynamic-MD
Tất cả các mục Memory Log phải tuân theo cấu trúc chính xác để đảm bảo rõ ràng, có thể truy vết, và giữ lại ngữ cảnh. Cho Hệ thống Memory Dynamic-MD, mỗi log nằm trong một file chuyên dụng được tạo trống bởi Manager Agent, sau đó được điền bởi Implementation Agent. Sử dụng Markdown có thể phân tích với **YAML front-matter** (phần đầu YAML – metadata cho log) và định dạng tối thiểu. Chỉ bao gồm các section tùy chọn khi boolean front-matter của chúng là true:

### 2.1 Cờ Frontmatter
- `important_findings: [true|false]` -> Đặt thành **true** nếu bạn phát hiện ràng buộc kiến trúc, yêu cầu mới, hoặc ngữ cảnh quan trọng cần Manager xem xét thêm.
- `compatibility_issues: [true|false]` -> Đặt thành **true** nếu đầu ra nhiệm vụ xung đột với hệ thống hiện có hoặc yêu cầu cập nhật kế hoạch.


### Template Memory Log
```yaml
---
agent: [Agent ID]
task_ref: [Task_ID]
status: [Completed|Partial|Blocked|Error]
ad_hoc_delegation: [true|false]
compatibility_issues: [true|false]
important_findings: [true|false]
---
```
```markdown

# Task Log: [Task Reference]

## Tóm tắt
[1-2 câu mô tả kết quả chính]

## Chi tiết
[Công việc đã thực hiện, quyết định đã đưa ra, các bước đã thực hiện theo thứ tự logic]

## Đầu ra
- Đường dẫn file cho các file đã tạo/sửa đổi
- Đoạn code (nếu cần thiết, ≤ 20 dòng)
- Thay đổi cấu hình
- Kết quả hoặc sản phẩm

## Vấn đề
[Blocker hoặc lỗi cụ thể, bao gồm thông báo lỗi nếu liên quan, hoặc "Không có"]

## Mối quan ngại Tương thích
[Chỉ bao gồm section này nếu compatibility_issues: true]
[Bất kỳ vấn đề tương thích nào đã xác định]

## Ủy thác Ad-Hoc Agent
[Chỉ bao gồm section này nếu ad_hoc_delegation: true]
[Chi tiết về bất kỳ ủy thác agent nào xảy ra trong nhiệm vụ này]

## Phát hiện Quan trọng
[Chỉ bao gồm section này nếu important_findings: true]
[Thông tin liên quan đến dự án được phát hiện trong quá trình làm việc mà Manager phải biết]

## Bước Tiếp theo
[Hành động theo dõi hoặc hướng dẫn cho agent tiếp theo hoặc "Không có"]
```

## 3. Quy trình Implementation Agent
Trách nhiệm chính và các bước quy trình cho Implementation Agent khi làm việc với Hệ thống Memory:

1. **Nhận Phân công Nhiệm vụ:** Manager Agent cung cấp prompt nhiệm vụ qua Người dùng với `memory_log_path` được chỉ định trong YAML frontmatter trỏ đến file log trống.
2. **Thực thi Nhiệm vụ:** Làm việc trên nhiệm vụ được giao như mô tả trong Task Assignment Prompt. Hoàn thành nhiệm vụ hoặc ghi chú bất kỳ vấn đề, blocker, hoặc lỗi nào ngăn cản hoàn thành.
3. **Cập nhật Log:** Điền tất cả các trường bắt buộc trong file log được cung cấp sử dụng định dạng đúng được định nghĩa trong section 2.
4. **Báo cáo Kết quả:** Thông báo cho Người dùng về hoàn thành nhiệm vụ hoặc vấn đề, xác nhận Memory Log đã được cập nhật.

## 4. Quy trình Manager Agent
Trách nhiệm chính và các bước quy trình cho Manager Agent khi duy trì Hệ thống Memory:

1. **Tạo Log Trống:** Vào đầu mỗi giai đoạn, tạo các file log **hoàn toàn trống** (hoặc section inline) cho tất cả nhiệm vụ giai đoạn. **KHÔNG điền bất kỳ nội dung nào.** Implementation Agent sẽ điền toàn bộ cấu trúc khi thực thi nhiệm vụ.
2. **Đính kèm vào Phân công:** Bao gồm đường dẫn file log trống phù hợp (hoặc section inline) với mỗi prompt phân công nhiệm vụ gửi đến Implementation Agent.
3. **Xem xét Log Hoàn thành:** Khi Người dùng trả về với nhiệm vụ hoàn thành, xem xét nội dung log về:
   - Nếu `important_findings` hoặc `compatibility_issues` là **true**, bạn được yêu cầu đọc các file và artifact được tham chiếu để xác nhận ngữ cảnh trước khi ra quyết định.
   - Trạng thái hoàn thành và chất lượng nhiệm vụ
   - Bất kỳ blocker hoặc vấn đề nào cần chú ý
   - Đầu ra thông tin cho các phân công nhiệm vụ tiếp theo
4. **Quyết định Hành động Tiếp theo:** Dựa trên xem xét log, xác định xem có nên:
   - Gửi prompt theo dõi cho cùng agent (nếu nhiệm vụ bị chặn hoặc cần tinh chỉnh)
   - Phân công ad-hoc agent cho công việc chuyên biệt hoặc giải quyết vấn đề
   - Tiếp tục với phân công nhiệm vụ theo kế hoạch tiếp theo (nếu mọi thứ tốt)

## 5. Hướng dẫn Nội dung

### 5.1. Viết Ngắn gọn và Hiệu quả
- Tóm tắt kết quả thay vì liệt kê mọi bước
- Tập trung vào quyết định và lý do chính, đặc biệt nếu kế hoạch thay đổi
- Tham chiếu artifact theo đường dẫn, tránh các khối code lớn
- Chỉ bao gồm đoạn code cho logic mới, phức tạp, hoặc quan trọng (≤ 20 dòng)
- Liên kết hành động với yêu cầu từ mô tả nhiệm vụ khi liên quan
- Bao gồm giải thích có giá trị được cung cấp trong quá trình thực thi nhiệm vụ khi chúng cung cấp insight cho người dùng

### 5.2. Xử lý Code và Đầu ra
- Cho thay đổi code: hiển thị đoạn code liên quan với đường dẫn file, không phải toàn bộ file
- Cho đầu ra lớn: lưu vào file riêng và tham chiếu đường dẫn
- Cho thông báo lỗi: bao gồm **stack trace** (dấu vết ngăn xếp) hoặc chi tiết lỗi liên quan
- Cho cấu hình: ghi chú các cài đặt chính đã thay đổi và tại sao

### 5.3. Vấn đề và Blocker
Khi ghi log blocker hoặc lỗi:
- Cụ thể về những gì ngăn cản hoàn thành
- Cung cấp thông tin có thể hành động cho Manager Agent
- Bao gồm thông báo lỗi hoặc thông tin chẩn đoán
- Đề xuất giải pháp tiềm năng nếu có thể

### 5.4. So sánh Chất lượng Ví dụ

- Log kém: "Tôi đã làm việc trên API endpoint. Tôi đã thực hiện một số thay đổi cho file. Có một số vấn đề nhưng tôi đã sửa chúng. Endpoint hoạt động bây giờ."

- Log tốt:
```markdown
---
agent: Agent_Backend
task_ref: Task 2.3
status: Completed
ad_hoc_delegation: false
compatibility_issues: false
important_findings: false
---

# Task Log: Task 2.3 - API User Endpoint

## Tóm tắt
Đã triển khai POST /api/users endpoint với xác thực đầu vào và sửa vấn đề CORS chặn request frontend.

## Chi tiết
- Thêm route đăng ký user trong routes/users.js sử dụng express-validator cho kiểm tra email và password
- Cập nhật cài đặt CORS trong server.js để cho phép tích hợp frontend
- Kiểm tra endpoint với dữ liệu hợp lệ/không hợp lệ để xác nhận xác thực và sửa lỗi CORS

## Đầu ra
- File đã sửa đổi: routes/users.js, server.js
- Chức năng endpoint: Chấp nhận {email, password, name}; xác thực đầu vào; trả về 201 khi thành công, 400 khi lỗi
- Logic xác thực chính được thêm cho định dạng email, độ dài password, và trường name bắt buộc

## Vấn đề
Không có

## Bước Tiếp theo
- Thêm unit/integration test cho xác thực và CORS
- Cập nhật tài liệu API cho endpoint mới
```

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
- **Memory Log** (Nhật ký Bộ nhớ – ghi lại kết quả thực thi task)
- **Dynamic-MD** (Định dạng Markdown Động – cấu trúc file log)
- **Frontmatter Flags** (Cờ YAML Đầu tệp – metadata cho log)
- **Task Completion Status** (Trạng thái Hoàn thành – kết quả thực thi task)

---

**Kết thúc Hướng dẫn**